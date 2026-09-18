#![forbid(unsafe_code)]

use crate::browser::{download_json, live_silent_rollback};
use crate::components::*;
use crate::data::{Scenario, UiData, short_hash, verify_evidence_json};
use dioxus::prelude::*;

#[component]
pub fn HomePage(data: UiData) -> Element {
    let head = data.head();
    rsx! {
        div { class: "page page-home",
            section { class: "home-hero",
                div { class: "home-hero-copy",
                    p { class: "eyebrow", "INDEPENDENT AUDIT SURFACE / PRIVATE AGENT STATE" }
                    h1 { "Verify the history, ", em { "not the memory." } }
                    p { class: "home-lede", "MemoryLineage checks whether a committed private-agent state is the authorized, canonical continuation of the history before it. Raw memory stays off-chain." }
                    div { class: "action-row",
                        a { class: "button button-primary", href: "/lab/silent-rollback", "Run Silent Rollback →" }
                        a { class: "button button-secondary", href: "/verify", "Verify Evidence" }
                    }
                }
                div { class: "home-hero-trust content-panel",
                    div { class: "panel-kicker", "TRUSTED INFRASTRUCTURE FOR PRIVATE AI" }
                    div { class: "trust-list",
                        div { class: "trust-item", span { class: "trust-icon", "⌑" }, div { strong { "Private by design" }, span { "Raw memory never goes on-chain" } } }
                        div { class: "trust-item", span { class: "trust-icon", "▣" }, div { strong { "Verifiable history" }, span { "Cryptographic lineage and portable evidence" } } }
                        div { class: "trust-item", span { class: "trust-icon", "♧" }, div { strong { "Authority controlled" }, span { "Rotating authorizers, on-chain governance" } } }
                        div { class: "trust-item", span { class: "trust-icon", "◇" }, div { strong { "Open and inspectable" }, span { "Rust/WASM, Solidity, and independent replay" } } }
                    }
                }
            }
            section { class: "home-metrics section-wrap",
                div { class: "home-metric", span { class: "metric-icon", "◎" }, div { strong { "{data.fixture.snapshots.len()}" }, span { "private snapshots in Silent Rollback fixture" } } }
                div { class: "home-metric", span { class: "metric-icon", "◇" }, div { strong { "{data.mutation_rejected}/{data.mutation_count}" }, span { "published mutation rejections" } } }
                div { class: "home-metric", span { class: "metric-icon", "◆" }, div { strong { "{data.bundle.valid_history.len()}" }, span { "states in protocol replay corpus" } } }
            }
            section { class: "home-overview section-wrap content-panel",
                div { class: "panel-title-row home-overview-heading",
                    div { p { class: "panel-kicker", "CANONICAL OVERVIEW" } h2 { "Committed private-memory lineage" } p { class: "panel-subtitle", "Latest committed state is shown on the right; the raw memory remains private." } }
                    SourceLine { source: "PRIVATE FIXTURE".to_owned(), note: format!("{} snapshots / commitments only", data.fixture.snapshots.len()) }
                    SourceLine { source: "PROTOCOL CORPUS".to_owned(), note: format!("{} transitions / Rust verifier", data.bundle.valid_history.len()) }
                }
                div { class: "home-overview-layout",
                    div { class: "home-overview-lineage",
                        LineageRail { data: data.clone(), compact: false }
                        a { class: "text-link", href: "/history", "View full history →" }
                    }
                    aside { class: "home-selected-state",
                        div { class: "panel-title-row", div { p { class: "panel-kicker", "SELECTED SNAPSHOT" } h2 { "State {head.delta.sequence}" } }, StatusBadge { label: "CURRENT / CANONICAL".to_owned(), tone: "verified".to_owned() } }
                        CopyValue { label: "state root".to_owned(), value: head.next_state_root.clone(), compact: false }
                        dl { class: "readout-list home-selected-readout",
                            ReadoutRow { label: "Status".to_owned(), value: "canonical".to_owned(), detail: "published replay head".to_owned() }
                            ReadoutRow { label: "Registry".to_owned(), value: short_hash(&data.deployment.registry_address, 12, 8), detail: "Ethereum Sepolia".to_owned() }
                            ReadoutRow { label: "Sequence".to_owned(), value: head.delta.sequence.to_string(), detail: "ordered transition".to_owned() }
                        }
                        a { class: "button button-secondary button-full", href: "/inspect", "View on Inspect →" }
                    }
                }
            }
            section { class: "section-wrap home-columns",
                div { class: "content-panel scope-panel-positive",
                    div { class: "panel-kicker", "WHAT MEMORYLINEAGE VERIFIES" }
                    h2 { "Continuity, authority, commitments." }
                    ul { class: "scope-list scope-list-positive",
                        li { strong { "Ordered history" }, span { "Sequence values extend one another without gaps." } }
                        li { strong { "Predecessor binding" }, span { "Each transition names the current committed root." } }
                        li { strong { "Authorization" }, span { "Configured registry rules are replayable." } }
                        li { strong { "Portable evidence" }, span { "A separate Rust verifier can replay the bundle." } }
                    }
                }
                div { class: "content-panel content-panel-muted scope-panel-negative",
                    div { class: "panel-kicker", "WHAT MEMORYLINEAGE DOES NOT VERIFY" }
                    h2 { "A valid lineage is not semantic truth." }
                    ul { class: "scope-list scope-list-muted",
                        li { strong { "Not memory truth" }, span { "The underlying private content is not semantically evaluated." } }
                        li { strong { "Not AI reasoning" }, span { "The protocol does not explain why an agent made a decision." } }
                        li { strong { "Not total safety" }, span { "Semantic poisoning remains outside this audit boundary." } }
                    }
                }
            }
            section { class: "section-wrap how-strip",
                div { class: "panel-kicker", "HOW IT WORKS" }
                div { class: "how-flow",
                    span { "Private memory" } span { class: "flow-arrow", "→" }
                    span { "Rust commitments" } span { class: "flow-arrow", "→" }
                    span { "Solidity registry" } span { class: "flow-arrow", "→" }
                    span { "Independent replay" }
                }
                a { class: "text-link", href: "/architecture", "Read the architecture →" }
            }
        }
    }
}

