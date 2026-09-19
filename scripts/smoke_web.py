#!/usr/bin/env python3
"""Run a dependency-free Chromium smoke test against the static Dioxus build.

The script intentionally uses only Python's standard library and Chromium's
CDP endpoint. It serves unknown paths from index.html so the test exercises the
same SPA fallback required by a static deployment, without deploying anything.
"""

from __future__ import annotations

import base64
import http.server
import json
import os
import secrets
import shutil
import socket
import socketserver
import subprocess
import tempfile
import threading
import time
from pathlib import Path
from urllib.parse import urlsplit
from urllib.request import urlopen


ROOT = Path(__file__).resolve().parents[1]
PUBLIC = ROOT / "target/dx/memorylineage-inspector/release/web/public"
CDP_SOCKET_TIMEOUT_SECONDS = 8
BOOT_TIMEOUT_SECONDS = 45
INTERACTION_TIMEOUT_SECONDS = 25
DESKTOP_VIEWPORT = (1440, 1000)
MOBILE_VIEWPORT = (390, 844)

ROUTES = {
    "/": "Verify the history",
    "/inspect": "What is canonical right now?",
    "/history": "How did the canonical history get here?",
    "/history/3": "What exactly happened in this transition?",
    "/lab": "Will an old backup pass as the next state?",
    "/verify": "Can I verify this without trusting the website?",
    "/evidence": "Where is the proof behind the claims?",
    "/architecture": "How does MemoryLineage work?",
    "/security": "What does this system actually guarantee?",
    "/reproduce": "Can another developer reproduce these claims?",
    "/prior-work": "What existed before the hackathon",
}


class SpaHandler(http.server.SimpleHTTPRequestHandler):
    server_version = "MemoryLineageSmoke/1.0"

    def __init__(self, *args, directory: str, **kwargs):
        super().__init__(*args, directory=directory, **kwargs)

    def translate_path(self, path: str) -> str:
        requested = Path(super().translate_path(urlsplit(path).path))
        public = Path(self.directory).resolve()
        try:
            requested.resolve().relative_to(public)
        except ValueError:
            return str(public / "index.html")
        return str(requested if requested.is_file() else public / "index.html")

    def log_message(self, _format: str, *_args) -> None:
        return


