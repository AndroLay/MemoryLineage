#!/usr/bin/env python3
"""Print the local HTML pitch deck to a self-contained PDF."""

from __future__ import annotations

import shutil
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "docs/submission/pitch-deck.html"
OUTPUT = ROOT / "docs/submission/MemoryLineage-3rd-Web-Hack.pdf"


def main() -> None:
    browser = next(
        (path for name in ("chromium", "chromium-browser", "google-chrome")
         if (path := shutil.which(name))),
        None,
    )
    if not browser:
        raise RuntimeError("Chromium is required to print the pitch PDF")
    with tempfile.TemporaryDirectory(prefix="memorylineage-pitch-") as profile:
        result = subprocess.run(
            [
                browser, "--headless=new", "--no-sandbox", "--disable-gpu",
                "--disable-dev-shm-usage", "--no-first-run",
                "--disable-crash-reporter", "--disable-breakpad",
                f"--user-data-dir={profile}", "--allow-file-access-from-files",
                "--no-pdf-header-footer", f"--print-to-pdf={OUTPUT}",
                SOURCE.as_uri(),
            ],
            capture_output=True,
            text=True,
            check=False,
        )
    if result.returncode or not OUTPUT.is_file() or not OUTPUT.read_bytes().startswith(b"%PDF-"):
        tail = "\n".join(result.stderr.splitlines()[-12:])
        raise RuntimeError(f"Chromium PDF export failed ({result.returncode}):\n{tail}")
    OUTPUT.chmod(0o644)
    print(f"wrote {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
