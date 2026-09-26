import contextlib
import io
import os
import unittest
from unittest.mock import patch

import scripts.smoke_web as smoke_web


class SepoliaProbeOptInTests(unittest.TestCase):
    def test_probe_is_skipped_without_explicit_opt_in(self) -> None:
        variable = "MEMORYLINEAGE_SMOKE_SEPOLIA_PROBE"
        with patch.dict(os.environ):
            os.environ.pop(variable, None)
            output = io.StringIO()
            with patch.object(smoke_web, "click_and_wait_for_sepolia_probe") as probe:
                with contextlib.redirect_stdout(output):
                    smoke_web.run_optional_sepolia_probe(object())

        probe.assert_not_called()
        self.assertIn("SKIP optional public Sepolia RPC probe", output.getvalue())

    def test_probe_runs_when_explicitly_enabled(self) -> None:
        variable = "MEMORYLINEAGE_SMOKE_SEPOLIA_PROBE"
        marker = object()
        with patch.dict(os.environ, {variable: "1"}):
            with patch.object(smoke_web, "click_and_wait_for_sepolia_probe") as probe:
                smoke_web.run_optional_sepolia_probe(marker)

        probe.assert_called_once_with(marker)


class KeyboardFocusSmokeTests(unittest.TestCase):
    def test_keyboard_check_stops_after_the_first_visible_focus(self) -> None:
        class CdpStub:
            def __init__(self) -> None:
                self.key_events: list[str] = []

            def evaluate(self, expression: str) -> object:
                if "document.activeElement" in expression:
                    return {"visible": True, "tabIndex": 0}
                return None

            def command(self, method: str, params: dict) -> dict:
                self.key_events.append(params["type"])
                return {}

        cdp = CdpStub()

        self.assertTrue(smoke_web.focus_first_control_by_tab(cdp))

        self.assertEqual(cdp.key_events, ["keyDown", "keyUp"])


class CdpTimeoutTests(unittest.TestCase):
    def test_devtools_command_timeout_allows_slow_local_wasm_turns(self) -> None:
        class HandshakeSocket:
            def __init__(self) -> None:
                self.timeout: float | None = None

            def settimeout(self, timeout: float) -> None:
                self.timeout = timeout

            def sendall(self, _payload: bytes) -> None:
                pass

            def recv(self, _size: int) -> bytes:
                return b"HTTP/1.1 101 Switching Protocols\r\n\r\n"

        fake_socket = HandshakeSocket()
        with patch.object(smoke_web.socket, "create_connection", return_value=fake_socket):
            smoke_web.CdpSocket("ws://127.0.0.1:9222/devtools/page/test")

        self.assertIsNotNone(fake_socket.timeout)
        self.assertGreaterEqual(fake_socket.timeout, 20)


class CdpDisconnectDiagnosticsTests(unittest.TestCase):
    def test_closed_connection_names_the_pending_command(self) -> None:
        class ClosedSocket:
            def recv(self, _size: int) -> bytes:
                return b""

        cdp = object.__new__(smoke_web.CdpSocket)
        cdp.sock = ClosedSocket()
        cdp.current_command = "Runtime.evaluate"

        with self.assertRaisesRegex(RuntimeError, "closed while waiting for Runtime.evaluate"):
            cdp._read_exact(1)

    def test_websocket_close_frame_reports_code_reason_and_command(self) -> None:
        class FakeSocket:
            def sendall(self, _payload: bytes) -> None:
                pass

        cdp = object.__new__(smoke_web.CdpSocket)
        cdp.sock = FakeSocket()
        cdp.next_id = 1
        cdp._receive_frame = lambda: (8, b"\x03\xe8closing")

        with self.assertRaisesRegex(
            RuntimeError,
            "code=1000, reason='closing'.*Runtime.evaluate",
        ):
            cdp.command("Runtime.evaluate")


class ChromiumIsolationTests(unittest.TestCase):
    def test_chromium_smoke_disables_desktop_extensions_and_background_services(self) -> None:
        command = smoke_web.build_chromium_command("chromium", 9222, smoke_web.Path("/tmp/profile"))

        for flag in (
            "--disable-extensions",
            "--disable-background-networking",
            "--disable-component-update",
            "--disable-sync",
            "--disable-default-apps",
        ):
            with self.subTest(flag=flag):
                self.assertIn(flag, command)
        self.assertEqual(command[-1], "about:blank")

    def test_chromium_profile_keeps_singleton_socket_under_linux_path_limit(self) -> None:
        profile = "/tmp/mlsmoke-example"
        with patch.object(smoke_web.tempfile, "mkdtemp", return_value=profile) as mkdtemp:
            self.assertEqual(smoke_web.create_chromium_profile_directory(), profile)

        mkdtemp.assert_called_once_with(prefix="mlsmoke-", dir="/tmp")
        singleton_socket = f"{profile}/SingletonSocket"
        self.assertLess(len(singleton_socket.encode()), 108)


if __name__ == "__main__":
    unittest.main()