class CdpSocket:
    def __init__(self, websocket_url: str):
        parsed = urlsplit(websocket_url)
        self.sock = socket.create_connection((parsed.hostname, parsed.port), timeout=10)
        # A cold Dioxus/WASM page can briefly occupy the renderer while the
        # runtime mounts. Keep individual CDP reads short so the bounded
        # polling loops can retry instead of treating that cold start as a
        # failed route.
        self.sock.settimeout(CDP_SOCKET_TIMEOUT_SECONDS)
        key = base64.b64encode(secrets.token_bytes(16)).decode("ascii")
        path = parsed.path or "/"
        if parsed.query:
            path += f"?{parsed.query}"
        request = (
            f"GET {path} HTTP/1.1\r\n"
            f"Host: {parsed.hostname}:{parsed.port}\r\n"
            "Upgrade: websocket\r\n"
            "Connection: Upgrade\r\n"
            f"Sec-WebSocket-Key: {key}\r\n"
            "Sec-WebSocket-Version: 13\r\n\r\n"
        ).encode("ascii")
        self.sock.sendall(request)
        response = self._read_until(b"\r\n\r\n")
        if b" 101 " not in response:
            raise RuntimeError("CDP WebSocket handshake failed")
        self.next_id = 1
        self.events: list[dict] = []

    def _read_until(self, marker: bytes) -> bytes:
        data = b""
        while marker not in data:
            chunk = self.sock.recv(4096)
            if not chunk:
                raise RuntimeError("CDP connection closed during handshake")
            data += chunk
        return data

    def _read_exact(self, size: int) -> bytes:
        data = b""
        while len(data) < size:
            chunk = self.sock.recv(size - len(data))
            if not chunk:
                raise RuntimeError("CDP connection closed")
            data += chunk
        return data

    def _send_frame(self, payload: bytes, opcode: int = 1) -> None:
        mask = secrets.token_bytes(4)
        masked = bytes(value ^ mask[index % 4] for index, value in enumerate(payload))
        length = len(masked)
        if length < 126:
            header = bytes([0x80 | opcode, 0x80 | length])
        elif length < 65536:
            header = bytes([0x80 | opcode, 0x80 | 126]) + length.to_bytes(2, "big")
        else:
            header = bytes([0x80 | opcode, 0x80 | 127]) + length.to_bytes(8, "big")
        self.sock.sendall(header + mask + masked)

    def _receive_frame(self) -> tuple[int, bytes]:
        first, second = self._read_exact(2)
        opcode = first & 0x0F
        length = second & 0x7F
        if length == 126:
            length = int.from_bytes(self._read_exact(2), "big")
        elif length == 127:
            length = int.from_bytes(self._read_exact(8), "big")
        masked = bool(second & 0x80)
        mask = self._read_exact(4) if masked else b""
        payload = self._read_exact(length)
        if masked:
            payload = bytes(value ^ mask[index % 4] for index, value in enumerate(payload))
        return opcode, payload

    def command(self, method: str, params: dict | None = None) -> dict:
        command_id = self.next_id
        self.next_id += 1
        self._send_frame(json.dumps({"id": command_id, "method": method, "params": params or {}}).encode())
        while True:
            opcode, payload = self._receive_frame()
            if opcode == 9:
                self._send_frame(payload, opcode=10)
                continue
            if opcode != 1:
                continue
            message = json.loads(payload.decode("utf-8"))
            if message.get("method") and os.environ.get("MEMORYLINEAGE_CDP_DEBUG"):
                print(f"CDP event {message['method']}", flush=True)
            if message.get("method"):
                self.events.append(message)
            if message.get("id") != command_id:
                continue
            if "error" in message:
                raise RuntimeError(f"CDP {method} failed")
            return message.get("result", {})

    def evaluate(self, expression: str) -> object:
        result = self.command(
            "Runtime.evaluate",
            {"expression": expression, "returnByValue": True, "awaitPromise": True},
        )
        return result.get("result", {}).get("value")

    def close(self) -> None:
        try:
            self._send_frame(b"", opcode=8)
        except OSError:
            pass
        self.sock.close()


def find_chromium() -> str:
    for name in ("chromium", "chromium-browser", "google-chrome"):
        path = shutil.which(name)
        if path:
            return path
    raise RuntimeError("Chromium is required for smoke-web")