#[component]
pub fn InspectPage(data: UiData) -> Element {
    let head = data.head();
    let first = data
        .bundle
        .valid_history
        .first()
        .expect("history is non-empty");
    let history_count = data.bundle.authority_history.len().saturating_sub(1);
    rsx! {
        div { class: "page workspace-page",
            PageHeader {
                kicker: "INSPECT / MEMORY SPACE".to_owned(),
                title: "What is canonical right now?".to_owned(),
                description: "Read the committed head, its authority, and the evidence source without exposing the private memory behind the commitments.".to_owned(),
                source: "PUBLISHED EVIDENCE".to_owned(),
            }
            div { class: "workspace-grid workspace-grid-inspect",
                section { class: "content-panel primary-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "PUBLIC REGISTRY / READOUT" } h2 { "Canonical memory space" } }, StatusBadge { label: "OBSERVED".to_owned(), tone: "observed".to_owned() } }
                    div { class: "readout-grid",
                        dl { class: "readout-list",
                            ReadoutRow { label: "Network".to_owned(), value: format!("Ethereum Sepolia / chain {}", data.deployment.chain_id), detail: "read-only trust anchor".to_owned() }
                            ReadoutRow { label: "Registry".to_owned(), value: short_hash(&data.deployment.registry_address, 12, 8), detail: "workspace-owned deployment".to_owned() }
                            ReadoutRow { label: "Space ID".to_owned(), value: short_hash(&data.deployment.space_id, 14, 8), detail: "Sepolia registration".to_owned() }
                            ReadoutRow { label: "Protocol replay head".to_owned(), value: format!("sequence {}", head.delta.sequence), detail: "local EVM evidence / chain 31337".to_owned() }
                            ReadoutRow { label: "Private fixture".to_owned(), value: format!("{} snapshots", data.fixture.snapshots.len()), detail: "raw SQLite remains local".to_owned() }
                        }
                        dl { class: "readout-list",
                            ReadoutRow { label: "State root".to_owned(), value: short_hash(&head.next_state_root, 14, 8), detail: "deterministically derived".to_owned() }
                            ReadoutRow { label: "Transition ID".to_owned(), value: short_hash(&head.transition_id, 14, 8), detail: "commitment-bound".to_owned() }
                            ReadoutRow { label: "First committed space".to_owned(), value: short_hash(&first.delta.space_id, 14, 8), detail: "published replay bundle".to_owned() }
                            ReadoutRow { label: "Authority changes".to_owned(), value: history_count.to_string(), detail: "configNonce trace available".to_owned() }
                        }
                    }
                    div { class: "action-row action-row-tight",
                        a { class: "button button-primary", href: "/lab/silent-rollback", "Simulate Rollback" }
                        a { class: "button button-secondary", href: "/history", "View History" }
                        a { class: "button button-quiet", href: "/verify", "Verify Bundle" }
                    }
                }
                aside { class: "content-panel boundary-panel-light",
                    div { class: "panel-kicker", "EVIDENCE SOURCE" }
                    h2 { "Two lanes, explicitly labeled." }
                    p { "Sepolia is the public Ethereum anchor. The detailed four-transition replay shown here is the published local evidence bundle." }
                    SourceLine { source: "SEPOLIA / OBSERVED".to_owned(), note: format!("block {} / reread PASS", data.reread.block_number) }
                    SourceLine { source: "PRIVATE FIXTURE".to_owned(), note: format!("{} snapshots / commitments only", data.fixture.snapshots.len()) }
                    SourceLine { source: "PROTOCOL CORPUS".to_owned(), note: format!("{} transitions / Rust replay", data.bundle.valid_history.len()) }
                    div { class: "mini-divider" }
                    div { class: "panel-kicker", "PRIVACY BOUNDARY" }
                    div { class: "boundary-columns",
                        div { strong { "ON CHAIN" } span { "sequence" } span { "state roots" } span { "commitments" } }
                        div { strong { "PRIVATE" } span { "raw memory" } span { "documents" } span { "locator contents" } }
                    }
                }
            }
            section { class: "section-wrap compact-section",
                div { class: "panel-title-row", div { p { class: "panel-kicker", "CANONICAL CUE RAIL" } h2 { "Committed succession" } }, StatusBadge { label: format!("{} / {} VERIFIED", data.bundle.valid_history.len(), data.bundle.valid_history.len()), tone: "verified".to_owned() } }
                LineageRail { data: data.clone(), compact: false }
            }
            section { class: "section-wrap inspect-bottom-grid",
                div { class: "content-panel",
                    div { class: "panel-kicker", "VERIFIED SURFACE" }
                    h3 { "The registry can answer" }
                    ul { class: "scope-list scope-list-positive", li { "Which sequence is current?" }, li { "Which predecessor root was accepted?" }, li { "Which authority configuration was active?" } }
                }
                div { class: "content-panel content-panel-muted",
                    div { class: "panel-kicker", "OUT OF SCOPE" }
                    h3 { "The registry cannot answer" }
                    ul { class: "scope-list scope-list-muted", li { "Whether private content is semantically true." }, li { "Whether the agent's reasoning was correct." }, li { "Whether an off-chain locator remains available." } }
                }
            }
        }
    }
}

#[component]
pub fn HistoryPage(data: UiData) -> Element {
    let head = data.head();
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "HISTORY / CANONICAL LINEAGE".to_owned(), title: "How did the canonical history get here?".to_owned(), description: "Review the ordered transitions, authority changes, and final root as one evidence-backed timeline.".to_owned(), source: "PUBLISHED LOCAL REPLAY".to_owned() }
            div { class: "history-workspace",
                section { class: "content-panel history-main-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "CANONICAL LINEAGE RAIL" } h2 { "Sequence of record" } }, StatusBadge { label: "VERIFIED".to_owned(), tone: "verified".to_owned() } }
                    div { class: "history-rail-list",
                        for transition in data.bundle.valid_history.iter() {
                            article { class: "history-row", key: "{transition.transition_id}",
                                div { class: "history-sequence", strong { "{transition.delta.sequence:02}" }, span { "SEQ" } }
                                div { class: "history-event",
                                    div { class: "event-title-row", h3 { "Committed transition {transition.delta.sequence}" }, StatusBadge { label: "COMMITTED".to_owned(), tone: "verified".to_owned() } }
                                    p { "The transition extends the previous committed state root under the pinned registry rules." }
                                    div { class: "history-meta", span { "PREV ", code { "{short_hash(&transition.delta.prev_state_root, 10, 6)}" } }, span { "NEXT ", code { "{short_hash(&transition.next_state_root, 10, 6)}" } }, a { href: "/history/{transition.delta.sequence}", "View detail →" } }
                                }
                            }
                        }
                    }
                    div { class: "source-note", "Raw memory, provenance content, and locator content are intentionally absent from this public bundle." }
                }
                aside { class: "history-sidebar",
                    div { class: "content-panel selected-state-panel",
                        div { class: "panel-kicker", "SELECTED CURRENT STATE" }
                        h2 { "Sequence {head.delta.sequence}" }
                        StatusBadge { label: "CANONICAL HEAD".to_owned(), tone: "observed".to_owned() }
                        div { class: "sidebar-readout", span { "STATE ROOT" }, CopyValue { label: "current state root".to_owned(), value: head.next_state_root.clone(), compact: false } }
                        div { class: "sidebar-readout", span { "TRANSITION ID" }, CopyValue { label: "current transition id".to_owned(), value: head.transition_id.clone(), compact: true } }
                        a { class: "text-link", href: "/history/{head.delta.sequence}", "Open transition detail →" }
                    }
                    div { class: "content-panel",
                        div { class: "panel-kicker", "AUTHORITY HISTORY" }
                        for authority in data.bundle.authority_history.iter() {
                            div { class: "authority-row", key: "{authority.config_nonce}-{authority.authorizer}", span { class: "authority-index", "{authority.config_nonce}" }, div { strong { "{authority.label.as_deref().unwrap_or(\"authority\")}" }, code { "{short_hash(&authority.authorizer, 10, 6)}" } } }
                        }
                        p { class: "small-note", "Authority rotation is represented by configNonce and must be considered when replaying authorization. This history belongs to the protocol test corpus." }
                    }
                }
            }
            section { class: "section-wrap stats-row",
                div { class: "metric-panel", strong { "{data.bundle.valid_history.len()}" }, span { "ordered transitions" } }
                div { class: "metric-panel", strong { "{data.bundle.authority_history.len()}" }, span { "authority records" } }
                div { class: "metric-panel", strong { "{data.mutation_rejected}/{data.mutation_count}" }, span { "rejected mutations" } }
                div { class: "metric-panel", strong { "MATCH" }, span { "head reconstructed" } }
            }
        }
    }
}

