# Local submission evidence envelope

[`manifest.json`](manifest.json) identifies the Demo Space V2 Silent Rollback
incident and records SHA-256 hashes for each referenced artifact. Its source
classes distinguish the local Rust/revm incident from the separate protocol
corpus. `submissionCommit` is `null`: a Git commit cannot contain its own final
SHA without changing that SHA. The exact submitted commit belongs in release
metadata and each external review report after a commit exists.

Check the package from the repository root:

```bash
cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json
```

The CLI checks artifact hashes, the SQLite snapshot commitments, the
transition-1 state root used as a stale predecessor, the canonical head, a
fresh local Rust/revm execution, the V2 replay, and both recovery receipts.
The checked-in JSON can be regenerated with
`python3 scripts/build_submission_bundle.py` after the underlying evidence is
intentionally changed. The package hash binds bytes inside this repository;
it does not authenticate a public chain observation by itself.

The current verdict is `VERIFIED_LOCAL_PACKAGE`. External human reproduction,
hosted CI, a public Demo Space V2 deployment, production agent adoption, and
formal security auditing retain their separate evidence requirements.
