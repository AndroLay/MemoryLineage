import copy
import unittest

from memory_lineage.model import (
    PUBLISHED_VECTOR,
    ZERO_ROOT,
    audit_history,
    make_transition,
    space_id,
    transition_id,
    verify_published_vector,
)
from memory_lineage.demo import AUTHOR, PROFILE, SALT, SPACE, valid_history


class MemoryLineageTest(unittest.TestCase):
    def test_published_golden_vector_recomputes(self):
        checks = verify_published_vector()
        self.assertTrue(all(checks.values()), checks)

    def test_valid_history_is_accepted(self):
        result = audit_history(valid_history(), expected_authorizer=AUTHOR)
        self.assertEqual(result["verdict"], "PASS")
        self.assertEqual(result["accepted_transitions"], 4)

    def test_rollback_and_gap_are_rejected(self):
        base = valid_history()
        rollback = list(base)
        rollback[2] = make_transition(
            space=SPACE,
            sequence=3,
            previous_root=base[0].next_state_root,
            authorizer=AUTHOR,
            payload=base[2].payload,
            provenance=base[2].provenance,
            locator=base[2].locator,
            profile_id=PROFILE,
            delta_salt=SALT,
            provenance_salt="0x" + "cc" * 32,
            locator_salt="0x" + "bb" * 32,
        )
        result = audit_history(rollback, expected_authorizer=AUTHOR)
        self.assertEqual(result["verdict"], "REJECT")
        self.assertIn("PREDECESSOR_MISMATCH", result["failure_codes"])

    def test_locator_and_payload_mutations_fail_closed(self):
        base = valid_history()
        locator = copy.deepcopy(base)
        locator[1].locator = "ipfs://substituted"
        payload = copy.deepcopy(base)
        payload[1].payload = '{"op":"poison"}'
        for history, expected in (
            (locator, "LOCATOR_COMMITMENT_MISMATCH"),
            (payload, "DELTA_COMMITMENT_MISMATCH"),
        ):
            result = audit_history(history, expected_authorizer=AUTHOR)
            self.assertEqual(result["verdict"], "REJECT")
            self.assertIn(expected, result["failure_codes"])

    def test_space_id_matches_published_vector(self):
        self.assertEqual(
            space_id(
                "0x2222222222222222222222222222222222222222",
                PUBLISHED_VECTOR["space_salt"],
            ),
            "0xef5037465ae0323637cb58434eb554ae4a1fe1131bcc10888900f2d4cbe349d8",
        )

    def test_transition_id_changes_when_signed_field_changes(self):
        transition = valid_history()[0]
        mutated = dict(transition.delta)
        mutated["profile_id"] = "0x" + "99" * 32
        self.assertNotEqual(transition_id(transition.delta), transition_id(mutated))
        self.assertEqual(transition.delta["prev_state_root"], ZERO_ROOT)


if __name__ == "__main__":
    unittest.main()
