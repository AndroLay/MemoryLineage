"""Fail-closed guard around LangGraph checkpoint retrieval."""

from __future__ import annotations

import asyncio
import json
import subprocess
from collections.abc import AsyncIterator, Callable, Iterator, Mapping, Sequence
from pathlib import Path
from typing import Any

try:
    from langgraph.checkpoint.base import BaseCheckpointSaver
except ModuleNotFoundError as error:
    if error.name not in {"langgraph", "langgraph.checkpoint"}:
        raise

    class BaseCheckpointSaver:  # type: ignore[no-redef]
        """Import fallback for stdlib-only adapter tests without LangGraph."""

        def __init__(self, *, serde: Any = None) -> None:
            self.serde = serde

from .canonicalization import (
    SNAPSHOT_PROFILE,
    SOURCE_CLASS,
    CanonicalizationError,
    canonicalize_checkpoint_tuple,
    make_recovery_input,
)

MAX_STDIN_BYTES = 16 * 1024 * 1024
MAX_STDOUT_BYTES = 1024 * 1024
EVIDENCE_INPUT_SCHEMA = "memorylineage-blinded-evidence-input-v1"
EVIDENCE_SCHEMA = "memorylineage-evidence-v2"
RESUME_ALLOWED = "RESUME_ALLOWED"


class RecoveryGateError(RuntimeError):
    """The protected checkpoint could not be assessed; the graph must stop."""


class RecoveryHeld(RecoveryGateError):
    """The evidence was valid but the selected checkpoint is not resumable."""

    def __init__(self, receipt: dict[str, Any]) -> None:
        self.receipt = receipt
        decision = receipt.get("decision", {})
        classification = decision.get("classification", "UNVERIFIED")
        action = decision.get("recommendedAction", "BLOCK_UNVERIFIED")
        super().__init__(f"MemoryLineage held checkpoint: {classification} / {action}")