#[component]
pub fn TransitionPage(data: UiData, sequence: u64) -> Element {
    let transition = data.transition(sequence);
    let report = data.report.as_ref();
    let transition_check = if report.is_some() {
        "MATCH"
    } else {
        "NOT AVAILABLE"
    };
    let transition_tone = if report.is_some() {
        "verified"
    } else {
        "neutral"
    };
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "HISTORY / TRANSITION FORENSICS".to_owned(), title: "What exactly happened in this transition?".to_owned(), description: format!("Inspect the commitment fields and derived state for sequence {} without opening the private memory.", transition.delta.sequence), source: "PUBLISHED EVIDENCE".to_owned() }
            div { class: "detail-hero-row",
                div { class: "content-panel detail-identity",
                    div { class: "panel-kicker", "TRANSITION" }
                    h2 { "Sequence {transition.delta.sequence}" }
                    StatusBadge { label: "COMMITTED".to_owned(), tone: "verified".to_owned() }
                    div { class: "detail-identity-grid",
                        div { span { "PREVIOUS STATE ROOT" }, CopyValue { label: "previous state root".to_owned(), value: transition.delta.prev_state_root.clone(), compact: false } }
                        div { span { "NEXT STATE ROOT" }, CopyValue { label: "next state root".to_owned(), value: transition.next_state_root.clone(), compact: false } }
                        div { span { "TRANSITION ID" }, CopyValue { label: "transition id".to_owned(), value: transition.transition_id.clone(), compact: false } }
                    }
                }
                aside { class: "content-panel detail-check-panel",
                    div { class: "panel-kicker", "REPLAY CHECKS" }
                    CheckRow { label: "Sequence continuity".to_owned(), value: "PASS".to_owned(), tone: "verified".to_owned() }
                    CheckRow { label: "Predecessor continuity".to_owned(), value: "PASS".to_owned(), tone: "verified".to_owned() }
                    CheckRow { label: "Transition ID".to_owned(), value: transition_check.to_owned(), tone: transition_tone.to_owned() }
                    CheckRow { label: "Raw memory".to_owned(), value: "OFF-CHAIN".to_owned(), tone: "observed".to_owned() }
                }
            }
            section { class: "section-wrap detail-grid",
                div { class: "content-panel",
                    div { class: "panel-kicker", "COMMITMENT FIELDS" }
                    div { class: "commitment-table",
                        CommitmentRow { label: "deltaCommitment".to_owned(), value: transition.delta.delta_commitment.clone() }
                        CommitmentRow { label: "provenanceCommitment".to_owned(), value: transition.delta.provenance_commitment.clone() }
                        CommitmentRow { label: "locatorCommitment".to_owned(), value: transition.delta.locator_commitment.clone() }
                        CommitmentRow { label: "profileId".to_owned(), value: transition.delta.profile_id.clone() }
                    }
                }
                div { class: "content-panel",
                    div { class: "panel-kicker", "REGISTRY CONTEXT" }
                    dl { class: "readout-list",
                        ReadoutRow { label: "Space ID".to_owned(), value: short_hash(&transition.delta.space_id, 14, 8), detail: "public identifier".to_owned() }
                        ReadoutRow { label: "Sequence".to_owned(), value: transition.delta.sequence.to_string(), detail: "uint64 / canonical order".to_owned() }
                        ReadoutRow { label: "Block / transaction".to_owned(), value: "NOT AVAILABLE IN THIS BUNDLE".to_owned(), detail: "no fabricated observation".to_owned() }
                        ReadoutRow { label: "Authority".to_owned(), value: "SEE AUTHORITY HISTORY".to_owned(), detail: "transition-level signature not exported".to_owned() }
                    }
                }
            }
            section { class: "section-wrap detail-footer-row", a { class: "button button-secondary", href: "/history", "← Back to History" }, a { class: "button button-primary", href: "/lab/silent-rollback", "Test a stale predecessor →" } }
        }
    }
}

