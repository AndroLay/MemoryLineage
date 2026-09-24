# MemoryLineage external developer reproduction report

This report records an independent clean-checkout reproduction. Complete it
from the exact repository commit supplied by the project owner. Do not infer
results from CI, a previous report, or a private walkthrough.

## Identity and environment

- Reviewer identifier or pseudonym:
- Date/time with timezone:
- Repository URL:
- Exact commit SHA:
- Operating system and version:
- CPU / architecture:
- Rust version (`rustc --version`):
- Cargo version (`cargo --version`):
- Dioxus CLI version (`dx --version`):
- Browser and version:
- Session start time with timezone:
- First correct explanation time (or `NOT_REACHED`):
- First point of confusion (unedited note):

## Commands

Record the exact commands and whether each completed successfully.

| Command | Result | Notes |
| --- | --- | --- |
| `cargo xtask reproduce` | `PASS / FAIL` | |
| `cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json` | `PASS / FAIL` | Record the `VERIFIED_LOCAL_PACKAGE` verdict and any failed row |
| static release server command | `PASS / FAIL` | |
| `/lab/silent-rollback` browser flow | `PASS / FAIL` | |
| `/verify` original bundle | `PASS / FAIL` | |
| `/verify` tampered bundle | `PASS / FAIL` | |
| `/verify` restored bundle | `PASS / FAIL` | |
| independent CLI verification | `PASS / FAIL` | |

Record each task without a private prompt.

| Task | Completed `YES / NO` | First confusion or failure |
| --- | --- | --- |
| Open Home and explain the problem | | |
| Find the current canonical head | | |
| Use **Check an old restore** | | |
| Observe `BAD_PREVIOUS_STATE` | | |
| Open Verify and observe `VERIFIED` | | |
| Tamper one commitment and observe `TRANSITION_ID_MISMATCH` | | |
| Restore the original and observe `VERIFIED` | | |
| Run the independent CLI and package verification | | |

## Expected observations

- Silent Rollback terminal reason:
- Tampered evidence result:
- Restored evidence result:
- Independent CLI verdict:
- Submission package verdict and incident ID:
- Any unexpected live/fallback source label:

## Comprehension

Answer in your own words before reading another reviewer report.

### What does MemoryLineage check before an agent resumes from a private snapshot?

Answer:

### What does MemoryLineage explicitly not prove about the memory or the agent?

Answer:

## Reproduction result

- Overall result: `REPRODUCED / PARTIAL / FAILED`
- Blocking issue, if any:
- Suggested documentation or UX change:

This report is evidence of one developer's reproduction experience. It is not a
security audit, a formal verification result, or evidence of future adoption.