def _run_cli(
    cli_path: str | Path,
    arguments: Sequence[str],
    payload: Mapping[str, Any],
    timeout_seconds: float,
) -> dict[str, Any]:
    try:
        encoded = bytearray(
            json.dumps(payload, ensure_ascii=False, allow_nan=False, separators=(",", ":")).encode(
                "utf-8"
            )
        )
    except (TypeError, ValueError, UnicodeError) as error:
        raise RecoveryGateError("recovery input could not be encoded") from error
    if len(encoded) > MAX_STDIN_BYTES:
        encoded[:] = b"\x00" * len(encoded)
        raise RecoveryGateError("recovery input exceeds the local gate size limit")

    try:
        completed = subprocess.run(
            [str(cli_path), *arguments],
            input=encoded,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            timeout=timeout_seconds,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise RecoveryGateError("the local MemoryLineage verifier was unavailable") from error
    finally:
        encoded[:] = b"\x00" * len(encoded)

    if completed.returncode != 0:
        raise RecoveryGateError("the local MemoryLineage verifier rejected its input")
    if len(completed.stdout) > MAX_STDOUT_BYTES:
        raise RecoveryGateError("the local verifier returned an oversized result")
    try:
        result = json.loads(completed.stdout)
    except (json.JSONDecodeError, UnicodeDecodeError) as error:
        raise RecoveryGateError("the local verifier returned invalid JSON") from error
    if not isinstance(result, dict):
        raise RecoveryGateError("the local verifier returned an unexpected result")
    return result


async def _run_cli_async(
    cli_path: str | Path,
    arguments: Sequence[str],
    payload: Mapping[str, Any],
    timeout_seconds: float,
) -> dict[str, Any]:
    """Run the local verifier without blocking the event loop or executor threads."""

    try:
        encoded = bytearray(
            json.dumps(payload, ensure_ascii=False, allow_nan=False, separators=(",", ":")).encode(
                "utf-8"
            )
        )
    except (TypeError, ValueError, UnicodeError) as error:
        raise RecoveryGateError("recovery input could not be encoded") from error
    if len(encoded) > MAX_STDIN_BYTES:
        encoded[:] = b"\x00" * len(encoded)
        raise RecoveryGateError("recovery input exceeds the local gate size limit")

    process: asyncio.subprocess.Process | None = None
    try:
        process = await asyncio.create_subprocess_exec(
            str(cli_path),
            *arguments,
            stdin=asyncio.subprocess.PIPE,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.DEVNULL,
        )
        try:
            stdout, _ = await asyncio.wait_for(process.communicate(encoded), timeout_seconds)
        except TimeoutError as error:
            process.kill()
            await process.communicate()
            raise RecoveryGateError("the local MemoryLineage verifier timed out") from error
    except RecoveryGateError:
        raise
    except asyncio.CancelledError:
        if process is not None and process.returncode is None:
            process.kill()
            await process.communicate()
        raise
    except (OSError, asyncio.SubprocessError) as error:
        if process is not None and process.returncode is None:
            process.kill()
            await process.wait()
        raise RecoveryGateError("the local MemoryLineage verifier was unavailable") from error
    finally:
        encoded[:] = b"\x00" * len(encoded)

    if process.returncode != 0:
        raise RecoveryGateError("the local verifier rejected its input")
    if len(stdout) > MAX_STDOUT_BYTES:
        raise RecoveryGateError("the local verifier returned an oversized result")
    try:
        result = json.loads(stdout)
    except (json.JSONDecodeError, UnicodeDecodeError) as error:
        raise RecoveryGateError("the local verifier returned invalid JSON") from error
    if not isinstance(result, dict):
        raise RecoveryGateError("the local verifier returned an unexpected result")
    return result


def generate_blinded_demo_evidence(
    checkpoint_tuples: Sequence[Any],
    blinding_secret: bytes,
    cli_path: str | Path,
    *,
    timeout_seconds: float = 10.0,
) -> dict[str, Any]:
    """Create local REVM evidence for exactly the three synthetic snapshots."""

    if len(checkpoint_tuples) != 3:
        raise CanonicalizationError("the local demo requires three LangGraph checkpoints")
    snapshots = [canonicalize_checkpoint_tuple(item) for item in checkpoint_tuples]
    if [item.sequence for item in snapshots] != [1, 2, 3]:
        raise CanonicalizationError("the local demo requires checkpoint sequences 1, 2, and 3")
    if len(blinding_secret) != 32 or not any(blinding_secret):
        raise CanonicalizationError("the local demo requires a non-zero 32-byte secret")
    payload = {
        "schemaVersion": EVIDENCE_INPUT_SCHEMA,
        "snapshotProfile": SNAPSHOT_PROFILE,
        "blindingSecret": "0x" + blinding_secret.hex(),
        "snapshots": [
            {"sequence": item.sequence, "values": item.values} for item in snapshots
        ],
    }
    evidence = _run_cli(
        cli_path,
        ["evidence", "demo-v2-blinded-json"],
        payload,
        timeout_seconds,
    )
    if evidence.get("schemaVersion") != EVIDENCE_SCHEMA or evidence.get("sourceClass") != SOURCE_CLASS:
        raise RecoveryGateError("the local verifier returned the wrong evidence profile")
    if evidence.get("head", {}).get("sequence") != 3:
        raise RecoveryGateError("the local verifier returned an unexpected evidence head")
    return evidence


class MemoryLineageCheckpointer(BaseCheckpointSaver):
    """Wrap a LangGraph saver so disallowed state never reaches graph code.

    The wrapped saver may deserialize its checkpoint before this guard runs.
    This adapter gates delivery to LangGraph; it does not secure the saver,
    storage permissions, or its deserializer. Use a strict serializer and a
    local-only secret provider for the synthetic demo.
    """

    def __init__(
        self,
        inner: Any,
        *,
        cli_path: str | Path,
        evidence_path: str | Path,
        secret_provider: Callable[[], bytes],
        timeout_seconds: float = 10.0,
        on_receipt: Callable[[dict[str, Any]], None] | None = None,
    ) -> None:
        if not 0 < timeout_seconds <= 60:
            raise ValueError("timeout_seconds must be between 0 and 60")
        self._inner = inner
        super().__init__(serde=getattr(inner, "serde", None))
        self._cli_path = str(cli_path)
        self._evidence_path = str(evidence_path)
        self._secret_provider = secret_provider
        self._timeout_seconds = timeout_seconds
        self._on_receipt = on_receipt

    def __getattr__(self, name: str) -> Any:
        try:
            inner = object.__getattribute__(self, "_inner")
        except AttributeError:
            raise AttributeError(name) from None
        return getattr(inner, name)

    @property
    def serde(self) -> Any:
        default_serde = getattr(BaseCheckpointSaver, "serde", None)
        return getattr(self._inner, "serde", default_serde)

    @serde.setter
    def serde(self, value: Any) -> None:
        self._inner.serde = value

    @property
    def config_specs(self) -> Any:
        return self._inner.config_specs

    def get_next_version(self, current: Any, channel: str) -> Any:
        return self._inner.get_next_version(current, channel)

    def put(self, config: Any, checkpoint: Any, metadata: Any, new_versions: Any) -> Any:
        return self._inner.put(config, checkpoint, metadata, new_versions)

    def put_writes(
        self, config: Any, writes: Any, task_id: str, task_path: str = ""
    ) -> Any:
        return self._inner.put_writes(config, writes, task_id, task_path)

    def delete_thread(self, thread_id: str) -> Any:
        return self._inner.delete_thread(thread_id)

    async def aput(
        self, config: Any, checkpoint: Any, metadata: Any, new_versions: Any
    ) -> Any:
        method = getattr(self._inner, "aput", None)
        if not callable(method):
            raise RecoveryGateError("the wrapped saver has no async put; use an async LangGraph saver")
        return await method(config, checkpoint, metadata, new_versions)

    async def aput_writes(
        self, config: Any, writes: Any, task_id: str, task_path: str = ""
    ) -> Any:
        method = getattr(self._inner, "aput_writes", None)
        if not callable(method):
            raise RecoveryGateError(
                "the wrapped saver has no async put_writes; use an async LangGraph saver"
            )
        return await method(config, writes, task_id, task_path)

    async def adelete_thread(self, thread_id: str) -> Any:
        method = getattr(self._inner, "adelete_thread", None)
        if not callable(method):
            raise RecoveryGateError(
                "the wrapped saver has no async delete_thread; use an async LangGraph saver"
            )
        return await method(thread_id)

    def _authorize(self, checkpoint_tuple: Any) -> None:
        try:
            secret = self._secret_provider()
            payload = make_recovery_input(checkpoint_tuple, secret)
        except Exception as error:
            if isinstance(error, RecoveryGateError):
                raise
            raise RecoveryGateError("checkpoint canonicalization or secret lookup failed") from error

        receipt = _run_cli(
            self._cli_path,
            ["recover", "preflight-json", self._evidence_path],
            payload,
            self._timeout_seconds,
        )
        if receipt.get("schemaVersion") != "memorylineage-recovery-receipt-v3":
            raise RecoveryGateError("the local verifier returned an unsupported receipt")
        verification = _run_cli(
            self._cli_path,
            ["recover", "verify-json", self._evidence_path],
            receipt,
            self._timeout_seconds,
        )
        if verification.get("verdict") != "VERIFIED":
            raise RecoveryGateError("the local recovery receipt did not verify")
        decision = receipt.get("decision")
        if not isinstance(decision, Mapping):
            raise RecoveryGateError("the recovery receipt has no decision")
        if decision.get("recommendedAction") != RESUME_ALLOWED:
            raise RecoveryHeld(receipt)
        if self._on_receipt is not None:
            try:
                self._on_receipt(receipt)
            except Exception as error:
                raise RecoveryGateError("the verified recovery receipt could not be recorded") from error

    def get_tuple(self, config: Any) -> Any:
        checkpoint_tuple = self._inner.get_tuple(config)
        if checkpoint_tuple is not None:
            self._authorize(checkpoint_tuple)
        return checkpoint_tuple

    def list(
        self,
        config: Any,
        *,
        filter: dict[str, Any] | None = None,
        before: Any = None,
        limit: int | None = None,
    ) -> Iterator[Any]:
        for checkpoint_tuple in self._inner.list(
            config, filter=filter, before=before, limit=limit
        ):
            self._authorize(checkpoint_tuple)
            yield checkpoint_tuple

    async def aget_tuple(self, config: Any) -> Any:
        method = getattr(self._inner, "aget_tuple", None)
        if not callable(method):
            raise RecoveryGateError(
                "the wrapped saver has no async get_tuple; use an async LangGraph saver"
            )
        checkpoint_tuple = await method(config)
        if checkpoint_tuple is not None:
            await self._authorize_async(checkpoint_tuple)
        return checkpoint_tuple

    async def alist(
        self,
        config: Any,
        *,
        filter: dict[str, Any] | None = None,
        before: Any = None,
        limit: int | None = None,
    ) -> AsyncIterator[Any]:
        method = getattr(self._inner, "alist", None)
        if not callable(method):
            raise RecoveryGateError(
                "the wrapped saver has no async list; use an async LangGraph saver"
            )
        async for checkpoint_tuple in method(
            config, filter=filter, before=before, limit=limit
        ):
            await self._authorize_async(checkpoint_tuple)
            yield checkpoint_tuple

    async def _authorize_async(self, checkpoint_tuple: Any) -> None:
        try:
            secret = self._secret_provider()
            payload = make_recovery_input(checkpoint_tuple, secret)
        except Exception as error:
            if isinstance(error, RecoveryGateError):
                raise
            raise RecoveryGateError("checkpoint canonicalization or secret lookup failed") from error

        receipt = await _run_cli_async(
            self._cli_path,
            ["recover", "preflight-json", self._evidence_path],
            payload,
            self._timeout_seconds,
        )
        if receipt.get("schemaVersion") != "memorylineage-recovery-receipt-v3":
            raise RecoveryGateError("the local verifier returned an unsupported receipt")
        verification = await _run_cli_async(
            self._cli_path,
            ["recover", "verify-json", self._evidence_path],
            receipt,
            self._timeout_seconds,
        )
        if verification.get("verdict") != "VERIFIED":
            raise RecoveryGateError("the local recovery receipt did not verify")
        decision = receipt.get("decision")
        if not isinstance(decision, Mapping):
            raise RecoveryGateError("the recovery receipt has no decision")
        if decision.get("recommendedAction") != RESUME_ALLOWED:
            raise RecoveryHeld(receipt)
        if self._on_receipt is not None:
            try:
                self._on_receipt(receipt)
            except Exception as error:
                raise RecoveryGateError("the verified recovery receipt could not be recorded") from error
