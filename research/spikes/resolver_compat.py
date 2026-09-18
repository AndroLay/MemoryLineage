"""Small, dependency-free ResolverCompat feasibility probe.

This module is intentionally a throwaway spike. It models the solver-facing
shape of an ERC-7683 resolver and tests a narrow, explicit local policy. It
does not prove economic safety or full ERC-7683 compliance.
"""

from __future__ import annotations

import copy
import hashlib
import json
import re
from typing import Any


EVIDENCE_SCHEMA = "resolver-compat-evidence-v1"
VERDICTS = {"PASS", "REJECT", "UNVERIFIED"}


def _canonical(value: Any) -> str:
    return json.dumps(value, ensure_ascii=True, sort_keys=True, separators=(",", ":"))


def _digest(value: Any) -> str:
    return "sha256:" + hashlib.sha256(_canonical(value).encode("utf-8")).hexdigest()


class MockResolver:
    """A deterministic resolver-shaped fixture used by the spike."""

    def __init__(self, fixture: dict[str, Any]):
        self.fixture = copy.deepcopy(fixture)
        self.calls = 0

    @property
    def fixture_id(self) -> str:
        return str(self.fixture.get("fixture_id", "unknown-fixture"))

    @property
    def payload(self) -> dict[str, Any]:
        return copy.deepcopy(self.fixture.get("payload", {}))

    def resolve(self, payload: dict[str, Any]) -> dict[str, Any]:
        self.calls += 1
        if payload != self.fixture.get("payload", {}):
            raise ValueError("mock resolver received an unexpected payload")
        return copy.deepcopy(self.fixture["resolved_order"])


def _check(
    checks: list[dict[str, str]],
    code: str,
    status: str,
    path: str,
    detail: str,
) -> None:
    checks.append(
        {
            "code": code,
            "status": status,
            "path": path,
            "detail": detail,
        }
    )


def _finish(
    *,
    resolver: Any,
    fixture_id: str,
    payload: dict[str, Any],
    policy: dict[str, Any],
    checks: list[dict[str, str]],
) -> dict[str, Any]:
    failures = [check for check in checks if check["status"] != "PASS"]
    failure_codes = list(dict.fromkeys(check["code"] for check in failures))
    if any(check["status"] == "FAIL" for check in failures):
        verdict = "REJECT"
    elif failures:
        verdict = "UNVERIFIED"
    else:
        verdict = "PASS"

    evidence_body = {
        "schema_version": EVIDENCE_SCHEMA,
        "fixture_id": fixture_id,
        "payload_hash": _digest(payload),
        "policy_hash": _digest(policy),
        "verdict": verdict,
        "failure_codes": failure_codes,
        "checks": checks,
        "resolver_calls": int(getattr(resolver, "calls", 1)),
        "replay": {
            "verifier": "spike.resolver_compat.verify_evidence",
            "resolver_calls": 0,
        },
    }
    evidence = dict(evidence_body)
    evidence["evidence_hash"] = _digest(evidence_body)
    return {
        "verdict": verdict,
        "failure_codes": failure_codes,
        "failed_checks": failures,
        "checks": checks,
        "evidence": evidence,
    }