def wait_for_url(base: str, path: str, expected: str, cdp: CdpSocket) -> None:
    cdp.command("Page.navigate", {"url": f"{base}{path}"})
    # Dioxus/WASM may still be compiling and mounting immediately after the
    # navigation response. Give the first runtime turn a bounded head start so
    # the CDP probe does not race the WASM bootstrap itself.
    time.sleep(1.5)
    deadline = time.time() + BOOT_TIMEOUT_SECONDS
    last_body = ""
    while time.time() < deadline:
        try:
            body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
        except (TimeoutError, socket.timeout):
            # The browser may still be compiling/mounting the WASM runtime.
            # Retry until the route-level deadline rather than failing on one
            # slow renderer turn.
            time.sleep(1.0)
            continue
        last_body = body
        if expected in body:
            print(f"PASS route {path or '/'}")
            return
        time.sleep(0.25)
    preview = " ".join(last_body.split())[:360]
    runtime_errors = [
        (
            event.get("params", {}).get("exceptionDetails", {}).get("exception", {}).get("description")
            or event.get("params", {}).get("exceptionDetails", {}).get("text", "")
        )
        for event in cdp.events
        if event.get("method") == "Runtime.exceptionThrown"
    ]
    console_errors = [
        " ".join(
            str(argument.get("value") or argument.get("description") or "")
            for argument in event.get("params", {}).get("args", [])
        ).strip()
        for event in cdp.events
        if event.get("method") == "Runtime.consoleAPICalled"
        and event.get("params", {}).get("type") == "error"
    ]
    network_failures = [
        f"{event.get('params', {}).get('request', {}).get('url', '')}: "
        f"{event.get('params', {}).get('errorText', '')}"
        for event in cdp.events
        if event.get("method") == "Network.loadingFailed"
    ]
    frame_urls = [
        event.get("params", {}).get("frame", {}).get("url", "")
        for event in cdp.events
        if event.get("method") == "Page.frameNavigated"
    ]
    network_responses = [
        (
            event.get("params", {}).get("type", ""),
            event.get("params", {}).get("response", {}).get("status", ""),
            event.get("params", {}).get("response", {}).get("url", ""),
        )
        for event in cdp.events
        if event.get("method") == "Network.responseReceived"
    ]
    error_summary = "; ".join((runtime_errors + console_errors + network_failures)[-3:])
    try:
        page_state = cdp.evaluate("""(() => ({
          href: location.href,
          readyState: document.readyState,
          title: document.title,
          mainHtml: document.querySelector('#main')?.innerHTML?.slice(0, 320) || ''
        }))()""")
    except Exception as error:
        page_state = f"evaluation failed: {type(error).__name__}"
    try:
        frame_tree = cdp.command("Page.getFrameTree").get("frameTree", {}).get("frame", {}).get("url")
    except Exception as error:
        frame_tree = f"unavailable: {type(error).__name__}"
    raise RuntimeError(
        f"route {path or '/'} did not render expected marker {expected!r}; "
        f"page text starts with {preview!r}; "
        f"runtime errors: {error_summary or 'none captured'}; "
        f"navigated frames: {frame_urls[-3:]!r}; "
        f"responses: {network_responses[-8:]!r}; "
        f"frame tree URL: {frame_tree!r}; "
        f"page state: {page_state!r}"
    )


def navigate_via_internal_link(
    cdp: CdpSocket, path: str, expected: str
) -> None:
    main_frame_navigations = sum(
        1
        for event in cdp.events
        if event.get("method") == "Page.frameNavigated"
        and not event.get("params", {}).get("frame", {}).get("parentId")
    )
    expression = f"""(() => {{
      const target = {json.dumps(path)};
      const anchor = Array.from(document.querySelectorAll('a[href]')).find(
        element => new URL(element.href).pathname === target
      );
      if (!anchor) return false;
      const rect = anchor.getBoundingClientRect();
      return {{ x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 }};
    }})()"""
    point = cdp.evaluate(expression)
    if not isinstance(point, dict):
        raise RuntimeError(f"internal link not found for route {path}")
    cdp.command(
        "Input.dispatchMouseEvent",
        {
            "type": "mousePressed",
            "x": point["x"],
            "y": point["y"],
            "button": "left",
            "clickCount": 1,
        },
    )
    cdp.command(
        "Input.dispatchMouseEvent",
        {
            "type": "mouseReleased",
            "x": point["x"],
            "y": point["y"],
            "button": "left",
            "clickCount": 1,
        },
    )

    deadline = time.time() + INTERACTION_TIMEOUT_SECONDS
    last_state: dict = {}
    while time.time() < deadline:
        try:
            state = cdp.evaluate("""(() => ({
              path: location.pathname,
              body: document.body ? document.body.innerText : '',
              page: document.querySelector('.page')?.className || '',
              heading: document.querySelector('.page h1')?.innerText || ''
            }))()""") or {}
        except (TimeoutError, socket.timeout):
            navigations = sum(
                1
                for event in cdp.events
                if event.get("method") == "Page.frameNavigated"
                and not event.get("params", {}).get("frame", {}).get("parentId")
            )
            if navigations > main_frame_navigations:
                raise RuntimeError(
                    f"internal link to {path} performed a full document reload; "
                    "the Inspector requires client-side route navigation"
                )
            time.sleep(0.25)
            continue
        last_state = state

        navigations = sum(
            1
            for event in cdp.events
            if event.get("method") == "Page.frameNavigated"
            and not event.get("params", {}).get("frame", {}).get("parentId")
        )
        if navigations > main_frame_navigations:
            raise RuntimeError(
                f"internal link to {path} performed a full document reload; "
                "the Inspector requires client-side route navigation"
            )
        if state.get("path") == path and expected in state.get("body", ""):
            print(f"PASS client navigation {path}")
            return
        time.sleep(0.25)

    navigated_frames = [
        event.get("params", {}).get("frame", {}).get("url", "")
        for event in cdp.events
        if event.get("method") == "Page.frameNavigated"
    ]
    runtime_errors = [
        event.get("params", {}).get("exceptionDetails", {}).get("text", "")
        for event in cdp.events
        if event.get("method") == "Runtime.exceptionThrown"
    ]
    raise RuntimeError(
        f"internal link did not reach {path} with expected marker {expected!r}; "
        f"last route={last_state.get('path')!r}; "
        f"body={ ' '.join(last_state.get('body', '').split())[:220]!r}; "
        f"page={last_state.get('page')!r}; heading={last_state.get('heading')!r}; "
        f"navigated frames={navigated_frames[-3:]!r}; "
        f"runtime errors={runtime_errors[-3:]!r}"
    )