#[component]
pub fn LabPage(data: UiData, initial_scenario: Scenario) -> Element {
    let mut selected = use_signal(|| initial_scenario);
    let mut attempted = use_signal(|| false);
    let busy = use_signal(|| false);
    let detail = use_signal(|| "PUBLISHED EVIDENCE / BAD_PREVIOUS_STATE".to_owned());
    let selected_scenario = selected();
    let was_attempted = attempted();
    let is_busy = busy();
    let result_detail = detail();
    let stale_predecessor = data.restored_snapshot().snapshot_commitment.clone();
    let run_rollback = {
        let mut attempted = attempted;
        let mut busy = busy;
        let mut detail = detail;
        let stale_predecessor = stale_predecessor.clone();
        move |_| {
            attempted.set(true);
            busy.set(true);
            let stale_predecessor = stale_predecessor.clone();
            spawn(async move {
                match live_silent_rollback(&stale_predecessor).await {
                    Ok(value) => detail.set(value),
                    Err(reason) => detail.set(format!("LIVE RPC UNAVAILABLE / USING PUBLISHED EVIDENCE / BAD_PREVIOUS_STATE ({reason})")),
                }
                busy.set(false);
            });
        }
    };
    let selected_mutation = selected_scenario.corpus_name().and_then(|name| {
        data.bundle
            .mutation_matrix
            .iter()
            .find(|mutation| mutation.name == name)
    });
    let page_source = if selected_scenario == Scenario::SilentRollback && was_attempted {
        "LIVE OR FALLBACK"
    } else {
        "PUBLISHED CORPUS"
    };
    let result_tone = if selected_scenario == Scenario::SemanticPoisoning {
        "warning"
    } else {
        "danger"
    };
    let result_label = if selected_scenario == Scenario::SemanticPoisoning {
        "OUT OF SCOPE"
    } else {
        "REJECT EXPECTED"
    };
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "TAMPERING LAB / FALSIFICATION WORKSPACE".to_owned(), title: "Can I break the committed history?".to_owned(), description: "Run the smallest useful attack against the actual contract behavior or inspect the published mutation corpus. Every result keeps its machine reason.".to_owned(), source: page_source.to_owned() }
            div { class: "lab-workspace",
                aside { class: "scenario-list content-panel",
                    div { class: "panel-kicker", "ATTACK SCENARIOS" }
                    for scenario in Scenario::ALL {
                        button { class: if scenario == selected_scenario { "scenario-item scenario-item-active" } else { "scenario-item" }, aria_pressed: scenario == selected_scenario, onclick: move |_| selected.set(scenario),
                            span { class: "scenario-index", "{scenario.evidence_kind()}" }
                            span { strong { "{scenario.title()}" }, small { "{scenario.description()}" } }
                            span { class: "scenario-chevron", "→" }
                        }
                    }
                }
                section { class: "attack-workspace content-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "CANONICAL VS ATTEMPT" } h2 { "{selected_scenario.title()}" } }, StatusBadge { label: result_label.to_owned(), tone: result_tone.to_owned() } }
                    if selected_scenario == Scenario::SilentRollback {
                        div { class: "rollback-story",
                            div { class: "rollback-column", span { class: "panel-kicker", "PRIVATE FIXTURE / CANONICAL SNAPSHOT" }, div { class: "snapshot-stack", for snapshot in data.fixture.snapshots.iter() { if snapshot.sequence == data.canonical_snapshot().sequence { strong { "SNAPSHOT {snapshot.sequence} / COMMITTED" } } else { span { "SNAPSHOT {snapshot.sequence}" } } if snapshot.sequence != data.canonical_snapshot().sequence { span { "↓" } } } }, code { "protocol root {short_hash(&data.head().next_state_root, 12, 8)}" } }
                            div { class: "rollback-operator", "→" }
                            div { class: "rollback-column rollback-column-attempt", span { class: "panel-kicker", "LOCAL RESTORE ATTEMPT" }, div { class: "snapshot-stack snapshot-stack-danger", strong { "RESTORE SNAPSHOT {data.restored_snapshot().sequence}" }, span { "↓" }, span { "TRY NEXT SEQUENCE" } }, code { "stale snapshot commitment {short_hash(&data.restored_snapshot().snapshot_commitment, 12, 8)}" } }
                        }
                        div { class: if was_attempted { "result-panel result-panel-rejected" } else { "result-panel" },
                            if was_attempted {
                                div { class: "result-heading", span { class: "result-symbol", "×" }, div { strong { "REJECTED" }, small { "{result_detail}" } } }
                                h3 { "The restored snapshot cannot replace the committed history." }
                                p { "The local memory snapshot changed. The registry's canonical head did not. The stale predecessor was derived from the private fixture commitment, while the exact contract reason is preserved above." }
                                button { class: "button button-quiet", onclick: move |_| attempted.set(false), "Reset rehearsal" }
                            } else {
                                div { class: "result-heading result-heading-ready", span { class: "result-symbol", "◉" }, div { strong { "READY TO REHEARSE" }, small { "Browser will try read-only Sepolia eth_call first." } } }
                                h3 { "Restore a stale snapshot, then ask the registry to accept it." }
                                p { "No transaction is broadcast and no gas is spent. A failed RPC is shown as published evidence, never as a fake live result." }
                                button { class: "button button-danger", onclick: run_rollback, disabled: is_busy, if is_busy { "Reading Sepolia…" } else { "Run Silent Rollback" } }
                            }
                        }
                    } else if selected_scenario == Scenario::SemanticPoisoning {
                        div { class: "scope-result",
                            StatusBadge { label: "OUT OF SCOPE".to_owned(), tone: "warning".to_owned() }
                            h3 { "A valid lineage can still contain unsafe content." }
                            p { "If malicious private text is inserted through an otherwise valid, ordered, authorized transition, MemoryLineage reports the lineage as structurally valid. It does not evaluate semantic truth or safety." }
                            div { class: "scope-example", span { "LINEAGE" }, strong { "VALID" }, span { "SEMANTIC SAFETY" }, strong { "NOT EVALUATED" } }
                        }
                    } else if let Some(mutation) = selected_mutation {
                        div { class: "result-panel result-panel-rejected",
                            div { class: "result-heading", span { class: "result-symbol", "×" }, div { strong { "REJECTED" }, small { "PUBLISHED CORPUS / {mutation.observed.reason.as_deref().unwrap_or(\"REASON NOT RECORDED\")}" } } }
                            h3 { "The selected mutation is rejected by the pinned contract lane." }
                            p { "This browser surface does not broadcast the attack. It exposes the published execution result for independent replay." }
                            dl { class: "result-readout", ReadoutRow { label: "Mutation".to_owned(), value: mutation.name.clone(), detail: "published local EVM corpus".to_owned() }, ReadoutRow { label: "Expected reason".to_owned(), value: mutation.observed.reason.clone().unwrap_or_else(|| "NOT RECORDED".to_owned()), detail: "exact machine output".to_owned() }, ReadoutRow { label: "Observed status".to_owned(), value: mutation.observed.status.clone(), detail: format!("receipt status {}", mutation.observed.receipt_status) } }
                        }
                    } else {
                        div { class: "result-panel result-panel-warning", StatusBadge { label: "NOT YET DEMONSTRATED".to_owned(), tone: "warning".to_owned() }, h3 { "No matching published mutation record." }, p { "The Inspector fails closed when a selected scenario has no evidence." } }
                    }
                }
            }
            section { class: "section-wrap lab-footer-grid",
                div { class: "content-panel", div { class: "panel-kicker", "EXACT REASONS" }, div { class: "reason-grid", for mutation in data.bundle.mutation_matrix.iter().filter(|mutation| mutation.expected == "REJECT").take(8) { div { class: "reason-cell", key: "{mutation.name}", code { "{mutation.observed.reason.as_deref().unwrap_or(\"NO_REASON\")}" }, span { "{mutation.name}" } } } } }
                div { class: "content-panel content-panel-muted", div { class: "panel-kicker", "NEXT STEP" }, h3 { "Take the evidence with you." }, p { "A browser rejection is useful only when the bundle can be independently replayed." }, a { class: "text-link", href: "/verify", "Open the Verify workspace →" } }
            }
        }
    }
}