def evaluate_resolver(resolver: Any, policy: dict[str, Any]) -> dict[str, Any]:
    """Resolve one payload and evaluate only the selected local invariants."""

    fixture_id = str(getattr(resolver, "fixture_id", "unknown-fixture"))
    payload = copy.deepcopy(getattr(resolver, "payload", {}))
    checks: list[dict[str, str]] = []

    try:
        resolved = resolver.resolve(payload)
    except Exception as exc:  # pragma: no cover - defensive boundary for the probe
        _check(checks, "RESOLVER_CALL_FAILED", "FAIL", "resolver", str(exc))
        return _finish(
            resolver=resolver,
            fixture_id=fixture_id,
            payload=payload,
            policy=policy,
            checks=checks,
        )

    if not isinstance(resolved, dict):
        _check(checks, "STRUCTURE_INVALID", "FAIL", "resolved_order", "must be an object")
        return _finish(
            resolver=resolver,
            fixture_id=fixture_id,
            payload=payload,
            policy=policy,
            checks=checks,
        )

    required = ("steps", "variables", "payments", "assumptions")
    missing = [key for key in required if not isinstance(resolved.get(key), list)]
    if missing:
        _check(
            checks,
            "STRUCTURE_INVALID",
            "FAIL",
            "resolved_order",
            "missing list fields: " + ", ".join(missing),
        )
        return _finish(
            resolver=resolver,
            fixture_id=fixture_id,
            payload=payload,
            policy=policy,
            checks=checks,
        )
    _check(checks, "STRUCTURE_VALID", "PASS", "resolved_order", "required lists are present")

    steps = resolved["steps"]
    variables = resolved["variables"]
    payments = resolved["payments"]
    assumptions = resolved["assumptions"]

    allowed_targets = set(policy.get("allowed_targets", []))
    target_failed = False
    for index, step in enumerate(steps):
        target = step.get("target") if isinstance(step, dict) else None
        selector = step.get("selector") if isinstance(step, dict) else None
        if target not in allowed_targets:
            target_failed = True
            _check(
                checks,
                "TARGET_NOT_ALLOWED",
                "FAIL",
                f"steps[{index}].target",
                f"target {target!r} is outside the local allowlist",
            )
        if not isinstance(selector, str) or not re.fullmatch(r"0x[0-9a-fA-F]{8}", selector):
            target_failed = True
            _check(
                checks,
                "INVALID_SELECTOR",
                "FAIL",
                f"steps[{index}].selector",
                "selector must be four bytes encoded as 0x plus eight hex digits",
            )
        attributes = step.get("attributes", []) if isinstance(step, dict) else []
        if not any(
            isinstance(attribute, dict) and attribute.get("type") == "RevertPolicy"
            for attribute in attributes
        ):
            _check(
                checks,
                "MISSING_REVERT_POLICY",
                "FAIL",
                f"steps[{index}].attributes",
                "every call must state its revert policy",
            )
    if not target_failed:
        _check(checks, "TARGET_AND_SELECTOR_VALID", "PASS", "steps", "targets and selectors pass policy")

    recipient_vars = [
        (index, variable)
        for index, variable in enumerate(variables)
        if isinstance(variable, dict) and variable.get("role") == "PaymentRecipient"
    ]
    payment_chain_vars = [
        (index, variable)
        for index, variable in enumerate(variables)
        if isinstance(variable, dict) and variable.get("role") == "PaymentChain"
    ]
    solver_address = policy.get("solver_address")
    recipient_failed = False
    chain_failed = False
    payment_index_failed = False
    for index, payment in enumerate(payments):
        if not isinstance(payment, dict):
            _check(checks, "PAYMENT_INVALID", "FAIL", f"payments[{index}]", "payment must be an object")
            continue
        recipient_index = payment.get("recipient_var_idx")
        if not isinstance(recipient_index, int) or not (0 <= recipient_index < len(variables)):
            recipient_failed = True
            _check(
                checks,
                "PAYMENT_RECIPIENT_INDEX_INVALID",
                "FAIL",
                f"payments[{index}].recipient_var_idx",
                "recipient variable index is outside the variables array",
            )
        elif variables[recipient_index].get("role") != "PaymentRecipient":
            recipient_failed = True
            _check(
                checks,
                "PAYMENT_RECIPIENT_ROLE_INVALID",
                "FAIL",
                f"payments[{index}].recipient_var_idx",
                "payment recipient must reference a PaymentRecipient variable",
            )
        elif variables[recipient_index].get("value") != solver_address:
            recipient_failed = True
            _check(
                checks,
                "PAYMENT_RECIPIENT_MISMATCH",
                "FAIL",
                f"variables[{recipient_index}].value",
                "payment recipient does not match the configured solver address",
            )

        step_index = payment.get("on_step_idx")
        if not isinstance(step_index, int) or not (0 <= step_index < len(steps)):
            payment_index_failed = True
            _check(
                checks,
                "PAYMENT_STEP_INDEX_INVALID",
                "FAIL",
                f"payments[{index}].on_step_idx",
                "payment step index is outside the steps array",
            )

        payment_chain = payment.get("chain")
        valid_chain_values = [variable.get("value") for _, variable in payment_chain_vars]
        if payment_chain not in valid_chain_values:
            chain_failed = True
            _check(
                checks,
                "PAYMENT_CHAIN_MISMATCH",
                "FAIL",
                f"payments[{index}].chain",
                "payment chain does not match a PaymentChain variable",
            )

    if not recipient_failed:
        _check(checks, "PAYMENT_RECIPIENT_VALID", "PASS", "payments", "recipient binding passes policy")
    if not chain_failed:
        _check(checks, "PAYMENT_CHAIN_VALID", "PASS", "payments", "chain binding passes policy")
    if not payment_index_failed:
        _check(checks, "PAYMENT_STEP_BINDING_VALID", "PASS", "payments", "payment step indexes are valid")

    variable_ref_failed = False
    for step_index, step in enumerate(steps):
        for argument_index, argument in enumerate(step.get("arguments", [])):
            if isinstance(argument, dict) and "var_idx" in argument:
                variable_index = argument["var_idx"]
                if not isinstance(variable_index, int) or not (0 <= variable_index < len(variables)):
                    variable_ref_failed = True
                    _check(
                        checks,
                        "VARIABLE_INDEX_INVALID",
                        "FAIL",
                        f"steps[{step_index}].arguments[{argument_index}]",
                        "argument variable index is outside the variables array",
                    )
    if not variable_ref_failed:
        _check(checks, "VARIABLE_REFERENCES_VALID", "PASS", "steps", "variable references are valid")

    unknown_assumptions = [
        assumption.get("name")
        for assumption in assumptions
        if not isinstance(assumption, dict)
        or assumption.get("name") not in set(policy.get("allowed_assumptions", []))
    ]
    if unknown_assumptions:
        _check(
            checks,
            "UNKNOWN_ASSUMPTION",
            "UNVERIFIED",
            "assumptions",
            "local policy does not recognize: " + ", ".join(map(str, unknown_assumptions)),
        )
    else:
        _check(checks, "ASSUMPTIONS_KNOWN", "PASS", "assumptions", "all assumptions have local policy")

    return _finish(
        resolver=resolver,
        fixture_id=fixture_id,
        payload=payload,
        policy=policy,
        checks=checks,
    )