def verify_home_problem_story(cdp: CdpSocket) -> None:
    manifest = json.loads(
        (ROOT / "fixtures/silent-rollback-v2/manifest.json").read_text()
    )
    evidence = json.loads(
        (ROOT / "evidence/local/demo_space_v2_evidence.json").read_text()
    )
    restored_sequence = manifest["attack"]["restoredSequence"]
    attempted_sequence = manifest["attack"]["attemptedSequence"]
    head_sequence = evidence["head"]["sequence"]
    expected_reason = manifest["attack"]["expectedContractReason"]
    expected = [
        "old memory backup",
        f"registry remains at state {head_sequence}",
        f"state {restored_sequence}",
        f"Transition {attempted_sequence}",
        "stale predecessor root",
        expected_reason,
        "without publishing the private memory itself",
        "existing Sepolia deployment is a separate observation",
        "Load an older private snapshot locally.",
        "Check it against the committed head.",
        "See the exact Solidity reason.",
        "A judge can understand the failure before reading the cryptography",
        *[snapshot["visibleLabel"] for snapshot in manifest["snapshots"]],
    ]
    deadline = time.time() + INTERACTION_TIMEOUT_SECONDS
    body = ""
    missing = expected
    while time.time() < deadline:
        try:
            body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
        except (TimeoutError, socket.timeout):
            time.sleep(0.5)
            continue
        body = " ".join(body.split())
        missing = [marker for marker in expected if marker not in body]
        if not missing:
            print("PASS home problem story / old backup + canonical head + fixture labels")
            return
        time.sleep(0.25)
    note_nodes = cdp.evaluate(
        "Array.from(document.querySelectorAll('.home-source-note')).map(node => "
        "({text: node.textContent, visibleText: node.innerText}))"
    )
    raise RuntimeError(
        f"home problem story is incomplete; missing source-backed context: {missing!r}; "
        f"visible body: {' '.join(body.split())[:700]!r}; note nodes: {note_nodes!r}"
    )


def click_and_wait(cdp: CdpSocket, text: str, expected: str) -> None:
    expression = f"""(() => {{
      const button = Array.from(document.querySelectorAll('button')).find(
        element => element.textContent && element.textContent.includes({json.dumps(text)})
      );
      if (!button) return false;
      button.click();
      return true;
    }})()"""
    if cdp.evaluate(expression) is not True:
        raise RuntimeError(f"button not found: {text}")
    deadline = time.time() + INTERACTION_TIMEOUT_SECONDS
    while time.time() < deadline:
        try:
            body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
        except (TimeoutError, socket.timeout):
            time.sleep(1.0)
            continue
        if expected in body:
            print(f"PASS interaction {text} -> {expected}")
            return
        time.sleep(0.25)
    raise RuntimeError(f"interaction {text} did not produce {expected}")