#[component]
pub fn VerifyPage(data: UiData) -> Element {
    let default_json = data.evidence_json();
    let tampered_json = data.tampered_json();
    let report = use_signal(|| data.report.clone());
    let error = use_signal(|| None::<String>);
    let source = use_signal(|| "PUBLISHED V2 EVIDENCE".to_owned());
    let imported_name = use_signal(|| None::<String>);
    let announcement =
        use_signal(|| "Published evidence is loaded in the Rust/WASM verifier.".to_owned());
    let current_report = report();
    let current_error = error();
    let current_source = source();
    let current_file = imported_name();
    let is_verified = current_error.is_none() && current_report.is_some();
    let verification_summary = if let Some(reason) = current_error.as_deref() {
        format!("Independent Rust verification rejected the bundle: {reason}.")
    } else if let Some(current_report) = current_report.as_ref() {
        format!(
            "{} transitions replayed; final root {}.",
            current_report.transition_count,
            short_hash(&current_report.final_root, 16, 12)
        )
    } else {
        "No verification result is available.".to_owned()
    };
    let export_action = {
        let export_json = default_json.clone();
        let mut report = report;
        let mut error = error;
        let mut source = source;
        let mut imported_name = imported_name;
        let mut announcement = announcement;
        move |_| {
            let result = download_json("memorylineage-evidence-v2.json", &export_json);
            report.set(verify_evidence_json(&export_json).ok());
            error.set(None);
            source.set("EXPORTED V2 EVIDENCE".to_owned());
            imported_name.set(None);
            announcement.set(match result {
                Ok(()) => "Evidence exported; the same bundle is verified in Rust/WASM.".to_owned(),
                Err(reason) => {
                    format!("Evidence is ready; browser download unavailable: {reason}.")
                }
            });
        }
    };
    let tamper_action = {
        let tampered_json = tampered_json.clone();
        let mut report = report;
        let mut error = error;
        let mut source = source;
        let mut imported_name = imported_name;
        let mut announcement = announcement;
        move |_| match verify_evidence_json(&tampered_json) {
            Ok(value) => {
                report.set(Some(value));
                error.set(None);
                announcement.set("Tamper check unexpectedly passed.".to_owned());
            }
            Err(reason) => {
                report.set(None);
                error.set(Some(reason.clone()));
                source.set("TAMPERED COPY".to_owned());
                imported_name.set(None);
                announcement.set(format!("Tamper check: REJECTED / {reason}"));
            }
        }
    };
    let restore_action = {
        let data_report = data.report.clone();
        let mut report = report;
        let mut error = error;
        let mut source = source;
        let mut announcement = announcement;
        move |_| {
            report.set(data_report.clone());
            error.set(None);
            source.set("PUBLISHED V2 EVIDENCE".to_owned());
            announcement.set("Original evidence restored and verified.".to_owned());
        }
    };
    let import_action = {
        let mut report = report;
        let mut error = error;
        let mut source = source;
        let mut imported_name = imported_name;
        let mut announcement = announcement;
        move |event: FormEvent| {
            if let Some(file) = event.files().into_iter().next() {
                let name = file.name();
                imported_name.set(Some(name.clone()));
                spawn(async move {
                    match file.read_string().await {
                        Ok(json) => match verify_evidence_json(&json) {
                            Ok(value) => {
                                report.set(Some(value));
                                error.set(None);
                                source.set("IMPORTED EVIDENCE".to_owned());
                                announcement.set(format!("Imported {name}: VERIFIED."));
                            }
                            Err(reason) => {
                                report.set(None);
                                error.set(Some(reason.clone()));
                                source.set("IMPORTED EVIDENCE".to_owned());
                                announcement.set(format!("Imported {name}: REJECTED / {reason}"));
                            }
                        },
                        Err(_) => {
                            report.set(None);
                            error.set(Some("FILE_READ_FAILED".to_owned()));
                            announcement.set(format!("Imported {name}: file read failed."));
                        }
                    }
                });
            }
        }
    };
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "VERIFY / PORTABLE EVIDENCE".to_owned(), title: "Can I verify this without trusting the website?".to_owned(), description: "Load, export, tamper, and restore a public evidence bundle. The browser uses the independent Rust verifier compiled to WASM.".to_owned(), source: current_source.clone() }
            div { class: "verify-workspace",
                aside { class: "content-panel evidence-actions-panel",
                    div { class: "panel-kicker", "EVIDENCE WORKBENCH" }
                    h2 { "Take the evidence with you." }
                    p { "The bundle contains sequence, predecessor roots, commitments, authority records, and the observed head. It never contains raw memory." }
                    div { class: "evidence-facts", div { span { "TRANSITIONS" }, strong { "{data.v2.transitions.len()}" } }, div { span { "SCHEMA" }, strong { "V2" } }, div { span { "RAW MEMORY" }, strong { "OFF-CHAIN" } } }
                    button { class: "button button-primary button-wide", onclick: export_action, "Export evidence.json" }
                    label { class: "file-drop", strong { "Import evidence.json" }, span { "V1 or V2 public evidence bundle" }, input { r#type: "file", accept: "application/json,.json", onchange: import_action } }
                    div { class: "source-line source-line-vertical", span { class: "source-dot source-dot-blue" }, strong { "{current_source}" }, if let Some(name) = current_file.as_deref() { span { "{name}" } } }
                }
                section { class: "content-panel verification-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "VERIFICATION SUMMARY" }, h2 { "Independent replay" } }, StatusBadge { label: if is_verified { "VERIFIED".to_owned() } else { "REJECTED".to_owned() }, tone: if is_verified { "verified".to_owned() } else { "danger".to_owned() } } }
                    div { class: if is_verified { "verification-banner verification-banner-pass" } else { "verification-banner verification-banner-fail" }, aria_live: "polite", span { class: "verification-icon", if is_verified { "✓" } else { "×" } }, div { strong { if is_verified { "VERIFIED" } else { "REJECTED" } }, p { "{verification_summary}" } } }
                    div { class: "check-table",
                        if let Some(current_report) = current_report.as_ref() {
                            CheckRow { label: "Schema".to_owned(), value: current_report.schema.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Registry identity".to_owned(), value: current_report.registry_identity.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Transition IDs".to_owned(), value: current_report.transition_ids.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "State roots".to_owned(), value: current_report.state_roots.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Sequence continuity".to_owned(), value: current_report.sequence_continuity.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Predecessor continuity".to_owned(), value: current_report.predecessor_continuity.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Authority history".to_owned(), value: current_report.authority_history.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Privacy boundary".to_owned(), value: current_report.privacy_boundary.clone(), tone: "verified".to_owned() }
                        } else { CheckRow { label: "Imported bundle".to_owned(), value: current_error.clone().unwrap_or_else(|| "NO RESULT".to_owned()), tone: "danger".to_owned() } }
                    }
                }
            }
            section { class: "section-wrap tamper-workbench",
                div { class: "panel-title-row", div { p { class: "panel-kicker", "TAMPER WORKBENCH" }, h2 { "Change one commitment." } }, StatusBadge { label: "FALSIFIABLE".to_owned(), tone: "warning".to_owned() } }
                p { "The browser changes the first locatorCommitment in a copy of the loaded evidence. A valid bundle must fail closed before it can be trusted." }
                div { class: "tamper-grid", div { class: "tamper-value", span { "ORIGINAL" }, code { "locatorCommitment / {short_hash(&data.bundle.valid_history[0].delta.locator_commitment, 14, 8)}" } }, div { class: "tamper-arrow", "→" }, div { class: "tamper-value tamper-value-danger", span { "TAMPERED COPY" }, code { "locatorCommitment / 0xffff…ffff" } } }
                div { class: "action-row", button { class: "button button-danger", onclick: tamper_action, "Tamper one field" }, button { class: "button button-secondary", onclick: restore_action, "Restore original" } }
                p { class: "announcement", "{announcement}" }
            }
            section { class: "section-wrap verify-bottom-grid",
                div { class: "content-panel", div { class: "panel-kicker", "SECONDARY AUDITOR" }, h3 { "Rust CLI" }, p { "The same evidence can be checked outside the browser with the independent Rust verifier." }, code { class: "command-block", "cargo run -q -p ml-cli -- verify evidence.json" } }
                div { class: "content-panel content-panel-muted", div { class: "panel-kicker", "TRUST MODEL" }, h3 { "Do not trust the dashboard." }, p { "Trust the deterministic bundle, the pinned semantics, and a verifier you can run separately." }, a { class: "text-link", href: "/security", "Read the boundary →" } }
            }
        }
    }
}

