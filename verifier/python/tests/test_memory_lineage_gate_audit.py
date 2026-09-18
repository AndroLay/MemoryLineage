import unittest

from memory_lineage.gates import evaluate_gates


class MemoryLineageGateAuditTest(unittest.TestCase):
    def test_current_shape_keeps_external_gates_false(self):
        result = evaluate_gates(
            local={
                "evidenceType": "workspace_owned_local_evm",
                "conformance": {"allMatch": True},
                "validHistory": {"transitions": [{}, {}, {}, {}]},
                "mutationCount": 20,
                "mutationRejectedCount": 20,
            },
            replay={"verdict": "PASS", "validTransitions": 4, "replayMismatches": 0, "mutationRejected": 20},
            deployment={},
            reread={},
            impact={"status": "PENDING", "completed": 0},
        )

        self.assertTrue(result["localEvm"]["contractConformance"])
        self.assertTrue(result["localEvm"]["independentReplay"])
        self.assertFalse(result["externalGates"]["workspacePublicTestnetDeployment"])
        self.assertFalse(result["externalGates"]["independentDeveloperValidation"])
        self.assertFalse(result["evidenceGatesPass"])

    def test_evidence_gate_requires_all_external_evidence(self):
        deployment = {
            "evidenceType": "workspace_owned_public_sepolia_deployment",
            "chainId": "11155111",
            "registryAddress": "0xregistry",
        }
        reread = {
            "evidenceType": "second_rpc_reread_workspace_owned_public_sepolia_deployment",
            "verdict": "PASS",
            "registryAddress": "0xregistry",
            "checks": {"codeHashMatches": True},
        }
        impact = {
            "status": "PASS",
            "completed": 2,
            "requiredIndependentDevelopers": 2,
            "runs": [
                {
                    "developerAlias": "a",
                    "timestamp": "2026-09-08T00:00:00Z",
                    "checkoutHash": "hash-a",
                    "answers": {},
                    "observedVerdicts": {},
                    "replayMilliseconds": 100,
                    "mismatches": 0,
                },
                {
                    "developerAlias": "b",
                    "timestamp": "2026-09-08T00:01:00Z",
                    "checkoutHash": "hash-b",
                    "answers": {},
                    "observedVerdicts": {},
                    "replayMilliseconds": 101,
                    "mismatches": 0,
                },
            ],
            "acceptance": {
                "validTransitionRecognized": True,
                "sequenceGapRecognized": True,
                "signatureBindingRecognized": True,
                "privacyBoundaryRecognized": True,
                "semanticTruthLimitationRecognized": True,
                "observedVerdictsMatchIndependentReplay": True,
            },
        }

        result = evaluate_gates(
            local={
                "evidenceType": "workspace_owned_local_evm",
                "conformance": {"allMatch": True},
                "validHistory": {"transitions": [{}, {}, {}, {}]},
                "mutationCount": 20,
                "mutationRejectedCount": 20,
            },
            replay={"verdict": "PASS", "validTransitions": 4, "replayMismatches": 0, "mutationRejected": 20},
            deployment=deployment,
            reread=reread,
            impact=impact,
        )

        self.assertTrue(result["evidenceGatesPass"])
        self.assertEqual(result["validationEvidence"]["mode"], "unspecified")
        self.assertFalse(result["validationEvidence"]["humanDeveloperValidation"])


if __name__ == "__main__":
    unittest.main()