def click_and_wait_for_rollback(cdp: CdpSocket) -> None:
    expression = """(() => {
      const button = Array.from(document.querySelectorAll('button')).find(
        element => element.textContent && element.textContent.includes('Run Silent Rollback')
      );
      if (!button) return false;
      button.click();
      return true;
    })()"""
    if cdp.evaluate(expression) is not True:
        raise RuntimeError("button not found: Run Silent Rollback")

    deadline = time.time() + INTERACTION_TIMEOUT_SECONDS
    terminal_states = {"LOCAL EVIDENCE: REJECTED"}
    while time.time() < deadline:
        try:
            result = cdp.evaluate("""(() => {
              const heading = document.querySelector('.result-panel .result-heading strong');
              const detail = document.querySelector('.result-panel .result-heading small');
              return { heading: heading?.textContent?.trim() || '', detail: detail?.textContent || '' };
            })()""") or {}
        except (TimeoutError, socket.timeout):
            time.sleep(1.0)
            continue
        heading = result.get("heading", "") if isinstance(result, dict) else ""
        detail = result.get("detail", "") if isinstance(result, dict) else ""
        if heading in terminal_states:
            if "BAD_PREVIOUS_STATE" not in detail:
                raise RuntimeError("rollback result omitted the exact BAD_PREVIOUS_STATE reason")
            print(f"PASS interaction Run Silent Rollback -> {heading} / BAD_PREVIOUS_STATE")
            return
        if heading == "UNEXPECTED LIVE RESULT":
            raise RuntimeError(f"rollback returned an unexpected live result: {detail}")
        time.sleep(0.25)
    raise RuntimeError("Silent Rollback did not reach an actual rejection result")


def verify_history_ledger(cdp: CdpSocket) -> None:
    result = cdp.evaluate("""(() => {
      const ledger = document.querySelector('.history-event-table');
      return {
        rows: ledger?.querySelectorAll('tbody tr').length || 0,
        text: ledger?.innerText || ''
      };
    })()""") or {}
    text = result.get("text", "") if isinstance(result, dict) else ""
    rows = result.get("rows", 0) if isinstance(result, dict) else 0
    required = (
        "Transition committed",
        "Rollback attempt",
        "BAD_PREVIOUS_STATE",
        "Rust/revm against published Solidity bytecode",
    )
    if rows != 4 or any(marker not in text for marker in required):
        raise RuntimeError("History ledger did not render the four evidence-backed incident events")
    print("PASS history ledger / 3 commits + stale-root rejection / evidence sources")


def verify_transition_detail(cdp: CdpSocket) -> None:
    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    required = ("AUTHORIZATION PROOF", "configNonce", "domainSeparator", "EOA_SIGNATURES_VERIFIED")
    missing = [marker for marker in required if marker not in body]
    if missing:
        raise RuntimeError(f"transition detail did not expose the signed authority proof: {missing!r}")
    print("PASS transition detail / active authority proof and EIP-712 fields")


def verify_reproduce_path(cdp: CdpSocket) -> None:
    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    required = (
        "cargo xtask reproduce",
        "JUDGE IN 90 SECONDS",
        "One incident, four checks",
        "EXTERNAL REPRODUCTION / HUMAN REPORT",
        "NOT YET DEMONSTRATED",
    )
    missing = [marker for marker in required if marker not in body]
    if missing:
        raise RuntimeError(f"reproduce path is incomplete; missing={missing!r}")
    print("PASS reproduce path / automated gate, judge flow, and human-report boundary")


def verify_restore_preflight(cdp: CdpSocket) -> None:
    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    if "MATCHES DEMO EVIDENCE HEAD" not in body:
        raise RuntimeError("Restore Preflight did not classify the current fixture head")
    click_and_wait(cdp, "STATE 1", "KNOWN HISTORICAL CHECKPOINT")
    click_and_wait(cdp, "Tamper candidate", "UNKNOWN / DIVERGED")
    click_and_wait(cdp, "Restore original", "KNOWN HISTORICAL CHECKPOINT")
    click_and_wait(cdp, "STATE 3", "MATCHES DEMO EVIDENCE HEAD")
    print("PASS Restore Preflight / current, historical, tampered, and restored candidate")