#[component]
pub fn EvidencePage(data: UiData) -> Element {
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "EVIDENCE / PUBLIC RECORD".to_owned(), title: "Where is the proof behind the claims?".to_owned(), description: "The public evidence page collects deployment observations, deterministic conformance, mutation results, and the artifacts available for independent replay.".to_owned(), source: "REPOSITORY EVIDENCE".to_owned() }
            section { class: "section-wrap evidence-grid",
                div { class: "content-panel evidence-deployment-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "SEPOLIA DEPLOYMENT" }, h2 { "Workspace-owned trust anchor" } }, StatusBadge { label: "PASS".to_owned(), tone: "verified".to_owned() } }
                    dl { class: "readout-list", ReadoutRow { label: "Network".to_owned(), value: format!("Ethereum Sepolia / {}", data.deployment.chain_id), detail: "read-only browser target".to_owned() }, ReadoutRow { label: "Registry".to_owned(), value: short_hash(&data.deployment.registry_address, 14, 8), detail: "contract address".to_owned() }, ReadoutRow { label: "Deployment tx".to_owned(), value: short_hash(&data.deployment.deployment_tx_hash, 14, 8), detail: "receipt recorded".to_owned() }, ReadoutRow { label: "Code hash".to_owned(), value: short_hash(&data.deployment.deployed_code_hash, 14, 8), detail: "deployed bytecode observation".to_owned() }, ReadoutRow { label: "Raw payload".to_owned(), value: if data.deployment.raw_payload_stored { "PRESENT".to_owned() } else { "NOT STORED".to_owned() }, detail: "fixed-size commitments only".to_owned() } }
                }
                div { class: "content-panel evidence-observation-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "CURRENT OBSERVATION" }, h2 { "Second RPC reread" } }, StatusBadge { label: data.reread.verdict.clone(), tone: "verified".to_owned() } }
                    dl { class: "readout-list", ReadoutRow { label: "Provider".to_owned(), value: data.reread.rpc_url.clone(), detail: "published observation".to_owned() }, ReadoutRow { label: "Block".to_owned(), value: data.reread.block_number.to_string(), detail: "observed head block".to_owned() }, ReadoutRow { label: "Observed sequence".to_owned(), value: data.reread.head.sequence.to_string(), detail: "Sepolia reread".to_owned() }, ReadoutRow { label: "Head root".to_owned(), value: short_hash(&data.reread.head.state_root, 14, 8), detail: "headMatches = true".to_owned() } }
                    p { class: "small-note", "This page does not claim provider consensus; it labels the independent reread that is actually published." }
                }
            }
            section { class: "section-wrap content-panel",
                div { class: "panel-title-row", div { p { class: "panel-kicker", "CONFORMANCE MATRIX" }, h2 { "Independent paths agree on the published corpus" } }, StatusBadge { label: "MATCH".to_owned(), tone: "verified".to_owned() } }
                div { class: "evidence-table",
                    EvidenceTableRow { label: "Pinned vector".to_owned(), value: data.conformance.pinned_vector.clone(), note: "contracts/vectors/erc8350_conformance.json".to_owned() }
                    EvidenceTableRow { label: "Published evidence".to_owned(), value: data.conformance.published_evidence.clone(), note: "Rust independent replay".to_owned() }
                    EvidenceTableRow { label: "Rust reference".to_owned(), value: data.conformance.rust_reference.clone(), note: "ml-core".to_owned() }
                    EvidenceTableRow { label: "Independent verifier".to_owned(), value: data.conformance.independent_verifier.clone(), note: "separate algorithm path".to_owned() }
                    EvidenceTableRow { label: "Rust/revm Silent Rollback".to_owned(), value: data.conformance.revm_silent_rollback.clone(), note: "actual Solidity bytecode lane".to_owned() }
                    EvidenceTableRow { label: "Rust/revm mutation lane".to_owned(), value: data.conformance.revm_core_mutations.clone(), note: "core mutation matrix".to_owned() }
                    EvidenceTableRow { label: "ERC-1271".to_owned(), value: data.conformance.revm_erc1271.clone(), note: "acceptance then invalid signature".to_owned() }
                    EvidenceTableRow { label: "Authority rotation".to_owned(), value: data.conformance.revm_authority_rotation.clone(), note: "configNonce and old/new authorizer".to_owned() }
                }
            }
            section { class: "section-wrap evidence-lower-grid",
                div { class: "content-panel", div { class: "panel-kicker", "DEMO FIXTURE" }, h3 { "Private snapshots remain separate from the protocol corpus." }, p { "The local Silent Rollback fixture contains {data.fixture.snapshots.len()} SQLite snapshots. Only their deterministic commitments are surfaced here; raw values are never bundled into public evidence." }, div { class: "reason-grid", for snapshot in data.fixture.snapshots.iter() { div { class: "reason-cell", key: "fixture-{snapshot.sequence}", code { "SNAPSHOT {snapshot.sequence}" }, span { "{snapshot.visible_label.as_deref().unwrap_or(\"private state\")}" } } } } }
                div { class: "content-panel", div { class: "panel-kicker", "PUBLISHED ADVERSARIAL CORPUS" }, h3 { "{data.mutation_rejected}/{data.mutation_count} expected rejection cases observed" }, div { class: "reason-grid reason-grid-wide", for mutation in data.bundle.mutation_matrix.iter().filter(|mutation| mutation.expected == "REJECT").take(8) { div { class: "reason-cell", key: "{mutation.name}", code { "{mutation.observed.reason.as_deref().unwrap_or(\"NO_REASON\")}" }, span { "{mutation.name}" } } } }, a { class: "text-link", href: "/lab", "Open Tampering Lab →" } }
                div { class: "content-panel content-panel-muted", div { class: "panel-kicker", "EXTERNAL REPRODUCTION" }, h3 { "NOT YET DEMONSTRATED" }, p { "No independent human clean-checkout result is claimed in this workspace yet. The repository's own gates remain separate from that future evidence." }, a { class: "text-link", href: "/reproduce", "See the reproducible commands →" } }
            }
            section { class: "section-wrap artifact-list content-panel",
                div { class: "panel-kicker", "AVAILABLE ARTIFACTS" }
                div { class: "artifact-row", span { "evidence/local/rust_revm_conformance.json" }, StatusBadge { label: "PUBLISHED".to_owned(), tone: "observed".to_owned() } }
                div { class: "artifact-row", span { "evidence/local/rust_revm_mutation_matrix.json" }, StatusBadge { label: "PUBLISHED".to_owned(), tone: "observed".to_owned() } }
                div { class: "artifact-row", span { "evidence/sepolia/sepolia_reread.json" }, StatusBadge { label: "PUBLISHED".to_owned(), tone: "observed".to_owned() } }
                div { class: "artifact-row", span { "evidence/sepolia/silent_rollback_fixture_eth_call.json" }, StatusBadge { label: "READ-ONLY".to_owned(), tone: "observed".to_owned() } }
                div { class: "artifact-row", span { "contracts/vectors/erc8350_conformance.json" }, StatusBadge { label: "PINNED".to_owned(), tone: "warning".to_owned() } }
            }
        }
    }
}

