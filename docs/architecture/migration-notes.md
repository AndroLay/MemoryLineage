# Repository migration notes

The workspace began as an extracted ZIP with Solidity, EVM scripts, Python
spikes, research, and generated artifacts sharing top-level locations. The
cleanup moves those inputs into explicit boundaries while preserving their
contents.

The migration does not intentionally change the registry semantics, signature
rules, state-root calculation, or independent replay model. Test results before
and after the move are recorded by the repository verification gate.
