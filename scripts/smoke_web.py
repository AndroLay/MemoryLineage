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
        self.sock.settimeout(10)
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
    deadline = time.time() + 12
    while time.time() < deadline:
        body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
        if expected in body:
            print(f"PASS route {path or '/'}")
            return
        time.sleep(0.25)
    raise RuntimeError(f"route {path or '/'} did not render expected marker")


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
    deadline = time.time() + 12
    while time.time() < deadline:
        body = cdp.evaluate("document.body ? document.body.innerText : ''") or ""
        if expected in body:
            print(f"PASS interaction {text} -> {expected}")
            return
        time.sleep(0.25)
    raise RuntimeError(f"interaction {text} did not produce {expected}")


def capture_requested_screenshots(base: str, cdp: CdpSocket) -> None:
    destination_text = os.environ.get("MEMORYLINEAGE_SCREENSHOT_DIR")
    if not destination_text:
        return
    destination = Path(destination_text)
    destination.mkdir(parents=True, exist_ok=True)
    cdp.command("Emulation.clearDeviceMetricsOverride")
    for path, filename, expected in (
        ("/", "home.png", "Verify the history"),
        ("/inspect", "inspect.png", "What is canonical right now?"),
        ("/lab/silent-rollback", "silent-rollback.png", "Run Silent Rollback"),
        ("/verify", "verify.png", "Can I verify this without trusting the website?"),
    ):
        wait_for_url(base, path, expected, cdp)
        payload = cdp.command(
            "Page.captureScreenshot",
            {"format": "png", "captureBeyondViewport": True},
        )
        image = base64.b64decode(payload["data"])
        (destination / filename).write_bytes(image)
        print(f"PASS screenshot {destination / filename}")


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
        deadline = time.time() + 12
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
        routes = {
            "/": "Verify the history",
            "/inspect": "What is canonical right now?",
            "/history": "How did the canonical history get here?",
            "/history/3": "What exactly happened in this transition?",
            "/lab": "Can I break the committed history?",
            "/verify": "Can I verify this without trusting the website?",
            "/evidence": "Where is the proof behind the claims?",
            "/architecture": "How does MemoryLineage work?",
            "/security": "What does this system actually guarantee?",
            "/reproduce": "Can another developer reproduce these claims?",
            "/prior-work": "What existed before the hackathon",
        }
        for path, expected in routes.items():
            wait_for_url(base, path, expected, cdp)

        wait_for_url(base, "/lab/silent-rollback", "Run Silent Rollback", cdp)
        click_and_wait(cdp, "Run Silent Rollback", "BAD_PREVIOUS_STATE")
        wait_for_url(base, "/verify", "VERIFIED", cdp)
        click_and_wait(cdp, "Tamper one field", "TRANSITION_ID_MISMATCH")
        click_and_wait(cdp, "Restore original", "VERIFIED")
        cdp.command(
            "Emulation.setDeviceMetricsOverride",
            {"width": 390, "height": 844, "deviceScaleFactor": 1, "mobile": True},
        )
        wait_for_url(base, "/", "Verify the history", cdp)
        if cdp.evaluate(
            "document.documentElement.scrollWidth <= window.innerWidth"
        ) is not True:
            raise RuntimeError("390px viewport has page-level horizontal overflow")
        print("PASS responsive 390px / no page overflow")
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
