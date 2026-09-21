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

The current tagged review candidate is
[`v1.0.1-rc.3`](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1-rc.3).
For an exact checkout, use:

```bash
git checkout v1.0.1-rc.3
```

The repository is intentionally able to run the core path without deployment:
the public synthetic fixture and portable evidence are bundled in the source
tree. The static Inspector is also available at
[memorylineage.pages.dev](https://memorylineage.pages.dev), but this
reproduction does not depend on the hosted site or a new Sepolia deployment.

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

## Reviewer archive

After checking out the exact release candidate, create a clean source/evidence
archive:

```bash
cargo xtask reviewer-package /tmp/memorylineage-reviewer-package.tar.gz
```

The command refuses a dirty worktree and checks the resulting archive for
private design references, generated build output, dependencies, credentials,
and generated evidence. This archive is a transportable source/evidence review
surface independent of hosted website availability; it does not make a private
GitHub repository public or configure the website deployment.

A reviewer who receives the archive can run the same path without GitHub
access:

```bash
mkdir -p /tmp/memorylineage-review
tar -xzf /path/to/memorylineage-reviewer-package.tar.gz \
  -C /tmp/memorylineage-review
cd /tmp/memorylineage-review
cargo xtask reproduce
```

After the gate builds the static site, a reviewer can inspect it locally with
an SPA fallback server, for example:

```bash
python3 -m http.server 4173 \
  --directory target/dx/memorylineage-inspector/release/web/public
```

The archive is the source/evidence transport surface; it does not expose the
private GitHub repository or create a public website.

The owner can verify that the transportable packet is runnable without Git
metadata by running:

```bash
cargo xtask reviewer-reproduce
```

This creates the archive from the clean committed `HEAD`, extracts it into a
temporary checkout, and runs `cargo xtask reproduce` there. It is still an
automated owner-side check; it does not become an external human report.

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