#[component]
pub fn ArchitecturePage(data: UiData) -> Element {
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "ARCHITECTURE / SYSTEM OVERVIEW".to_owned(), title: "How does MemoryLineage work?".to_owned(), description: "The website is an inspection surface over a Rust-first application and verification stack with a minimal Solidity Ethereum trust anchor.".to_owned(), source: "CURRENT RUST STACK".to_owned() }
            section { class: "section-wrap architecture-flow content-panel",
                div { class: "architecture-node architecture-private", strong { "PRIVATE AGENT MEMORY" }, span { "SQLite snapshots" } }
                div { class: "architecture-arrow", "↓" }
                div { class: "architecture-node", strong { "RUST MEMORY STORE" }, span { "deterministic read / restore" } }
                div { class: "architecture-arrow", "↓" }
                div { class: "architecture-node", strong { "RUST CANONICALIZATION" }, span { "ml-core / commitments / pinned vectors" } }
                div { class: "architecture-arrow", "↓" }
                div { class: "architecture-node architecture-chain", strong { "SOLIDITY REGISTRY" }, span { "Ethereum Sepolia / EIP-712 / ERC-1271" } }
                div { class: "architecture-branches", div { class: "architecture-branch", strong { "Rust/WASM Inspector" }, span { "read-only RPC + browser verifier" } }, div { class: "architecture-branch", strong { "Portable Evidence" }, span { "V1/V2 JSON + independent replay" } }, div { class: "architecture-branch", strong { "Rust CLI" }, span { "secondary developer auditor" } } }
            }
            section { class: "section-wrap architecture-grid",
                div { class: "content-panel", div { class: "panel-kicker", "WHY BLOCKCHAIN" }, h2 { "The operator should not be the only historian." }, p { "The registry enforces a public append-only boundary for the commitments that define succession. The operator may still control private memory storage; an external verifier does not have to blindly trust its presented order." }, a { class: "text-link", href: "/security", "Read the trust boundary →" } }
                div { class: "content-panel", div { class: "panel-kicker", "CURRENT EVIDENCE" }, dl { class: "readout-list", ReadoutRow { label: "Rust toolchain".to_owned(), value: "1.97.1".to_owned(), detail: "repository pinned".to_owned() }, ReadoutRow { label: "Pinned spec".to_owned(), value: "ERC-8350".to_owned(), detail: "v1-pinned-vector-2026-09-18".to_owned() }, ReadoutRow { label: "Dioxus".to_owned(), value: "0.8.0-alpha.1".to_owned(), detail: "WASM frontend".to_owned() }, ReadoutRow { label: "Published head".to_owned(), value: data.v2.head.sequence.to_string(), detail: "local replay bundle".to_owned() } } }
            }
            section { class: "section-wrap content-panel", div { class: "panel-kicker", "DATA TYPES" }, div { class: "data-type-grid", div { strong { "ON CHAIN" }, span { "sequence" }, span { "prevStateRoot" }, span { "commitments" }, span { "transitionId" } }, div { strong { "OFF CHAIN" }, span { "raw memory" }, span { "private documents" }, span { "locator contents" }, span { "agent context" } }, div { strong { "EVIDENCE" }, span { "head observation" }, span { "authority history" }, span { "replay result" }, span { "mutation outcome" } } } }
        }
    }
}

#[component]
pub fn SecurityPage() -> Element {
    let verified = [
        "committed ordering",
        "sequence continuity",
        "predecessor continuity",
        "configured authorization",
        "authority rotation history",
        "commitment integrity",
        "deterministic state derivation",
        "replayable evidence",
        "canonical committed head",
    ];
    let not_verified = [
        "semantic truth",
        "semantic memory safety",
        "AI reasoning correctness",
        "inference correctness",
        "agent behavioral safety",
        "off-chain data availability",
        "causal action proof",
        "complete protection from all memory poisoning",
    ];
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "SECURITY / CLAIM BOUNDARY".to_owned(), title: "What does this system actually guarantee?".to_owned(), description: "MemoryLineage is intentionally narrow. This page makes the verified surface and the remaining trust assumptions visible.".to_owned(), source: "EXPLICIT SCOPE".to_owned() }
            section { class: "section-wrap security-scope-grid",
                div { class: "content-panel scope-panel scope-panel-positive", div { class: "panel-kicker", "WHAT MEMORYLINEAGE VERIFIES" }, h2 { "Committed history integrity" }, for item in verified { div { class: "scope-row", key: "{item}", span { class: "scope-symbol", "✓" }, span { "{item}" } } } }
                div { class: "content-panel scope-panel scope-panel-muted", div { class: "panel-kicker", "WHAT MEMORYLINEAGE DOES NOT VERIFY" }, h2 { "Semantic or behavioral truth" }, for item in not_verified { div { class: "scope-row", key: "{item}", span { class: "scope-symbol scope-symbol-muted", "—" }, span { "{item}" } } } }
            }
            section { class: "section-wrap security-detail-grid",
                div { class: "content-panel", div { class: "panel-kicker", "THREAT MODEL" }, h3 { "The operator can change private storage." }, p { "The protocol does not stop an operator from restoring, editing, or deleting an off-chain snapshot. It makes a stale or conflicting snapshot observable when it tries to become the next canonical committed state." }, h3 { "The first valid write is still trusted." }, p { "A malicious memory value can be authorized and committed. The lineage remains structurally valid because semantic content is outside this protocol's scope." } }
                div { class: "content-panel", div { class: "panel-kicker", "AUTHORIZATION BOUNDARY" }, h3 { "ERC-1271 is observed, not overclaimed." }, p { "For contract-based authorizers, the published lane demonstrates registry acceptance and rejection behavior. That is distinct from a full historical offline re-execution of the signer contract." }, div { class: "boundary-callout boundary-callout-light", strong { "ON-CHAIN ACCEPTANCE" }, span { "does not automatically mean FULL HISTORICAL OFFLINE SIGNER-CONTRACT REEXECUTION" } } }
            }
            section { class: "section-wrap content-panel", div { class: "panel-kicker", "STATUS VOCABULARY" }, div { class: "status-vocabulary", StatusBadge { label: "VERIFIED".to_owned(), tone: "verified".to_owned() }, span { "relevant deterministic checks passed" }, StatusBadge { label: "OBSERVED".to_owned(), tone: "observed".to_owned() }, span { "read from a public source" }, StatusBadge { label: "REJECTED".to_owned(), tone: "danger".to_owned() }, span { "actual verification/simulation failed as expected" }, StatusBadge { label: "OUT OF SCOPE".to_owned(), tone: "warning".to_owned() }, span { "not evaluated by this product" }, StatusBadge { label: "NOT YET DEMONSTRATED".to_owned(), tone: "neutral".to_owned() }, span { "no evidence is being implied" } } }
        }
    }
}

