# Snapshot commitment privacy profile

Demo Space V2 uses a deterministic V2 commitment over public synthetic SQLite
fixtures. Its length-prefixed encoding prevents delimiter ambiguity, but its
unsalted hash is not a hiding commitment for low-entropy private memory.
Anyone who can guess the complete snapshot may test that guess against a
published deterministic commitment.

`ml-memory-store::snapshot_commitment_blinded_v1` is an opt-in preparation
helper for future adapters. It hashes a domain-separated V2 encoding together
with a 32-byte space ID and a caller-provided 32-byte blinding secret. The
all-zero secret is rejected. A real caller must generate a fresh secret from a
cryptographically secure random source for each state, retain it privately,
and keep it out of chain calldata, portable evidence, logs, URLs, analytics,
and screenshots. Reusing, losing, publishing, or predictably deriving the
secret changes the privacy and recovery properties.

This helper is not wired into Demo Space V2, a production agent framework, or
the current public evidence format. The repository does not yet provide secret
storage, rotation, recovery, independent privacy review, or an upgrade path for
existing V2 commitments. Therefore no production privacy claim follows from
the helper. An independent auditor without the raw snapshot and secret can
verify public lineage, but cannot independently reconstruct the private
snapshot commitment.