def verify_recovery_receipt(cdp: CdpSocket) -> None:
    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    required = (
        "RECOVERY DECISION RECEIPT",
        "RECEIPT VERIFIED",
        "CURRENT_HEAD",
        "RESUME_ALLOWED",
        "STRICT-CURRENT-HEAD-ONLY-V1",
        "EOA_SIGNATURES_VERIFIED",
        "TIMELINE_BOUND",
        "PUBLISHED RECOVERY RECEIPT",
    )
    missing = [marker for marker in required if marker not in body]
    if missing:
        raise RuntimeError(
            "published recovery receipt is not visible as a verified browser decision; "
            f"missing={missing!r}; body={' '.join(body.split())[-1200:]!r}"
        )
    click_and_wait(cdp, "Tamper decision", "RECOVERY_DECISION_MISMATCH")
    expression = """(() => {
      const button = Array.from(document.querySelectorAll('.recovery-receipt-actions button')).find(
        element => element.textContent && element.textContent.trim() === 'Restore'
      );
      if (!button) return false;
      button.click();
      return true;
    })()"""
    if cdp.evaluate(expression) is not True:
        raise RuntimeError("recovery receipt restore button not found")
    deadline = time.time() + INTERACTION_TIMEOUT_SECONDS
    while time.time() < deadline:
        body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
        if "RECEIPT VERIFIED" in body and "PUBLISHED RECOVERY RECEIPT" in body:
            print("PASS Recovery Decision Receipt / verify + tamper + restore")
            return
        time.sleep(0.25)
    raise RuntimeError("recovery receipt did not return to VERIFIED after restore")


def capture_requested_screenshots(base: str, cdp: CdpSocket) -> None:
    destination_text = os.environ.get("MEMORYLINEAGE_SCREENSHOT_DIR")
    if not destination_text:
        return
    destination = Path(destination_text)
    desktop = destination / "desktop"
    desktop.mkdir(parents=True, exist_ok=True)
    cdp.command(
        "Emulation.setDeviceMetricsOverride",
        {
            "width": DESKTOP_VIEWPORT[0],
            "height": DESKTOP_VIEWPORT[1],
            "deviceScaleFactor": 1,
            "mobile": False,
        },
    )
    for path, expected in ROUTES.items():
        wait_for_url(base, path, expected, cdp)
        if path == "/lab":
            click_and_wait_for_rollback(cdp)
        payload = cdp.command(
            "Page.captureScreenshot",
            {"format": "png", "captureBeyondViewport": True},
        )
        image = base64.b64decode(payload["data"])
        filename = (path.strip("/").replace("/", "-") or "home") + ".png"
        (desktop / filename).write_bytes(image)
        print(f"PASS desktop screenshot {desktop / filename}")
        if path == "/verify":
            click_and_wait(cdp, "Tamper one field", "TRANSITION_ID_MISMATCH")
            tampered = cdp.command(
                "Page.captureScreenshot",
                {"format": "png", "captureBeyondViewport": True},
            )
            (desktop / "verify-tampered.png").write_bytes(
                base64.b64decode(tampered["data"])
            )
            click_and_wait(cdp, "Restore original", "VERIFIED")