#[component]
pub fn ReproducePage(data: UiData) -> Element {
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "REPRODUCE / CLEAN CHECKOUT".to_owned(), title: "Can another developer reproduce these claims?".to_owned(), description: "The commands below are the repository's current verification surfaces. They are evidence of reproducibility work, not a security audit.".to_owned(), source: "REPOSITORY COMMANDS".to_owned() }
            section { class: "section-wrap reproduce-hero content-panel", div { class: "panel-kicker", "PRIMARY GATE" }, h2 { "cargo xtask verify" }, p { "Runs the repository's current Rust verification, conformance, local execution, evidence replay, WASM, and package-boundary checks." }, code { class: "command-block command-block-large", "cargo xtask verify" }, div { class: "action-row", a { class: "button button-primary", href: "/verify", "Verify evidence in the browser" }, a { class: "button button-secondary", href: "/evidence", "Inspect published outputs" } } }
            section { class: "section-wrap reproduce-grid",
                div { class: "content-panel", div { class: "panel-kicker", "LOCAL COMMANDS" }, CommandRow { command: "cargo test --workspace".to_owned(), note: "Rust workspace tests".to_owned() }, CommandRow { command: "cargo run -p ml-verifier-independent -- evidence.json".to_owned(), note: "independent evidence replay".to_owned() }, CommandRow { command: "cargo run -p ml-cli -- demo silent-rollback".to_owned(), note: "fixture rehearsal".to_owned() }, CommandRow { command: "cargo xtask build-web".to_owned(), note: "static Rust/WASM website build".to_owned() } }
                div { class: "content-panel", div { class: "panel-kicker", "CURRENT FINGERPRINT" }, dl { class: "readout-list", ReadoutRow { label: "Rust".to_owned(), value: "1.97.1".to_owned(), detail: "rust-toolchain.toml".to_owned() }, ReadoutRow { label: "Published vectors".to_owned(), value: "MATCH".to_owned(), detail: "ERC-8350 pinned corpus".to_owned() }, ReadoutRow { label: "Silent Rollback".to_owned(), value: "REJECTED".to_owned(), detail: "BAD_PREVIOUS_STATE".to_owned() }, ReadoutRow { label: "External developers".to_owned(), value: "NOT YET DEMONSTRATED".to_owned(), detail: "do not infer human validation".to_owned() }, ReadoutRow { label: "Submission tag".to_owned(), value: "NOT YET CREATED".to_owned(), detail: "no fabricated release identity".to_owned() } } }
            }
            section { class: "section-wrap content-panel", div { class: "panel-kicker", "WHAT THIS PROVES" }, div { class: "reproduce-proof-grid", div { strong { "Same source" }, span { "A clean checkout can execute the published gates." } }, div { strong { "Same evidence" }, span { "The browser and CLI consume portable JSON." } }, div { strong { "Same boundaries" }, span { "The product does not silently promote unproven claims." } } }, p { class: "small-note", "Current published bundle: {data.bundle.evidence_type}. An independent human clean-checkout result is intentionally not claimed yet." } }
        }
    }
}

#[component]
pub fn PriorWorkPage(data: UiData) -> Element {
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "PRIOR WORK / SUBMISSION PROVENANCE".to_owned(), title: "What existed before the hackathon?".to_owned(), description: "MemoryLineage separates standards and Ethereum primitives from the evidence-backed work packaged for this submission.".to_owned(), source: "PROVENANCE LEDGER".to_owned() }
            section { class: "section-wrap provenance-grid",
                div { class: "content-panel provenance-column", div { class: "panel-kicker", "BEFORE 3RD-WEB-HACK" }, h2 { "Standards and primitives" }, ProvenanceRow { label: "ERC-8350 draft semantics".to_owned(), detail: "pinned as an external standard snapshot".to_owned(), tone: "observed".to_owned() }, ProvenanceRow { label: "EIP-712".to_owned(), detail: "Ethereum typed-data authorization primitive".to_owned(), tone: "observed".to_owned() }, ProvenanceRow { label: "ERC-1271".to_owned(), detail: "contract-based signature validation boundary".to_owned(), tone: "observed".to_owned() }, ProvenanceRow { label: "Ethereum Sepolia".to_owned(), detail: "public testnet used as trust anchor".to_owned(), tone: "observed".to_owned() } }
                div { class: "provenance-divider", "→" }
                div { class: "content-panel provenance-column provenance-column-built", div { class: "panel-kicker", "BUILT / EVIDENCED FOR THIS SUBMISSION" }, h2 { "Independent audit product" }, ProvenanceRow { label: "MemoryLineage Inspector".to_owned(), detail: "Rust/WASM evidence workspace".to_owned(), tone: "verified".to_owned() }, ProvenanceRow { label: "Rust reference + verifier".to_owned(), detail: "separate deterministic replay paths".to_owned(), tone: "verified".to_owned() }, ProvenanceRow { label: "Silent Rollback fixture".to_owned(), detail: format!("private SQLite snapshots {} / {} / {}", data.fixture.snapshots[0].sequence, data.fixture.snapshots[1].sequence, data.fixture.snapshots[2].sequence), tone: "verified".to_owned() }, ProvenanceRow { label: "Tampering corpus".to_owned(), detail: format!("{}/{} expected rejection cases", data.mutation_rejected, data.mutation_count), tone: "verified".to_owned() }, ProvenanceRow { label: "Sepolia evidence".to_owned(), detail: "deployment + second RPC reread".to_owned(), tone: "verified".to_owned() } }
            }
            section { class: "section-wrap content-panel provenance-ledger",
                div { class: "panel-title-row", div { p { class: "panel-kicker", "SUBMISSION IDENTITY" }, h2 { "Only finalized values are listed." } }, StatusBadge { label: "TRANSPARENT".to_owned(), tone: "warning".to_owned() } }
                dl { class: "readout-list", ReadoutRow { label: "Contract address".to_owned(), value: short_hash(&data.deployment.registry_address, 14, 8), detail: "workspace-owned Sepolia deployment".to_owned() }, ReadoutRow { label: "Spec pin".to_owned(), value: "ERC-8350 / v1-pinned-vector-2026-09-18".to_owned(), detail: "published vector snapshot".to_owned() }, ReadoutRow { label: "GitHub URL".to_owned(), value: "AndroLay/MemoryLineage".to_owned(), detail: "private repository / configured".to_owned() }, ReadoutRow { label: "Submission tag".to_owned(), value: "NOT YET CREATED".to_owned(), detail: "release identity remains pending".to_owned() } }
                p { class: "small-note", "MemoryLineage does not claim to have invented ERC-8350 or to be the first AI memory lineage system. Its distinction is the independent, authorization-bound audit surface and reproducible evidence around the pinned semantics." }
            }
            section { class: "section-wrap content-panel", div { class: "panel-kicker", "RELATED SURFACES" }, div { class: "related-link-grid", a { href: "/architecture", "Architecture →" }, a { href: "/evidence", "Public evidence →" }, a { href: "/security", "Security boundary →" }, a { href: "/reproduce", "Reproduce →" } } }
        }
    }
}
