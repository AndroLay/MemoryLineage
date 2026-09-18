import unittest
from tempfile import TemporaryDirectory
from pathlib import Path

from slippage_truth import evaluate_case, replay_evidence
from run_slippage_truth import run_spike


class SlippageTruthTest(unittest.TestCase):
    def test_static_floor_can_accept_adverse_output_that_live_floor_rejects(self):
        result = evaluate_case(
            {
                "case_id": "adverse-output",
                "static_min_out": 900,
                "reference_out": 1000,
                "expected_cost_bps": 300,
                "max_deviation_bps": 100,
                "hard_floor": 0,
                "recipient_delta": 930,
                "route_reported_out": 930,
                "oracle_fresh": True,
                "deadline_ok": True,
            }
        )

        self.assertTrue(result["static_accepted"])
        self.assertEqual(result["verdict"], "REJECT")
        self.assertIn("RECIPIENT_OUTPUT_BELOW_LIVE_FLOOR", result["failure_codes"])
        self.assertEqual(result["live_floor"], 960)

    def test_recipient_balance_delta_is_the_protected_output(self):
        result = evaluate_case(
            {
                "case_id": "unpaid-route-report",
                "static_min_out": 900,
                "reference_out": 1000,
                "expected_cost_bps": 300,
                "max_deviation_bps": 100,
                "hard_floor": 0,
                "recipient_delta": 0,
                "route_reported_out": 1000,
                "oracle_fresh": True,
                "deadline_ok": True,
            }
        )

        self.assertFalse(result["static_accepted"])
        self.assertEqual(result["verdict"], "REJECT")
        self.assertEqual(result["recipient_delta"], 0)
        self.assertIn("RECIPIENT_OUTPUT_BELOW_LIVE_FLOOR", result["failure_codes"])

    def test_stale_reference_fails_closed_even_when_output_is_large(self):
        result = evaluate_case(
            {
                "case_id": "stale-oracle",
                "static_min_out": 900,
                "reference_out": 1000,
                "expected_cost_bps": 300,
                "max_deviation_bps": 100,
                "hard_floor": 0,
                "recipient_delta": 1000,
                "route_reported_out": 1000,
                "oracle_fresh": False,
                "deadline_ok": True,
            }
        )

        self.assertEqual(result["verdict"], "REJECT")
        self.assertIn("STALE_REFERENCE", result["failure_codes"])

    def test_evidence_replay_matches_and_tampering_is_rejected(self):
        result = evaluate_case(
            {
                "case_id": "honest-output",
                "static_min_out": 900,
                "reference_out": 1000,
                "expected_cost_bps": 300,
                "max_deviation_bps": 100,
                "hard_floor": 950,
                "recipient_delta": 965,
                "route_reported_out": 965,
                "oracle_fresh": True,
                "deadline_ok": True,
            }
        )

        self.assertEqual(result["verdict"], "PASS")
        self.assertEqual(replay_evidence(result["evidence"]), "PASS")

        tampered = dict(result["evidence"])
        tampered["recipient_delta"] = 1
        with self.assertRaises(ValueError):
            replay_evidence(tampered)

    def test_runner_writes_a_replayable_five_case_summary(self):
        with TemporaryDirectory() as directory:
            summary = run_spike(Path(directory))

        self.assertEqual(summary["case_count"], 5)
        self.assertEqual(summary["replay_mismatches"], 0)
        self.assertEqual(summary["verdicts"], ["REJECT", "PASS", "REJECT", "REJECT", "REJECT"])


if __name__ == "__main__":
    unittest.main()