def main() -> int:
    if not (PUBLIC / "index.html").is_file():
        raise RuntimeError("static release is missing; run cargo xtask build-web first")
    chromium = find_chromium()
    server = None
    browser = None
    temporary = tempfile.mkdtemp(prefix="memorylineage-smoke-")
    try:
        handler = lambda *args, **kwargs: SpaHandler(*args, directory=str(PUBLIC), **kwargs)
        server = socketserver.ThreadingTCPServer(("127.0.0.1", 0), handler)
        server.daemon_threads = True
        threading.Thread(target=server.serve_forever, daemon=True).start()
        port = server.server_address[1]
        base = f"http://127.0.0.1:{port}"

        debug_port = socket.socket()
        debug_port.bind(("127.0.0.1", 0))
        chosen_port = debug_port.getsockname()[1]
        debug_port.close()
        browser = subprocess.Popen(
            [
                chromium,
                "--headless=new",
                "--disable-gpu",
                "--disable-dev-shm-usage",
                "--no-first-run",
                "--no-default-browser-check",
                "--remote-allow-origins=*",
                f"--remote-debugging-port={chosen_port}",
                f"--user-data-dir={temporary}",
                "about:blank",
            ],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        deadline = time.time() + BOOT_TIMEOUT_SECONDS
        page = None
        while time.time() < deadline:
            try:
                with urlopen(f"http://127.0.0.1:{chosen_port}/json/list", timeout=1) as response:
                    pages = json.load(response)
                page = next(
                    (
                        candidate
                        for candidate in pages
                        if candidate.get("type") == "page"
                        and candidate.get("webSocketDebuggerUrl")
                    ),
                    None,
                )
                if page:
                    break
            except Exception:
                time.sleep(0.25)
        if not page:
            raise RuntimeError("Chromium page debugging target did not start")
        cdp = CdpSocket(page["webSocketDebuggerUrl"])
        cdp.command("Page.enable")
        cdp.command("Runtime.enable")
        cdp.command("Network.enable")
        cdp.command(
            "Emulation.setDeviceMetricsOverride",
            {
                "width": DESKTOP_VIEWPORT[0],
                "height": DESKTOP_VIEWPORT[1],
                "deviceScaleFactor": 1,
                "mobile": False,
            },
        )
        wait_for_url(base, "/", ROUTES["/"], cdp)
        verify_home_problem_story(cdp)
        navigate_via_internal_link(cdp, "/inspect", ROUTES["/inspect"])
        verify_restore_preflight(cdp)
        for path, expected in ROUTES.items():
            wait_for_url(base, path, expected, cdp)
            if path == "/history":
                verify_history_ledger(cdp)
            if path == "/history/3":
                verify_transition_detail(cdp)
            if path == "/reproduce":
                verify_reproduce_path(cdp)

        wait_for_url(base, "/lab/silent-rollback", "Run Silent Rollback", cdp)
        click_and_wait_for_rollback(cdp)
        wait_for_url(base, "/verify", "VERIFIED", cdp)
        click_and_wait(cdp, "Tamper one field", "TRANSITION_ID_MISMATCH")
        click_and_wait(cdp, "Restore original", "VERIFIED")
        verify_recovery_receipt(cdp)
        screenshot_root = os.environ.get("MEMORYLINEAGE_SCREENSHOT_DIR")
        mobile_directory = Path(screenshot_root) / "mobile" if screenshot_root else None
        if mobile_directory:
            mobile_directory.mkdir(parents=True, exist_ok=True)
        cdp.command(
            "Emulation.setDeviceMetricsOverride",
            {
                "width": MOBILE_VIEWPORT[0],
                "height": MOBILE_VIEWPORT[1],
                "deviceScaleFactor": 1,
                "mobile": True,
            },
        )
        for path, expected in ROUTES.items():
            wait_for_url(base, path, expected, cdp)
            if cdp.evaluate(
                "document.documentElement.scrollWidth <= window.innerWidth"
            ) is not True:
                raise RuntimeError(
                    f"390px viewport has page-level horizontal overflow on {path}"
                )
            if mobile_directory:
                payload = cdp.command(
                    "Page.captureScreenshot",
                    {"format": "png", "captureBeyondViewport": True},
                )
                image = base64.b64decode(payload["data"])
                filename = (path.strip("/").replace("/", "-") or "home") + ".png"
                (mobile_directory / filename).write_bytes(image)
                print(f"PASS mobile screenshot {mobile_directory / filename}")
            else:
                print(f"PASS responsive 390px / no overflow / {path}")
        cdp.command("Emulation.clearDeviceMetricsOverride")
        capture_requested_screenshots(base, cdp)
        cdp.close()
        print("PASS: static browser smoke")
        return 0
    finally:
        if browser is not None:
            browser.terminate()
            try:
                browser.wait(timeout=3)
            except subprocess.TimeoutExpired:
                browser.kill()
        if server is not None:
            server.shutdown()
            server.server_close()
        shutil.rmtree(temporary, ignore_errors=True)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as error:
        print(f"FAIL: {error}")
        raise SystemExit(1)
