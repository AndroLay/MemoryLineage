import json
import unittest
from pathlib import Path

try:
    from memory_lineage.independent import verify_bundle
except ImportError:
    from memory_lineage.independent import verify_bundle


class IndependentMemoryLineageTest(unittest.TestCase):
    def setUp(self):
        evidence_path = Path(__file__).resolve().parents[3] / "evidence" / "local" / "memory_lineage_evm_evidence.json"
        self.evidence = json.loads(evidence_path.read_text())

    def test_exported_evm_bundle_replays_without_publisher(self):
        result = verify_bundle(self.evidence)
        self.assertEqual(result["verdict"], "PASS")
        self.assertEqual(result["validTransitions"], 4)
        self.assertEqual(result["mutationRejected"], 20)

    def test_tampered_public_transition_is_rejected(self):
        tampered = json.loads(json.dumps(self.evidence))
        tampered["validHistory"][2]["nextStateRoot"] = "0x" + "11" * 32
        with self.assertRaises(AssertionError):
            verify_bundle(tampered)

    def test_private_payload_does_not_enter_export(self):
        serialized = json.dumps(self.evidence)
        self.assertNotIn('"payload"', serialized)
        self.assertNotIn('"provenance"', serialized)
        self.assertNotIn('"locator"', serialized)


if __name__ == "__main__":
    unittest.main()
