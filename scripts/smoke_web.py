#!/usr/bin/env python3
"""Run a dependency-free Chromium smoke test against the static Dioxus build.

The script intentionally uses only Python's standard library and Chromium's
CDP endpoint. It serves unknown paths from index.html so the test exercises the
same SPA fallback required by a static deployment, without deploying anything.
"""

from __future__ import annotations

import base64
import argparse
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
CDP_SOCKET_TIMEOUT_SECONDS = 20
BOOT_TIMEOUT_SECONDS = 45
INTERACTION_TIMEOUT_SECONDS = 25
# The optional Sepolia probe performs several public-RPC reads and can be
# slower than a local UI interaction. Keep its deadline separate so a slow
# provider does not make the whole release smoke flaky while preserving the
# truthful terminal-state checks below.
SEPOLIA_PROBE_TIMEOUT_SECONDS = 45
SEPOLIA_PROBE_OPT_IN = "MEMORYLINEAGE_SMOKE_SEPOLIA_PROBE"
KEYBOARD_FOCUS_MAX_TABS = 8
DESKTOP_VIEWPORT = (1440, 1000)
MOBILE_VIEWPORT = (390, 844)

ROUTES = {
    "/": "A backup can open and still be out of date.",
    "/app": "Can this backup continue the shared history?",
    "/overview": "Recorded history for Demo Space V2.",
    "/challenge": "Does Backup 1 match the latest shared history?",
    "/inspect": "Inspect Memory Space",
    "/history": "Canonical committed lineage and authority events.",
    "/history/3": "Transition #3",
    "/lab": "Tampering Lab",
    "/lab/not-a-real-case": "Unknown tampering scenario",
    "/verify": "Verify Evidence",
    "/evidence": "Public Evidence",
    "/architecture": "How It Works",
    "/security": "Security & Scope",
    "/reproduce": "Reproduce the Submission",
    "/prior-work": "Standards and project contribution",
}


def normalize_visible_text(value: str) -> str:
    """Collapse layout whitespace before comparing rendered text markers."""
    return " ".join(value.split())


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
        # A slow Dioxus/WASM renderer turn can delay a single CDP response
        # even while Chromium and the page remain alive. Allow that response
        # to complete; route and interaction polling still use their own
        # bounded deadlines.
        self.sock.settimeout(CDP_SOCKET_TIMEOUT_SECONDS)
        self.current_command: str | None = None
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
                command = f" while waiting for {self.current_command}" if self.current_command else ""
                raise RuntimeError(f"CDP connection closed{command}")
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
        self.current_command = method
        command_id = self.next_id
        self.next_id += 1
        self._send_frame(json.dumps({"id": command_id, "method": method, "params": params or {}}).encode())
        while True:
            opcode, payload = self._receive_frame()
            if opcode == 9:
                self._send_frame(payload, opcode=10)
                continue
            if opcode == 8:
                code = int.from_bytes(payload[:2], "big") if len(payload) >= 2 else None
                reason = payload[2:].decode("utf-8", errors="replace") if len(payload) > 2 else ""
                raise RuntimeError(
                    f"CDP peer closed WebSocket (code={code}, reason={reason!r}) "
                    f"during {method}"
                )
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


def build_chromium_command(chromium: str, debug_port: int, profile: Path) -> list[str]:
    return [
        chromium,
        "--headless=new",
        "--disable-gpu",
        "--disable-dev-shm-usage",
        "--no-first-run",
        "--no-default-browser-check",
        "--disable-extensions",
        "--disable-background-networking",
        "--disable-component-update",
        "--disable-sync",
        "--disable-default-apps",
        "--remote-allow-origins=*",
        f"--remote-debugging-port={debug_port}",
        f"--user-data-dir={profile}",
        "about:blank",
    ]


def create_chromium_profile_directory() -> str:
    # Chromium places a Unix-domain singleton socket below this directory; keep
    # the full path short enough for Linux's socket-path limit.
    return tempfile.mkdtemp(prefix="mlsmoke-", dir="/tmp")


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
        if expected in normalize_visible_text(body):
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


