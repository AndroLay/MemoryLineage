import unittest
from pathlib import Path

from memory_lineage.impact import summarize_evidence


ROOT = Path(__file__).resolve().parents[1]


class MemoryLineageImpactTest(unittest.TestCase):
    def test_summary_separates_machine_metrics_from_human_validation(self):
        local = {
            "evidenceType": "workspace_owned_local_evm",
            "validHistory": {
                "transitions": [{"sequence": 1}, {"sequence": 2}],
                "gas": [
                    {"sequence": 1, "totalGasSpent": "100"},
                    {"sequence": 2, "totalGasSpent": "50"},
                ],
            },
            "mutationCount": 3,
            "mutationRejectedCount": 3,
        }
        replay = {
            "verdict": "PASS",
            "validTransitions": 2,
            "mutationRejected": 3,
        }

        result = summarize_evidence(local, replay, replay_runs=4, replay_ms=[1.0, 2.0, 3.0, 4.0])

        self.assertEqual(result["machineMetrics"]["falseRejectRateOnValidCorpus"], 0.0)
        self.assertEqual(result["machineMetrics"]["mutationRejectionRate"], 1.0)
        self.assertEqual(result["machineMetrics"]["gas"]["firstTransition"], 100)
        self.assertEqual(result["machineMetrics"]["gas"]["continuationMean"], 50.0)
        self.assertEqual(result["replayBenchmark"]["runs"], 4)
        self.assertEqual(result["developerValidation"]["status"], "PENDING")
        self.assertEqual(result["developerValidation"]["completed"], 0)


if __name__ == "__main__":
    unittest.main()
