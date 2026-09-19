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

## Commands

Record the exact commands and whether each completed successfully.

| Command | Result | Notes |
| --- | --- | --- |
| `cargo xtask reproduce` | `PASS / FAIL` | |
| static release server command | `PASS / FAIL` | |
| `/lab/silent-rollback` browser flow | `PASS / FAIL` | |
| `/verify` original bundle | `PASS / FAIL` | |
| `/verify` tampered bundle | `PASS / FAIL` | |
| `/verify` restored bundle | `PASS / FAIL` | |
| independent CLI verification | `PASS / FAIL` | |

## Expected observations

- Silent Rollback terminal reason:
- Tampered evidence result:
- Restored evidence result:
- Independent CLI verdict:
- Any unexpected live/fallback source label:

## Comprehension

Answer in your own words before reading another reviewer report.

### What does MemoryLineage prove?

Answer:

### What does MemoryLineage explicitly not prove?

Answer:

## Reproduction result

- Overall result: `REPRODUCED / PARTIAL / FAILED`
- Blocking issue, if any:
- Suggested documentation or UX change:

This report is evidence of one developer's reproduction experience. It is not a
security audit, a formal verification result, or evidence of future adoption.