def verify_accessibility_and_keyboard(cdp: CdpSocket, path: str) -> None:
    """Check the route's basic semantics and exercise its keyboard order.

    This is intentionally a deterministic smoke check, not a replacement for
    assistive-technology testing. It catches the regressions that are easiest
    to introduce in a WASM route migration: missing headings, unnamed
    controls, missing live status, and a route whose controls cannot receive
    focus through the normal Tab order.
    """
    audit = cdp.evaluate(
        """(() => {
          const visible = element => {
            const style = getComputedStyle(element);
            return element.getClientRects().length > 0
              && style.display !== 'none'
              && style.visibility !== 'hidden'
              && style.visibility !== 'collapse';
          };
          const name = element => {
            const labelledBy = element.getAttribute('aria-labelledby');
            const labelledText = labelledBy
              ? labelledBy.split(/\\s+/).map(id => document.getElementById(id)?.innerText || '').join(' ')
              : '';
            const label = element.labels?.[0]?.innerText || '';
            return (element.getAttribute('aria-label') || labelledText || label
              || element.getAttribute('title') || element.innerText || element.value || '').trim();
          };
          const interactive = Array.from(document.querySelectorAll(
            'a, button, input, select, textarea, [role="button"], [tabindex]'
          )).filter(visible).filter(element => !element.disabled);
          const unnamed = interactive
            .filter(element => !name(element))
            .slice(0, 8)
            .map(element => `${element.tagName.toLowerCase()}${element.className ? '.' + element.className : ''}`);
          const liveRegions = Array.from(document.querySelectorAll('[aria-live]')).filter(visible);
          return {
            headings: document.querySelectorAll('h1').length,
            interactive: interactive.length,
            unnamed,
            focusable: interactive.filter(element => element.tabIndex >= 0).length,
            liveRegions: liveRegions.length,
          };
        })()"""
    ) or {}
    if not isinstance(audit, dict):
        raise RuntimeError(f"accessibility audit returned an invalid result on {path}")
    if audit.get("headings") != 1:
        raise RuntimeError(f"route {path} must expose exactly one visible h1")
    if not audit.get("interactive") or not audit.get("focusable"):
        raise RuntimeError(f"route {path} has no visible keyboard-interactive controls")
    if audit.get("unnamed"):
        raise RuntimeError(f"route {path} has unnamed controls: {audit['unnamed']!r}")
    if path in {"/lab", "/verify", "/challenge"} and not audit.get("liveRegions"):
        raise RuntimeError(f"route {path} has no aria-live result region")

    if not focus_first_control_by_tab(cdp):
        raise RuntimeError(f"route {path} did not expose a focusable control through Tab")
    print(f"PASS accessibility {path or '/'} / h1 + named controls + keyboard Tab focus")


def focus_first_control_by_tab(cdp: CdpSocket) -> bool:
    cdp.evaluate(
        """(() => {
          document.body?.setAttribute('data-memorylineage-focus-start', 'true');
          document.body?.setAttribute('tabindex', '-1');
          document.body?.focus();
        })()"""
    )
    try:
        for _ in range(KEYBOARD_FOCUS_MAX_TABS):
            cdp.command(
                "Input.dispatchKeyEvent",
                {
                    "type": "keyDown",
                    "key": "Tab",
                    "code": "Tab",
                    "windowsVirtualKeyCode": 9,
                    "nativeVirtualKeyCode": 9,
                },
            )
            cdp.command(
                "Input.dispatchKeyEvent",
                {
                    "type": "keyUp",
                    "key": "Tab",
                    "code": "Tab",
                    "windowsVirtualKeyCode": 9,
                    "nativeVirtualKeyCode": 9,
                },
            )
            state = cdp.evaluate(
                """(() => {
                  const element = document.activeElement;
                  const style = element ? getComputedStyle(element) : null;
                  return {
                    visible: Boolean(element && element.getClientRects().length
                      && style?.display !== 'none' && style?.visibility !== 'hidden'),
                    tabIndex: element?.tabIndex ?? -1,
                  };
                })()"""
            ) or {}
            if isinstance(state, dict) and state.get("visible") and state.get("tabIndex", -1) >= 0:
                return True
        return False
    finally:
        try:
            cdp.evaluate(
                "document.body?.removeAttribute('data-memorylineage-focus-start'); "
                "document.body?.removeAttribute('tabindex')"
            )
        except Exception:
            pass


