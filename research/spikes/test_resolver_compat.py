import json
import copy
import tempfile
import unittest
from pathlib import Path

from resolver_compat import (
    MockResolver,
    evaluate_resolver,
    verify_evidence,
)


def valid_fixture():
    return {
        "fixture_id": "valid-order",
        "payload": {"order_id": "order-001", "nonce": 7},
        "resolved_order": {
            "steps": [
                {
                    "target": "0xsettler",
                    "selector": "0x12345678",
                    "arguments": [],
                    "attributes": [{"type": "RevertPolicy", "policy": "abort"}],
                }
            ],
            "variables": [
                {"role": "PaymentRecipient", "value": "0xsolver"},
                {"role": "PaymentChain", "value": 1},
            ],
            "payments": [
                {
                    "token": "0xtoken",
                    "sender": "0xuser",
                    "amount": 100,
                    "recipient_var_idx": 0,
                    "chain": 1,
                    "on_step_idx": 0,
                }
            ],
            "assumptions": [{"name": "trusted-settler", "data": "0xsettler"}],
        },
    }


POLICY = {
    "solver_address": "0xsolver",
    "allowed_assumptions": ["trusted-settler"],
    "allowed_targets": ["0xsettler"],
}


class ResolverCompatSpikeTests(unittest.TestCase):
    def test_valid_resolved_order_passes_and_second_verifier_replays_evidence(self):
        result = evaluate_resolver(MockResolver(valid_fixture()), POLICY)

        self.assertEqual(result["verdict"], "PASS")
        self.assertEqual(result["failed_checks"], [])
        self.assertEqual(verify_evidence(result["evidence"]), "PASS")

    def test_payment_recipient_mutation_is_rejected(self):
        fixture = valid_fixture()
        fixture["resolved_order"]["variables"][0]["value"] = "0xattacker"

        result = evaluate_resolver(MockResolver(fixture), POLICY)

        self.assertEqual(result["verdict"], "REJECT")
        self.assertIn("PAYMENT_RECIPIENT_MISMATCH", result["failure_codes"])
        self.assertEqual(verify_evidence(result["evidence"]), "REJECT")

    def test_unknown_assumption_is_unverified(self):
        fixture = valid_fixture()
        fixture["resolved_order"]["assumptions"][0]["name"] = "unknown-condition"

        result = evaluate_resolver(MockResolver(fixture), POLICY)

        self.assertEqual(result["verdict"], "UNVERIFIED")
        self.assertIn("UNKNOWN_ASSUMPTION", result["failure_codes"])
        self.assertEqual(verify_evidence(result["evidence"]), "UNVERIFIED")

    def test_evidence_round_trip_is_json_only(self):
        result = evaluate_resolver(MockResolver(valid_fixture()), POLICY)
        encoded = json.dumps(result["evidence"], sort_keys=True)
        evidence = json.loads(encoded)

        self.assertEqual(verify_evidence(evidence), "PASS")

    def test_evidence_can_be_written_for_external_replay(self):
        result = evaluate_resolver(MockResolver(valid_fixture()), POLICY)
        evidence_path = Path("evidence") / "generated" / "research-spikes" / "test-evidence.json"
        evidence_path.parent.mkdir(parents=True, exist_ok=True)
        evidence_path.write_text(json.dumps(result["evidence"], indent=2) + "\n")

        self.assertEqual(verify_evidence(json.loads(evidence_path.read_text())), "PASS")

    def test_twelve_mutations_never_pass_the_selected_policy(self):
        mutations = []

        fixture = valid_fixture()
        fixture["resolved_order"]["steps"][0]["target"] = "0xattacker"
        mutations.append(("target", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["steps"][0]["selector"] = "0x1234"
        mutations.append(("selector", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["variables"][0]["value"] = "0xattacker"
        mutations.append(("recipient-value", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["payments"][0]["chain"] = 2
        mutations.append(("payment-chain", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["payments"][0]["recipient_var_idx"] = 99
        mutations.append(("recipient-index", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["payments"][0]["on_step_idx"] = 99
        mutations.append(("step-index", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["steps"][0]["attributes"] = []
        mutations.append(("revert-policy", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["steps"][0]["arguments"] = [{"var_idx": 99}]
        mutations.append(("variable-index", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["assumptions"][0]["name"] = "unknown-condition"
        mutations.append(("unknown-assumption", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["payments"][0]["recipient_var_idx"] = 1
        mutations.append(("recipient-role", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["payments"][0]["chain"] = "0x1"
        mutations.append(("chain-type", fixture, POLICY))

        fixture = valid_fixture()
        fixture["resolved_order"]["payments"][0] = None
        mutations.append(("payment-structure", fixture, POLICY))

        self.assertEqual(len(mutations), 12)
        for name, fixture, policy in mutations:
            with self.subTest(mutation=name):
                result = evaluate_resolver(MockResolver(copy.deepcopy(fixture)), policy)
                self.assertIn(result["verdict"], {"REJECT", "UNVERIFIED"})
                self.assertEqual(verify_evidence(result["evidence"]), result["verdict"])

    def test_spike_runner_reports_three_fixtures_and_twelve_mutations(self):
        from run_spike import run_spike

        with tempfile.TemporaryDirectory() as directory:
            summary = run_spike(Path(directory))

        self.assertTrue(summary["gate_pass"])
        self.assertEqual(summary["fixture_count"], 3)
        self.assertEqual(summary["mutation_count"], 12)
        self.assertEqual(summary["mutation_non_pass_count"], 12)
        self.assertEqual(summary["replay_mismatches"], 0)


if __name__ == "__main__":
    unittest.main()
