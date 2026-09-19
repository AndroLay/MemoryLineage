# Independent clean-checkout reproduction

This runbook is for a developer who has not received a private walkthrough of
MemoryLineage. It tests whether the repository, evidence, and website explain
the same claim from a fresh checkout.

The result of `cargo xtask reproduce` is an automated local result. It must not
be recorded as an external human reproduction. A human report is only valid
when another developer runs the steps and completes the report template in
this directory.

## Reviewer packet

Give the reviewer:

1. the repository URL;
2. the exact commit to check out;
3. this runbook;
4. no additional explanation of the product.

The repository is intentionally able to run the core path without deployment:
the public synthetic fixture and portable evidence are bundled in the source
tree. A hosted website or a new Sepolia deployment is not required for this
reproduction.

## Automated path

From a clean checkout:

```bash
cargo xtask reproduce
```

The command checks the pinned Rust toolchain, workspace and independent
verifier gates, Demo Space V2 regeneration, Rust/revm execution, the static
Dioxus build, Chromium route/interaction smoke, and the release package
boundary. It prints an explicit note that external human reproduction is still
unproven.

## Human comprehension path

After the automated command passes:

1. Open the static release artifact with an SPA fallback server.
2. Open `/` and state the problem in your own words.
3. Open `/inspect` and identify the current Demo Space V2 head.
4. Open `/lab/silent-rollback` and run the scenario.
5. Confirm the terminal result contains `BAD_PREVIOUS_STATE`.
6. Open `/verify`, load the published Demo Space V2 evidence, and verify it.
7. Tamper the tested commitment and confirm `TRANSITION_ID_MISMATCH`.
8. Restore the original bundle and confirm `VERIFIED`.
9. Run the independent CLI verification command shown by the page.

Do not use the screenshot references in `internal/`; they are private design
references and are not part of the reviewer packet.

## Required report

Copy [external-developer-report.md](external-developer-report.md), complete it
without changing the question wording, and place the returned report under:

```text
evidence/reproduction/
```

Only completed reports from real external developers belong there. Empty,
placeholder, or self-authored reports must stay out of the release evidence.

The two comprehension questions are deliberately required:

- What does MemoryLineage prove?
- What does MemoryLineage explicitly not prove?

If the reviewer describes semantic memory poisoning detection, the report is a
product-clarity failure to fix, not a successful adoption result.