def navigate_via_internal_link(
    cdp: CdpSocket, path: str, expected: str, link_label: str | None = None
) -> None:
    main_frame_navigations = sum(
        1
        for event in cdp.events
        if event.get("method") == "Page.frameNavigated"
        and not event.get("params", {}).get("frame", {}).get("parentId")
    )
    expression = f"""(() => {{
      const target = {json.dumps(path)};
      const label = {json.dumps(link_label)};
      const anchor = Array.from(document.querySelectorAll('a[href]')).find(
        element => new URL(element.href).pathname === target
          && (!label || element.innerText.includes(label))
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
        if state.get("path") == path and expected in normalize_visible_text(state.get("body", "")):
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


def navigate_via_history(cdp: CdpSocket, path: str, expected: str) -> None:
    expression = f"""(() => {{
      window.history.pushState([0, 0], '', {json.dumps(path)});
      window.dispatchEvent(new PopStateEvent('popstate'));
      return true;
    }})()"""
    if cdp.evaluate(expression) is not True:
        raise RuntimeError(f"could not dispatch a history navigation to {path}")
    deadline = time.time() + INTERACTION_TIMEOUT_SECONDS
    while time.time() < deadline:
        state = cdp.evaluate("""(() => ({
          path: location.pathname,
          body: document.body ? document.body.innerText : ''
        }))()""") or {}
        if (
            isinstance(state, dict)
            and state.get("path") == path
            and expected in normalize_visible_text(state.get("body", ""))
        ):
            return
        time.sleep(0.1)
    raise RuntimeError(f"history navigation did not render {path} with {expected!r}")


def verify_welcome_page(cdp: CdpSocket) -> None:
    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    visible_body = " ".join(body.split())
    primary_navigation = cdp.evaluate("Boolean(document.querySelector('.primary-nav'))")
    landing_count = cdp.evaluate("document.querySelectorAll('.memory-landing').length")
    if primary_navigation:
        raise RuntimeError("Welcome page exposes audit navigation before the visitor enters the app")
    if landing_count != 1:
        raise RuntimeError(f"Welcome route must render exactly one landing page; found {landing_count!r}")
    leaked_results = ("BAD_PREVIOUS_STATE", "LOCAL EVIDENCE: REJECTED", "EXACT MACHINE RESULT")
    if any(marker in visible_body for marker in leaked_results):
        raise RuntimeError("Welcome page reveals the challenge verdict before the visitor starts")
    expected = (
        "A backup can open and still be out of date.",
        "Backup 1",
        "Backup 3",
        "Get started",
        "Explore freely",
        "SYNTHETIC LOCAL EXAMPLE",
        "What this demo can and cannot show",
    )
    missing = [marker for marker in expected if marker.casefold() not in visible_body.casefold()]
    sections = cdp.evaluate(
        "(() => Array.from(document.querySelectorAll('.memory-landing section[id]'))"
        ".map(section => section.id))()"
    )
    required_sections = {
        "problem",
        "how-it-works",
        "evidence-boundary",
        "inside-inspector",
        "questions",
    }
    links = cdp.evaluate(
        "(() => Array.from(document.querySelectorAll('.landing-hero-actions a')).map(link => ({"
        "label: link.innerText, path: new URL(link.href).pathname})))()"
    )
    github = cdp.evaluate(
        "(() => { const link = document.querySelector('.github-button'); return link ? {"
        "path: new URL(link.href).pathname, icon: Boolean(link.querySelector('svg')), "
        "label: link.getAttribute('aria-label')} : null; })()"
    )
    if missing or not isinstance(sections, list) or not required_sections.issubset(sections) \
        or not isinstance(links, list) or len(links) != 2 \
        or not any(
            link.get("path") == "/app"
            and link.get("label", "").strip().casefold().startswith("get started")
            for link in links
        ) \
        or not any(
            link.get("path") == "/overview"
            and link.get("label", "").strip().casefold().startswith("explore freely")
            for link in links
        ) \
        or not isinstance(github, dict) or github.get("icon") is not True:
        raise RuntimeError(
            f"Welcome choices or GitHub source button are incomplete; missing={missing!r}; "
            f"sections={sections!r}; links={links!r}; github={github!r}; "
            f"visible body: {visible_body[:700]!r}"
        )
    print("PASS landing / product story, clear entry paths, no verdict leak, GitHub source button")


def verify_landing_scroll_reveal(cdp: CdpSocket) -> None:
    state = cdp.evaluate("""(() => {
      const root = document.querySelector('.memory-landing');
      const target = root?.querySelector('.landing-contrast');
      if (!root || !target) return null;
      const nativeTimeline = CSS.supports('animation-timeline', 'view()');
      const reducedMotion = matchMedia('(prefers-reduced-motion: reduce)').matches;
      const style = getComputedStyle(target);
      return {
        nativeTimeline,
        reducedMotion,
        animationName: style.animationName,
        animationTimeline: style.animationTimeline || style.getPropertyValue('animation-timeline'),
        fallbackReady: root.dataset.scrollRevealReady === 'true',
        fallbackObserver: Boolean(window.__memoryLineageRevealObserver),
        revealTargets: root.querySelectorAll('.landing-contrast, .landing-flow, .landing-history-preview').length
      };
    })()""") or {}
    if state.get("nativeTimeline") and not state.get("reducedMotion"):
        if state.get("animationName") != "landing-scroll-rise" \
            or "view(" not in str(state.get("animationTimeline", "")):
            raise RuntimeError(f"native landing scroll animation is not attached to its section: {state!r}")
    else:
        if not state.get("fallbackReady") or not state.get("fallbackObserver"):
            raise RuntimeError(f"landing scroll reveal fallback did not install: {state!r}")
        if state.get("revealTargets", 0) < 3:
            raise RuntimeError(f"landing scroll reveal has too few targets: {state!r}")
        cdp.evaluate("document.querySelector('.landing-flow')?.scrollIntoView({block: 'center'})")
        deadline = time.time() + 3
        revealed = False
        while time.time() < deadline:
            revealed = cdp.evaluate(
                "document.querySelector('.landing-flow')?.classList.contains('scroll-revealed')"
            ) is True
            if revealed:
                break
            time.sleep(0.05)
        if not revealed:
            raise RuntimeError("landing scroll reveal did not activate after scrolling to its target")
        cdp.evaluate("window.scrollTo(0, 0)")
    print("PASS landing / scroll reveal has a native timeline or installed observer fallback")


def verify_overview_is_not_landing(cdp: CdpSocket) -> None:
    state = cdp.evaluate("""(() => ({
      path: location.pathname,
      landingCount: document.querySelectorAll('.memory-landing').length,
      heading: document.querySelector('h1')?.innerText || '',
      challenge: (() => {
        const link = document.querySelector('.overview-challenge-cta');
        return link ? {path: new URL(link.href).pathname, label: link.innerText} : null;
      })(),
      body: document.body?.innerText || ''
    }))()""") or {}
    if state.get("path") != "/overview" or state.get("landingCount") != 0 \
        or "recorded history" not in state.get("heading", "").casefold() \
        or not isinstance(state.get("challenge"), dict) \
        or state["challenge"].get("path") != "/challenge" \
        or "one-minute challenge" not in state["challenge"].get("label", "").casefold():
        raise RuntimeError(
            f"Overview must stay distinct from the landing and offer the challenge without the tour: {state!r}"
        )
    print("PASS overview / technical page is distinct from the public landing page")


def verify_home_problem_story(cdp: CdpSocket) -> None:
    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    visible_body = " ".join(body.split())
    leaked_results = ("BAD_PREVIOUS_STATE", "LOCAL EVIDENCE: REJECTED", "EXACT MACHINE RESULT")
    if any(marker in visible_body for marker in leaked_results):
        raise RuntimeError("Home reveals the challenge verdict before the visitor starts")
    expected = (
        "Can this backup continue the shared history?",
        "Backup 1",
        "Backup 3",
        "SYNTHETIC DEMO",
        "No real agent or transaction",
    )
    missing = [marker for marker in expected if marker.casefold() not in visible_body.casefold()]
    primary = cdp.evaluate(
        "(() => { const link = document.querySelector('.home-challenge-cta'); "
        "return link ? {label: link.innerText, path: new URL(link.href).pathname} : null; })()"
    )
    if missing or not isinstance(primary, dict) or primary.get("path") != "/challenge" \
        or "one-minute challenge" not in primary.get("label", "").casefold():
        raise RuntimeError(
            f"Home challenge entry is incomplete; missing={missing!r}; primary={primary!r}; "
            f"visible body: {visible_body[:700]!r}"
        )
    print("PASS app home / concise scenario, no verdict leak, one challenge action")


def verify_guided_tour_step(
    cdp: CdpSocket, step_label: str, expected_spotlight_count: int
) -> None:
    state = cdp.evaluate("""(() => ({
      panel: document.querySelector('.guided-tour-panel')?.innerText || '',
      spotlightCount: document.querySelectorAll('.tour-spotlight').length
    }))()""") or {}
    if step_label not in state.get("panel", "") \
        or state.get("spotlightCount") != expected_spotlight_count:
        raise RuntimeError(f"guided tour step {step_label!r} is incomplete: {state!r}")


def verify_guided_next_state(cdp: CdpSocket, enabled: bool) -> None:
    state = cdp.evaluate("""(() => {
      const panel = document.querySelector('.guided-tour-panel');
      const next = panel && Array.from(panel.querySelectorAll('button')).find(
        button => button.innerText.trim() === 'Next'
      );
      return next ? { disabled: next.disabled } : null;
    })()""")
    if not isinstance(state, dict) or state.get("disabled") is enabled:
        raise RuntimeError(f"guided tour Next enabled={enabled} expected, found {state!r}")


def verify_free_entry(cdp: CdpSocket) -> None:
    state = cdp.evaluate("""(() => ({
      guided: Boolean(document.querySelector('.guided-tour-panel')),
      navigation: Boolean(document.querySelector('.primary-nav'))
    }))()""") or {}
    if state.get("guided") or not state.get("navigation"):
        raise RuntimeError(f"Explore freely should open the app without the tour: {state!r}")


def select_challenge_answer(cdp: CdpSocket, answer: str) -> None:
    label = json.dumps(answer)
    clicked = cdp.evaluate(f"""(() => {{
      const button = Array.from(document.querySelectorAll('button.challenge-answer')).find(
        element => element.textContent && element.textContent.includes({label})
      );
      if (!button) return false;
      button.click();
      return true;
    }})()""")
    if clicked is not True:
        raise RuntimeError(f"challenge answer is missing: {answer!r}")
    deadline = time.time() + INTERACTION_TIMEOUT_SECONDS
    while time.time() < deadline:
        selected = cdp.evaluate(f"""(() => {{
          const button = Array.from(document.querySelectorAll('button.challenge-answer')).find(
            element => element.textContent && element.textContent.includes({label})
          );
          return button ? button.getAttribute('aria-pressed') === 'true' : false;
        }})()""")
        if selected is True:
            return
        time.sleep(0.1)
    raise RuntimeError(f"challenge answer did not expose its selected state: {answer!r}")


def verify_first_run_challenge(cdp: CdpSocket) -> None:
    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    if cdp.evaluate("Boolean(document.querySelector('.primary-nav'))"):
        raise RuntimeError("challenge exposes advanced navigation before the visitor completes it")
    if "BAD_PREVIOUS_STATE" in body or "CHECK PASSED" in body:
        raise RuntimeError("first-run challenge reveals the result before an answer is checked")
    if "Choose one answer, then check it." not in body:
        raise RuntimeError("first-run challenge does not explain the next action")

    disabled = cdp.evaluate("""(() => {
      const button = Array.from(document.querySelectorAll('button')).find(
        element => element.textContent && element.textContent.includes('Check my answer')
      );
      return button ? button.disabled : null;
    })()""")
    if disabled is not True:
        raise RuntimeError("challenge check should stay disabled until the visitor chooses an answer")

    select_challenge_answer(cdp, "It matches the latest history")
    click_and_wait(cdp, "Check my answer", "CHECK PASSED")

    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    required = (
        "Not quite",
        "CHECK PASSED",
        "RESTORE DECISION",
        "Backup 1 is older than the shared history at Backup 3",
    )
    missing = [marker for marker in required if marker.casefold() not in body.casefold()]
    if missing:
        raise RuntimeError(f"challenge result does not explain the verified outcome; missing={missing!r}")

    details = cdp.evaluate("""(() => {
      const disclosure = document.querySelector('details.challenge-technical');
      return {
        exists: Boolean(disclosure),
        closed: Boolean(disclosure && !disclosure.open),
        hasMachineReason: Boolean(disclosure && disclosure.textContent.includes('BAD_PREVIOUS_STATE')),
        reasonVisible: document.body?.innerText.includes('BAD_PREVIOUS_STATE') || false,
        hasLiveStatus: Boolean(document.querySelector('.challenge-result[aria-live="polite"]')),
      };
    })()""") or {}
    if not isinstance(details, dict) or not all(
        details.get(key) is True for key in ("exists", "closed", "hasMachineReason", "hasLiveStatus")
    ) or details.get("reasonVisible"):
        raise RuntimeError(f"technical reason is not progressively disclosed: {details!r}")

    opened = cdp.evaluate("""(() => {
      const disclosure = document.querySelector('details.challenge-technical');
      disclosure?.querySelector('summary')?.click();
      return disclosure?.open === true;
    })()""")
    if opened is not True or "BAD_PREVIOUS_STATE" not in (
        cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    ):
        raise RuntimeError("technical disclosure did not reveal BAD_PREVIOUS_STATE on request")

    click_and_wait(cdp, "Try another answer", "Choose one answer, then check it.")
    select_challenge_answer(cdp, "It is an older checkpoint")
    click_and_wait(cdp, "Check my answer", "Good catch")
    capture_demo_frame(cdp, "02-challenge.png", focus=".challenge-result")
    print("PASS first-run challenge / predict, verify, separate check from decision, disclose exact reason")


def verify_submission_context(cdp: CdpSocket, path: str) -> None:
    manifest = json.loads((ROOT / "evidence/submission/manifest.json").read_text())
    body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
    if path != "/evidence" and manifest["incidentId"].casefold() not in body.casefold():
        raise RuntimeError(f"{path} does not show the named submission incident; body starts with {' '.join(body.split())[:430]!r}")
    if path == "/verify":
        bundle_hash = next(
            item["sha256"] for item in manifest["artifacts"]
            if item["path"] == "evidence/local/demo_space_v2_evidence.json"
        )
        details = cdp.evaluate("""(() => ({
          hashShown: Array.from(document.querySelectorAll('[title]')).some(
            element => element.title === """ + json.dumps(bundle_hash) + """),
          hasCommand: document.body.innerText.includes(
            'submission verify evidence/submission/manifest.json'),
          hasSource: document.body.innerText.includes('DEMO_SPACE_V2_LOCAL'),
          hasReplayScope: document.body.innerText.includes('OFFLINE_BUNDLE_REPLAY'),
          hasCount: document.body.innerText.toLowerCase().includes('artifact count')
        }))()""") or {}
        if not isinstance(details, dict) or not all(details.values()):
            raise RuntimeError(f"Verify page lacks published package context: {details!r}")
    if path in {"/inspect", "/history"}:
        controls = cdp.evaluate(
            "Array.from(document.querySelectorAll('.inspect-view-controls, .history-view-controls'))"
            ".flatMap(node => Array.from(node.querySelectorAll('button, .view-control')))"
            ".map(node => node.innerText.trim())"
        ) or []
        if [control.casefold() for control in controls] != ["linear"]:
            raise RuntimeError(f"{path} exposes unsupported lineage controls: {controls!r}")
    if path == "/evidence":
        if not all(
            text.casefold() in body.casefold()
            for text in (
                "Missing authorization proof",
                "BLOCK_UNVERIFIED",
                "current head held before loader",
            )
        ):
            raise RuntimeError(
                "Evidence page does not show the missing-proof runtime hold"
            )
    if path == "/inspect":
        primary = cdp.evaluate(
            "(() => { const link = document.querySelector('.inspect-query a.button-primary'); "
            "return link ? new URL(link.href).pathname : null; })()"
        )
        if primary != "/lab/silent-rollback":
            raise RuntimeError(f"Inspect primary action does not open the restore check: {primary!r}")
    print(f"PASS submission context {path}")


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
        if expected in normalize_visible_text(body):
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
    terminal_states = {"CHECK PASSED", "LIVE RPC: REJECTED"}
    while time.time() < deadline:
        try:
            result = cdp.evaluate("""(() => {
              const heading = document.querySelector('.result-panel .result-heading strong');
              const detail = document.querySelector('.result-panel .result-heading small');
              const panel = document.querySelector('.result-panel');
              const symbol = document.querySelector('.result-panel .result-symbol');
              return {
                heading: heading?.textContent?.trim() || '',
                detail: detail?.textContent || '',
                panelClass: panel?.className || '',
                symbolClass: symbol?.className || '',
              };
            })()""") or {}
        except (TimeoutError, socket.timeout):
            time.sleep(1.0)
            continue
        heading = result.get("heading", "") if isinstance(result, dict) else ""
        detail = result.get("detail", "") if isinstance(result, dict) else ""
        if heading in terminal_states:
            if "BAD_PREVIOUS_STATE" not in detail:
                raise RuntimeError("rollback result omitted the exact BAD_PREVIOUS_STATE reason")
            body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
            expected = (
                "The evidence check passed.",
                "Backup 1 cannot continue the history at Snapshot 3.",
                "HOLD OLD BACKUP",
            )
            missing = [marker for marker in expected if marker.casefold() not in body.casefold()]
            if heading == "CHECK PASSED" and (
                missing or "LOCAL EVIDENCE: REJECTED" in body
            ):
                raise RuntimeError(
                    f"local rollback conflates evidence verification and restore decision; missing={missing!r}"
                )
            if heading == "CHECK PASSED" and (
                "result-panel-verified" not in result.get("panelClass", "")
                or "result-symbol-verified" not in result.get("symbolClass", "")
            ):
                raise RuntimeError("passed evidence result does not use a consistent verified status treatment")
            print(f"PASS interaction Run Silent Rollback -> {heading} / BAD_PREVIOUS_STATE")
            return
        if heading == "UNEXPECTED LIVE RESULT":
            raise RuntimeError(f"rollback returned an unexpected live result: {detail}")
        time.sleep(0.25)
    raise RuntimeError("Silent Rollback did not reach an actual rejection result")


def click_and_wait_for_sepolia_probe(cdp: CdpSocket) -> None:
    """Exercise the optional browser RPC path without hiding its source.

    A browser may be unable to use the public RPC because of provider CORS or
    network policy. That is a valid unavailable outcome; an unavailable probe
    must never be rendered as a live rejection.
    """
    expression = """(() => {
      const button = Array.from(document.querySelectorAll('button')).find(
        element => element.textContent && element.textContent.includes('Probe separate Sepolia')
      );
      if (!button) return false;
      button.click();
      return true;
    })()"""
    if cdp.evaluate(expression) is not True:
        raise RuntimeError("button not found: Probe separate Sepolia")

    deadline = time.time() + SEPOLIA_PROBE_TIMEOUT_SECONDS
    terminal_states = {"LIVE RPC: REJECTED", "SEPOLIA PROBE: UNAVAILABLE"}
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
            if heading == "LIVE RPC: REJECTED" and "BAD_PREVIOUS_STATE" not in detail:
                raise RuntimeError("live Sepolia probe omitted BAD_PREVIOUS_STATE")
            if heading == "SEPOLIA PROBE: UNAVAILABLE" and "SEPARATE SEPOLIA PROBE" not in detail:
                raise RuntimeError("unavailable Sepolia probe omitted its source boundary")
            print(f"PASS browser Sepolia probe -> {heading}")
            return
        if heading == "UNEXPECTED LIVE RESULT":
            raise RuntimeError(f"browser Sepolia probe returned an unexpected result: {detail}")
        time.sleep(0.25)
    raise RuntimeError("browser Sepolia probe did not reach a truthful terminal state")


def run_optional_sepolia_probe(cdp: CdpSocket) -> None:
    if os.environ.get(SEPOLIA_PROBE_OPT_IN) == "1":
        click_and_wait_for_sepolia_probe(cdp)
    else:
        print(
            "SKIP optional public Sepolia RPC probe "
            f"(set {SEPOLIA_PROBE_OPT_IN}=1 to enable)"
        )


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
        "STRICT-AUTHORIZED-CURRENT-HEAD-V2",
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


def set_file_input(cdp: CdpSocket, selector: str, path: Path) -> None:
    root = cdp.command("DOM.getDocument", {"depth": 0}).get("root", {}).get("nodeId")
    if not root:
        raise RuntimeError("could not locate the document for file import smoke")
    node = cdp.command(
        "DOM.querySelector", {"nodeId": root, "selector": selector}
    ).get("nodeId")
    if not node:
        raise RuntimeError(f"file import control was not found: {selector}")
    cdp.command("DOM.setFileInputFiles", {"nodeId": node, "files": [str(path)]})


def wait_for_body_text(cdp: CdpSocket, expected: str) -> None:
    deadline = time.time() + INTERACTION_TIMEOUT_SECONDS
    while time.time() < deadline:
        body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
        if expected in normalize_visible_text(body):
            return
        time.sleep(0.1)
    raise RuntimeError(f"Inspector did not show the expected import result: {expected!r}")


def verify_import_guards(cdp: CdpSocket) -> None:
    with tempfile.TemporaryDirectory(prefix="memorylineage-import-smoke-") as directory:
        unsupported = Path(directory) / "evidence.txt"
        unsupported.write_text("{}", encoding="utf-8")
        oversized = Path(directory) / "evidence.json"
        with oversized.open("wb") as evidence_file:
            evidence_file.truncate(1024 * 1024 + 1)

        set_file_input(cdp, ".evidence-actions-panel input[type=file]", unsupported)
        wait_for_body_text(cdp, "Choose a file with a .json extension.")
        set_file_input(cdp, ".evidence-actions-panel input[type=file]", oversized)
        wait_for_body_text(cdp, "The file exceeds the 1024 KiB import limit.")
        set_file_input(cdp, ".recovery-receipt-actions input[type=file]", oversized)
        wait_for_body_text(cdp, "The file exceeds the 1024 KiB import limit.")
        restored = cdp.evaluate("""(() => {
          const button = Array.from(document.querySelectorAll('.recovery-receipt-actions button')).find(
            element => element.textContent && element.textContent.trim() === 'Restore'
          );
          if (!button) return false;
          button.click();
          return true;
        })()""")
        if restored is not True:
            raise RuntimeError("recovery receipt restore control was not found after import guard checks")
        wait_for_body_text(cdp, "PUBLISHED RECOVERY RECEIPT")
        wait_for_body_text(cdp, "RECEIPT VERIFIED")
    print("PASS file imports / unsupported extension and oversized files rejected before read")


def capture_requested_screenshots(cdp: CdpSocket) -> None:
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
        navigate_via_history(cdp, path, expected)
        cdp.evaluate("window.scrollTo(0, 0)")
        if path == "/lab":
            click_and_wait_for_rollback(cdp)
        payload = cdp.command(
            "Page.captureScreenshot",
            {"format": "png", "captureBeyondViewport": False},
        )
        image = base64.b64decode(payload["data"])
        filename = (path.strip("/").replace("/", "-") or "home") + ".png"
        (desktop / filename).write_bytes(image)
        print(f"PASS desktop screenshot {desktop / filename}")
        if path == "/verify":
            click_and_wait(cdp, "Tamper one field", "TRANSITION_ID_MISMATCH")
            tampered = cdp.command(
                "Page.captureScreenshot",
                {"format": "png", "captureBeyondViewport": False},
            )
            (desktop / "verify-tampered.png").write_bytes(
                base64.b64decode(tampered["data"])
            )
            click_and_wait(cdp, "Restore original", "VERIFIED")


def capture_demo_frame(cdp: CdpSocket, filename: str, *, focus: str | None = None) -> None:
    destination_text = os.environ.get("MEMORYLINEAGE_DEMO_FRAMES_DIR")
    if not destination_text:
        return
    destination = Path(destination_text)
    destination.mkdir(parents=True, exist_ok=True)
    if focus:
        cdp.evaluate(
            f"document.querySelector({json.dumps(focus)})?.scrollIntoView({{block: 'center'}})"
        )
    else:
        cdp.evaluate("window.scrollTo(0, 0); document.activeElement?.blur?.()")
    time.sleep(0.2)
    payload = cdp.command(
        "Page.captureScreenshot",
        {"format": "png", "captureBeyondViewport": False},
    )
    (destination / filename).write_bytes(base64.b64decode(payload["data"]))


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run the MemoryLineage browser acceptance smoke against a static artifact or an existing server."
    )
    parser.add_argument(
        "--base-url",
        help="check an already-running server instead of starting the static-artifact fallback server",
    )
    args = parser.parse_args()
    if not args.base_url and not (PUBLIC / "index.html").is_file():
        raise RuntimeError("static release is missing; run cargo xtask build-web first")
    chromium = find_chromium()
    server = None
    browser = None
    cdp = None
    chosen_port = None
    temporary = create_chromium_profile_directory()
    try:
        if args.base_url:
            base = args.base_url.rstrip("/")
        else:
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
            build_chromium_command(chromium, chosen_port, Path(temporary)),
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
        verify_welcome_page(cdp)
        capture_demo_frame(cdp, "00-landing.png")
        navigate_via_internal_link(cdp, "/app", ROUTES["/app"])
        verify_home_problem_story(cdp)
        verify_guided_tour_step(cdp, "STEP 1 OF 6", expected_spotlight_count=1)
        capture_demo_frame(cdp, "01-home.png")
        click_and_wait(cdp, "Next", "STEP 2 OF 6")
        verify_guided_tour_step(cdp, "STEP 2 OF 6", expected_spotlight_count=1)
        verify_guided_next_state(cdp, enabled=False)
        verify_first_run_challenge(cdp)
        verify_guided_next_state(cdp, enabled=True)
        navigate_via_internal_link(cdp, "/", ROUTES["/"])
        verify_welcome_page(cdp)
        verify_landing_scroll_reveal(cdp)
        navigate_via_internal_link(cdp, "/overview", ROUTES["/overview"])
        verify_overview_is_not_landing(cdp)
        navigate_via_internal_link(cdp, "/inspect", ROUTES["/inspect"])
        verify_restore_preflight(cdp)
        capture_demo_frame(cdp, "02-inspect.png")
        for path, expected in ROUTES.items():
            if path in {"/lab/not-a-real-case", "/verify"}:
                # Cold-load representative deep links, including the unknown
                # parameter route. Other pages use SPA history navigation so
                # this smoke does not repeatedly restart the WASM application.
                wait_for_url(base, path, expected, cdp)
            else:
                navigate_via_history(cdp, path, expected)
            verify_accessibility_and_keyboard(cdp, path)
            if path == "/lab/not-a-real-case":
                unknown_scenario = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
                if "UNKNOWN SCENARIO" not in unknown_scenario or "No lab scenario is registered" not in unknown_scenario:
                    raise RuntimeError("unknown lab slug did not render the not-found state")
                print("PASS tampering lab / unknown slug renders not-found instead of Silent Rollback")
            if path == "/challenge":
                verify_first_run_challenge(cdp)
            if path in {"/inspect", "/history", "/verify", "/evidence"}:
                verify_submission_context(cdp, path)
            if path == "/history":
                verify_history_ledger(cdp)
            if path == "/history/3":
                verify_transition_detail(cdp)
            if path == "/reproduce":
                verify_reproduce_path(cdp)

        navigate_via_internal_link(cdp, "/lab", "Run Silent Rollback")
        verify_accessibility_and_keyboard(cdp, "/lab")
        navigate_via_internal_link(cdp, "/", ROUTES["/"])
        verify_welcome_page(cdp)
        navigate_via_internal_link(
            cdp,
            "/overview",
            ROUTES["/overview"],
            link_label="Explore freely",
        )
        verify_overview_is_not_landing(cdp)
        verify_free_entry(cdp)
        navigate_via_internal_link(
            cdp, "/challenge", ROUTES["/challenge"], link_label="Try the one-minute challenge"
        )
        if cdp.evaluate("Boolean(document.querySelector('.guided-tour-panel'))") is True:
            raise RuntimeError("the free overview's challenge link unexpectedly starts the guided tour")
        verify_first_run_challenge(cdp)
        navigate_via_internal_link(cdp, "/lab", "Tampering Lab")
        navigate_via_history(cdp, "/lab/silent-rollback", "Run Silent Rollback")
        verify_accessibility_and_keyboard(cdp, "/lab/silent-rollback")
        click_and_wait_for_rollback(cdp)
        capture_demo_frame(cdp, "03-rollback.png")
        run_optional_sepolia_probe(cdp)
        navigate_via_internal_link(cdp, "/verify", "BUNDLE REPLAY VERIFIED")
        verify_import_guards(cdp)
        click_and_wait(cdp, "Tamper one field", "TRANSITION_ID_MISMATCH")
        capture_demo_frame(cdp, "04-tampered.png", focus=".verify-results-detail")
        click_and_wait(cdp, "Restore original", "VERIFIED")
        capture_demo_frame(cdp, "05-restored.png", focus=".verify-results-detail")
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
            navigate_via_history(cdp, path, expected)
            cdp.evaluate("window.scrollTo(0, 0)")
            if cdp.evaluate(
                "document.documentElement.scrollWidth <= window.innerWidth"
            ) is not True:
                raise RuntimeError(
                    f"390px viewport has page-level horizontal overflow on {path}"
                )
            if mobile_directory:
                payload = cdp.command(
                    "Page.captureScreenshot",
                    {"format": "png", "captureBeyondViewport": False},
                )
                image = base64.b64decode(payload["data"])
                filename = (path.strip("/").replace("/", "-") or "home") + ".png"
                (mobile_directory / filename).write_bytes(image)
                print(f"PASS mobile screenshot {mobile_directory / filename}")
            else:
                print(f"PASS responsive 390px / no overflow / {path}")
        cdp.command("Emulation.clearDeviceMetricsOverride")
        capture_requested_screenshots(cdp)
        cdp.close()
        print(
            "PASS: development browser smoke"
            if args.base_url
            else "PASS: static browser smoke"
        )
        return 0
    except Exception:
        if os.environ.get("MEMORYLINEAGE_CDP_DEBUG") == "1":
            browser_state = browser.poll() if browser is not None else None
            targets = []
            if chosen_port is not None:
                try:
                    with urlopen(f"http://127.0.0.1:{chosen_port}/json/list", timeout=1) as response:
                        targets = [
                            {"type": item.get("type"), "url": item.get("url")}
                            for item in json.load(response)
                        ]
                except Exception as diagnostic_error:
                    targets = [f"unavailable: {type(diagnostic_error).__name__}"]
            event_names = [event.get("method") for event in (cdp.events if cdp else [])]
            print(
                f"CDP diagnosis browser_exit={browser_state!r} "
                f"targets={targets!r} last_events={event_names[-8:]!r}",
                flush=True,
            )
        raise
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