def verify_evidence(evidence: dict[str, Any]) -> str:
    """Replay a verdict from evidence only; this function never calls a resolver."""

    if not isinstance(evidence, dict) or evidence.get("schema_version") != EVIDENCE_SCHEMA:
        raise ValueError("unsupported evidence schema")
    stored_hash = evidence.get("evidence_hash")
    body = {key: value for key, value in evidence.items() if key != "evidence_hash"}
    if stored_hash != _digest(body):
        raise ValueError("evidence hash mismatch")

    checks = evidence.get("checks")
    if not isinstance(checks, list):
        raise ValueError("evidence checks must be a list")
    statuses = {check.get("status") for check in checks if isinstance(check, dict)}
    if not statuses.issubset({"PASS", "FAIL", "UNVERIFIED"}):
        raise ValueError("evidence has an invalid check status")

    expected = "REJECT" if "FAIL" in statuses else "UNVERIFIED" if "UNVERIFIED" in statuses else "PASS"
    if evidence.get("verdict") != expected:
        raise ValueError("evidence verdict does not match check statuses")
    expected_codes = list(
        dict.fromkeys(
            check["code"]
            for check in checks
            if isinstance(check, dict) and check.get("status") != "PASS"
        )
    )
    if evidence.get("failure_codes") != expected_codes:
        raise ValueError("evidence failure codes do not match checks")
    if evidence.get("replay", {}).get("resolver_calls") != 0:
        raise ValueError("replay verifier must not call a resolver")
    return expected
