import copy
import json
import unittest

from memory_lineage.portable import verify_portable_bundle


def bundle():
    root = "0x" + "00" * 32
    return {
        "schemaVersion": "memorylineage-evidence-v1",
        "evidenceType": "memorylineage_inspector_export",
        "network": {"name": "Local EthereumJS", "chainId": "31337"},
        "registry": {"address": "0x" + "11" * 20, "spaceId": "0x" + "22" * 32},
        "transitions": [
            {
                "sequence": 1,
                "prevStateRoot": root,
                "nextStateRoot": "0x" + "33" * 32,
                "transitionId": "0x" + "44" * 32,
                "deltaCommitment": "0x" + "55" * 32,
                "provenanceCommitment": "0x" + "66" * 32,
                "profileId": "0x" + "77" * 32,
                "locatorCommitment": "0x" + "88" * 32,
            }
        ],
        "head": {"sequence": 1, "transitionId": "0x" + "44" * 32, "stateRoot": "0x" + "33" * 32},
        "privacy": {"rawMemoryOnChain": False},
    }


class PortableEvidenceTest(unittest.TestCase):
    def test_valid_bundle_is_verified(self):
        result = verify_portable_bundle(bundle())
        self.assertEqual(result["verdict"], "VERIFIED")

    def test_stale_predecessor_is_rejected(self):
        candidate = copy.deepcopy(bundle())
        candidate["transitions"][0]["prevStateRoot"] = "0x" + "99" * 32
        with self.assertRaisesRegex(AssertionError, "BAD_PREVIOUS_STATE"):
            verify_portable_bundle(candidate)

    def test_private_field_is_rejected(self):
        candidate = copy.deepcopy(bundle())
        candidate["transitions"][0]["payload"] = "private text"
        with self.assertRaisesRegex(AssertionError, "PRIVATE_FIELD_EXPORTED:payload"):
            verify_portable_bundle(candidate)

    def test_head_mismatch_is_rejected(self):
        candidate = copy.deepcopy(bundle())
        candidate["head"]["stateRoot"] = "0x" + "aa" * 32
        with self.assertRaisesRegex(AssertionError, "HEAD_ROOT_MISMATCH"):
            verify_portable_bundle(candidate)
