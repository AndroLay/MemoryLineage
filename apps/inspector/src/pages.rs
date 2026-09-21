#![forbid(unsafe_code)]

use crate::AppRoute;
use crate::browser::{
    LiveRollbackOutcome, SEPOLIA_PROVIDER_LABEL, download_json, live_silent_rollback,
};
use crate::components::*;
use crate::data::{
    RestoreAssessment, Scenario, UiData, classify_restore_candidate, short_hash, tamper_commitment,
    verify_evidence_json, verify_recovery_receipt_json,
};
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RollbackUiState {
    Ready,
    ProbingSepolia,
    LiveRejected,
    EvidenceRejected,
    LiveUnavailable,
    EvidenceInvalid,
    UnexpectedLive,
}

#[component]
pub fn HomePage(data: UiData) -> Element {
    let head = data.head();
    let attack = data.v2.attack.as_ref();
    let restored_sequence = data.restored_snapshot().sequence;
    let attempted_sequence = attack
        .and_then(|value| value.attempted_sequence)
        .unwrap_or(head.delta.sequence.saturating_add(1));
    let stale_root = attack
        .and_then(|value| value.stale_predecessor.as_deref())
        .unwrap_or("NOT AVAILABLE");
    let contract_reason = attack
        .and_then(|value| value.reason.as_deref())
        .unwrap_or("BAD_PREVIOUS_STATE");
    rsx! {
        div { class: "page page-home",
            section { class: "home-hero",
                div { class: "home-hero-copy",
                    p { class: "eyebrow", "MEMORY LINEAGE" }
                    p { class: "home-tagline", "Verify the history, not the memory." }
                    h1 { "An old AI-memory backup can look current." }
                    p { class: "home-lede", "The file still opens and the agent can still start. MemoryLineage checks whether that restored private state is the authorized continuation of the shared history before anyone treats it as current, without publishing the private memory itself." }
                    div { class: "action-row",
                        Link { class: "button button-primary", to: AppRoute::Inspect {}, "Check an old restore ", Icon { name: IconName::Arrow, size: 16 } }
                        Link { class: "button button-secondary", to: AppRoute::Verify {}, "Verify Evidence" }
                    }
                    section { class: "home-metrics",
                        div { class: "home-metric", Icon { name: IconName::Database, size: 27 }, div { strong { "{data.fixture.snapshots.len()}" }, span { "private snapshots in the demo" } } }
                        div { class: "home-metric", Icon { name: IconName::Shield, size: 27 }, div { strong { "{data.mutation_rejected}/{data.mutation_count}" }, span { "known history attacks rejected" } } }
                        div { class: "home-metric", Icon { name: IconName::Open, size: 27 }, div { strong { "Sepolia" }, span { "separate chain observation" } } }
                    }
                }
                div { class: "home-hero-trust content-panel",
                    div { class: "home-trust-heading",
                        div { p { class: "panel-kicker", "THE INCIDENT / ONE EVIDENCE PATH" } strong { "Old restore vs canonical head" } }
                        span { class: "home-trust-status", "LOCAL DEMO SPACE V2" }
                    }
                    div { class: "incident-rail", aria_label: "Demo Space V2 canonical snapshots",
                        for snapshot in data.fixture.snapshots.iter() {
                            div { class: if snapshot.sequence == head.delta.sequence { "incident-state incident-state-current" } else { "incident-state" },
                                span { class: "incident-sequence", "{snapshot.sequence:02}" }
                                span { class: "incident-label", "{snapshot.visible_label.as_deref().unwrap_or(\"Snapshot\")}" }
                                strong { if snapshot.sequence == head.delta.sequence { "CANONICAL HEAD" } else { "COMMITTED" } }
                                code { "{short_hash(&snapshot.snapshot_commitment, 8, 6)}" }
                            }
                        }
                    }
                    div { class: "incident-attempt",
                        div { span { "RESTORED LOCALLY" } strong { "Snapshot {restored_sequence}" } code { "{short_hash(&stale_root, 8, 6)}" } }
                        div { class: "incident-arrow", "→" }
                        div { span { "ATTEMPTED CONTINUATION" } strong { "Transition {attempted_sequence}" } code { "{contract_reason}" } }
                    }
                    div { class: "incident-result", span { "EXACT MACHINE RESULT" }, StatusBadge { label: "BAD_PREVIOUS_STATE".to_owned(), tone: "danger".to_owned() } }
                    div { class: "trust-list",
                        div { class: "trust-item", Icon { name: IconName::Lock, size: 24 }, div { strong { "Memory stays local" }, span { "Only commitments cross the boundary." } } }
                        div { class: "trust-item", Icon { name: IconName::History, size: 24 }, div { strong { "The checkpoint is shared" }, span { "Anyone can replay the committed succession." } } }
                        div { class: "trust-item", Icon { name: IconName::Authority, size: 24 }, div { strong { "Authority is recorded" }, span { "Changes are part of the evidence." } } }
                        div { class: "trust-item", Icon { name: IconName::Open, size: 24 }, div { strong { "Evidence travels" }, span { "Export the bundle and verify it elsewhere." } } }
                    }
                }
            }
            section { class: "section-wrap home-problem-explainer content-panel",
                div { class: "home-problem-copy",
                    p { class: "panel-kicker", "THE PROBLEM IN PLAIN LANGUAGE" }
                    h2 { "An old backup can look perfectly normal." }
                    p { "The database opens. The agent starts. But the people who did not operate it have no independent way to tell whether the restored memory is the latest state or an older one." }
                    p { class: "home-problem-emphasis", "MemoryLineage answers one question: can this private snapshot continue the history everyone agreed on?" }
                }
                div { class: "home-problem-steps", aria_label: "The restore problem",
                    div { class: "home-problem-step",
                        span { class: "home-problem-step-number", "01" }
                        strong { "The agent remembers" }
                        span { "Useful context is stored in a private database." }
                    }
                    div { class: "home-problem-step",
                        span { class: "home-problem-step-number", "02" }
                        strong { "An old backup is restored" }
                        span { "The local file can go back to snapshot {restored_sequence}, even when newer states were already recorded." }
                    }
                    div { class: "home-problem-step home-problem-step-alert",
                        span { class: "home-problem-step-number", "03" }
                        strong { "The reviewer cannot tell" }
                        span { "Without a shared history, an older state can look like the current one." }
                    }
                }
                div { class: "home-problem-answer",
                    span { class: "home-problem-answer-label", "MEMORYLINEAGE ADDS THE MISSING CHECK" }
                    strong { "An older root cannot silently become the next shared state." }
                    Link { class: "text-link", to: AppRoute::LabScenario { scenario: "silent-rollback".to_owned() }, "See the failed restore →" }
                }
            }
            section { class: "home-overview section-wrap content-panel",
                div { class: "panel-title-row home-overview-heading",
                    div { p { class: "panel-kicker", "CANONICAL OVERVIEW" } h2 { "Committed private-memory lineage" } p { class: "panel-subtitle", "Blue is the current head. Red is the attempted continuation from an older snapshot." } }
                    div { class: "home-overview-sources",
                        SourceLine { source: "SYNTHETIC SNAPSHOTS".to_owned(), note: format!("{} public sample commitments", data.fixture.snapshots.len()) }
                        SourceLine { source: "LOCAL REVM".to_owned(), note: format!("{} transitions / authority rotation", data.v2.transitions.len()) }
                    }
                }
                div { class: "home-overview-layout",
                    div { class: "home-overview-lineage",
                        HomeLineageRail { data: data.clone() }
                        Link { class: "text-link", to: AppRoute::History {}, "View full history ", Icon { name: IconName::Arrow, size: 15 } }
                    }
                    aside { class: "home-selected-state",
                        div { class: "panel-title-row", div { p { class: "panel-kicker", "SELECTED CANONICAL HEAD" } h2 { "State {head.delta.sequence}" } }, StatusBadge { label: "CURRENT / CANONICAL".to_owned(), tone: "verified".to_owned() } }
                        CopyValue { label: "state root".to_owned(), value: head.next_state_root.clone(), compact: false }
                        dl { class: "readout-list home-selected-readout",
                            ReadoutRow { label: "Status".to_owned(), value: "canonical".to_owned(), detail: "Demo Space V2 head".to_owned() }
                            ReadoutRow { label: "Registry".to_owned(), value: short_hash(&data.v2.registry.address, 12, 8), detail: "Rust/revm Solidity lane".to_owned() }
                            ReadoutRow { label: "Sequence".to_owned(), value: head.delta.sequence.to_string(), detail: "ordered transition".to_owned() }
                        }
                        Link { class: "button button-secondary button-full", to: AppRoute::Inspect {}, "View on Inspect →" }
                    }
                }
            }
            section { class: "section-wrap home-columns",
                div { class: "content-panel scope-panel-positive",
                    div { class: "panel-kicker", "WHAT MEMORYLINEAGE VERIFIES" }
                    h2 { "What the registry can answer." }
                    ul { class: "scope-list scope-list-positive",
                        li { strong { "Ordered history" }, span { "Each accepted update uses the next sequence; gaps are rejected." } }
                        li { strong { "Predecessor binding" }, span { "A new update must extend the current root, not an older backup." } }
                        li { strong { "Registry authorization rule" }, span { "The registry checks the configured authorizer; Demo Space V2 records authority changes and EIP-712 proof material." } }
                        li { strong { "Portable evidence" }, span { "A separate Rust verifier can replay the commitments and head." } }
                    }
                }
                div { class: "content-panel content-panel-muted scope-panel-negative",
                    div { class: "panel-kicker", "WHAT MEMORYLINEAGE DOES NOT VERIFY" }
                    h2 { "What the registry cannot answer." }
                    ul { class: "scope-list scope-list-muted",
                        li { strong { "Not memory truth" }, span { "The verifier checks commitments, not private text's meaning or accuracy." } }
                        li { strong { "Not AI reasoning" }, span { "It does not explain the agent's decision." } }
                        li { strong { "Not total safety" }, span { "Authorized content can still be unsafe." } }
                        li { strong { "Not a runtime gate" }, span { "This fixture does not gate an external runtime." } }
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
                Link { class: "text-link", to: AppRoute::Architecture {}, "Read the architecture →" }
            }
            section { class: "section-wrap home-reviewer-flow content-panel",
                div { class: "panel-kicker", "HOW TO USE MEMORYLINEAGE" }
                h2 { "Follow one short path." }
                div { class: "judge-path", aria_label: "Judge path",
                    div { class: "judge-step",
                        span { class: "judge-step-number", "01" }
                        div { strong { "Inspect" }, span { "Choose a snapshot and see how it compares with the recorded head." }, Link { class: "judge-step-link", to: AppRoute::Inspect {}, "Open Inspect →" } }
                    }
                    div { class: "judge-step-arrow", "→" }
                    div { class: "judge-step",
                        span { class: "judge-step-number", "02" }
                        div { strong { "Test" }, span { "Try to continue from an older snapshot." }, Link { class: "judge-step-link", to: AppRoute::LabScenario { scenario: "silent-rollback".to_owned() }, "Open Tampering Lab →" } }
                    }
                    div { class: "judge-step-arrow", "→" }
                    div { class: "judge-step",
                        span { class: "judge-step-number judge-step-number-danger", "03" }
                        div { strong { "Verify" }, span { "Export, tamper, restore, and replay the evidence." }, Link { class: "judge-step-link", to: AppRoute::Verify {}, "Open Verify →" } }
                    }
                }
                p { class: "home-judge-note", "You do not need to understand the cryptography first. Start with Inspect, try the older snapshot in the Lab, then use Verify to check the same result outside the page." }
            }
            section { class: "section-wrap home-evidence-note",
                SourceLine { source: "DEMO SPACE V2 / LOCAL REVM".to_owned(), note: format!("{} committed states / {} mutation cases", data.v2.transitions.len(), data.mutation_count) }
                p { class: "home-source-note", "The old memory backup is state {restored_sequence}; the registry remains at state {head.delta.sequence}. Transition {attempted_sequence} presents a stale predecessor root ({short_hash(stale_root, 10, 8)}) and returns {contract_reason}. The existing Sepolia deployment is a separate observation." }
                div { class: "home-snapshot-notes",
                    for snapshot in data.fixture.snapshots.iter() {
                        span { "State {snapshot.sequence}: {snapshot.visible_label.as_deref().unwrap_or(\"Label unavailable\")}" }
                    }
                }
            }
            section { class: "section-wrap content-panel home-web3-reason",
                div { class: "panel-kicker", "WHY A SHARED REGISTRY" }
                h2 { "An operator can restore a file. They cannot rewrite the checkpoint." }
                p { "A local log can identify its signer, but the operator still controls which history is shown after a restore. The registry keeps the predecessor and authority boundary comparable while the underlying memory stays private." }
                div { class: "web3-reason-grid",
                    div { class: "web3-reason-item", strong { "Operator-controlled log" }, span { "The runtime and its audit storage can be rolled back together." } }
                    div { class: "web3-reason-item", strong { "Shared registry" }, span { "Independent parties can compare a candidate against one committed head and authority timeline." } }
                    div { class: "web3-reason-item", strong { "Private boundary" }, span { "Raw memory, documents, and locator contents stay outside chain data and portable evidence." } }
                }
            }
            section { class: "section-wrap judge-next-step content-panel",
                div { class: "panel-kicker", "NEXT REVIEW STEP" }
                div { class: "judge-next-layout",
                    div {
                        h2 { "Continue the review." }
                        p { "Inspect the head, trace the transitions, run the stale-root attempt, and export the same evidence bundle." }
                    }
                    div { class: "action-row action-row-tight",
                        Link { class: "button button-primary", to: AppRoute::Inspect {}, "Start Inspect →" }
                        Link { class: "button button-quiet", to: AppRoute::Reproduce {}, "Reproduce locally" }
                    }
                }
            }
        }
    }
}

#[component]
fn PreflightSnapshotButton(
    sequence: u64,
    label: String,
    selected: bool,
    on_select: EventHandler<u64>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: if selected { "preflight-snapshot preflight-snapshot-selected" } else { "preflight-snapshot" },
            aria_pressed: selected,
            onclick: move |_| on_select.call(sequence),
            span { class: "preflight-sequence", "STATE {sequence}" }
            strong { "{label}" }
        }
    }
}

#[component]
pub fn InspectPage(data: UiData) -> Element {
    let head = data.head();
    let history_count = data.v2.authorization_history.len().saturating_sub(1);
    let mut selected_snapshot = use_signal(|| data.canonical_snapshot().sequence);
    let mut candidate_tampered = use_signal(|| false);
    let selected_sequence = selected_snapshot();
    let candidate_tampered_now = candidate_tampered();
    let selected_snapshot_record = data
        .fixture
        .snapshots
        .iter()
        .find(|snapshot| snapshot.sequence == selected_sequence);
    let candidate_commitment = selected_snapshot_record.map(|snapshot| {
        if candidate_tampered_now {
            tamper_commitment(&snapshot.snapshot_commitment)
                .unwrap_or_else(|| snapshot.snapshot_commitment.clone())
        } else {
            snapshot.snapshot_commitment.clone()
        }
    });
    let assessment = if candidate_tampered_now {
        candidate_commitment
            .as_deref()
            .map(|commitment| classify_restore_candidate(commitment, &data.v2))
            .unwrap_or_else(|| data.assess_snapshot(selected_sequence))
    } else {
        data.assess_snapshot(selected_sequence)
    };
    let recovery_receipt = candidate_commitment.as_deref().and_then(|commitment| {
        data.recovery_receipt_for_candidate(selected_sequence, commitment)
            .ok()
    });
    let recovery_receipt_json = recovery_receipt.as_ref().map(|receipt| {
        serde_json::to_string_pretty(receipt).expect("recovery receipt must serialize")
    });
    let receipt_download = recovery_receipt_json.clone();
    let receipt_available = receipt_download.is_some();
    let (assessment_label, assessment_tone, assessment_detail) = match &assessment {
        RestoreAssessment::EvidenceHeadMatch { sequence } => (
            "MATCHES DEMO EVIDENCE HEAD",
            "verified",
            format!("Snapshot #{sequence} matches the head of the independently replayed local Demo Space V2 bundle. This is not a live RPC observation or runtime approval."),
        ),
        RestoreAssessment::KnownHistoricalCheckpoint { sequence, head_sequence } => (
            "KNOWN HISTORICAL CHECKPOINT",
            "warning",
            format!("Snapshot #{sequence} is present in the verified history, whose current evidence head is #{head_sequence}. Use this older state for isolated rehearsal; it is not the current continuation."),
        ),
        RestoreAssessment::UnknownOrDiverged => (
            "UNKNOWN / DIVERGED",
            "danger",
            "This snapshot commitment does not match a checkpoint in the replayed Demo Space V2 history.".to_owned(),
        ),
        RestoreAssessment::Unverified { reason } => (
            "UNVERIFIED",
            "warning",
            format!("The candidate or its evidence could not be verified ({reason}). This result cannot be treated as a current head."),
        ),
    };
    rsx! {
        div { class: "page workspace-page",
            PageHeader {
                kicker: "INSPECT / MEMORY SPACE".to_owned(),
                title: "Inspect Memory Space".to_owned(),
                description: "Choose a snapshot below. MemoryLineage will tell you whether it is current, an older checkpoint, or a state it cannot verify.".to_owned(),
                source: "DEMO SPACE V2 / LOCAL REVM".to_owned(),
            }
            div { class: "inspect-context-bar",
                span { class: "inspect-context-label", "SPACE" }
                code { "{short_hash(&data.v2.registry.space_id, 10, 6)}" }
                span { class: "inspect-context-separator", "•" }
                span { "Inspect canonical head" }
            }
            div { class: "inspect-query",
                div { class: "field-readout", span { "NETWORK" }, strong { "{data.v2.network.name}" }, code { "CHAIN {data.v2.network.chain_id}" } }
                div { class: "field-readout", span { "REGISTRY ADDRESS" }, strong { "{short_hash(&data.v2.registry.address, 14, 8)}" }, code { "LOCAL SOLIDITY INSTANCE" } }
                div { class: "field-readout", span { "SPACE ID" }, strong { "{short_hash(&data.v2.registry.space_id, 14, 8)}" }, code { "DEMO SPACE V2" } }
                Link { class: "button button-primary", to: AppRoute::History {}, "View history ", Icon { name: IconName::Arrow, size: 16 } }
            }
            div { class: "inspect-layout",
                main {
                    section { class: "content-panel inspect-lineage-panel",
                        div { class: "panel-title-row",
                            div { p { class: "panel-kicker", "COMMITTED PRIVATE-MEMORY LINEAGE" } h2 { "{data.v2.transitions.len()} observed states" } }
                            div { class: "inspect-view-controls", span { "View" }, button { class: "view-control view-control-active", r#type: "button", "Linear" }, button { class: "view-control", r#type: "button", "Tree" } }
                        }
                        LineageRail { data: data.clone(), compact: false }
                        div { class: "inspect-lineage-footer", span { "{data.v2.transitions.len()} observed states" }, span { "|" }, span { "head sequence {head.delta.sequence}" }, span { "|" }, span { "local Demo Space V2" }, Link { class: "text-link", to: AppRoute::History {}, "View full history ", Icon { name: IconName::Arrow, size: 14 } } }
                    }
                    section { class: "inspect-summary-cards",
                        div { class: "content-panel inspect-summary-card", Icon { name: IconName::Database, size: 24 }, div { p { class: "panel-kicker", "CURRENT RECORDED HEAD" } h2 { "State {head.delta.sequence}" } CopyValue { label: "canonical head".to_owned(), value: head.next_state_root.clone(), compact: true } } }
                        div { class: "content-panel inspect-summary-card", Icon { name: IconName::Authority, size: 24 }, div { p { class: "panel-kicker", "WHEN AUTHORITY CHANGED" } h2 { "{history_count}" } span { "approval changes recorded" } } }
                        div { class: "content-panel inspect-summary-card", Icon { name: IconName::Shield, size: 24 }, div { p { class: "panel-kicker", "MISSING STEPS" } h2 { "0" } span { "History is continuous" } } }
                    }
                    section { class: "content-panel inspect-boundary-panel",
                        div { class: "boundary-half", div { class: "panel-kicker", "ON CHAIN" }, p { "What's public and verifiable" }, div { class: "boundary-tags", span { "state commitments" }, span { "transition metadata" }, span { "authorizations" }, span { "timestamps" } } }
                        div { class: "boundary-half", div { class: "panel-kicker", "PRIVATE / OFF-CHAIN" }, p { "What stays private" }, div { class: "boundary-tags", span { "raw memory content" }, span { "agent thoughts" }, span { "sensitive data" }, span { "intermediate context" } } }
                    }
                    section { class: "inspect-scope-grid",
                        div { class: "content-panel scope-panel scope-panel-positive", div { class: "panel-kicker", "WHAT WE VERIFY" }, for item in ["State transitions and lineage integrity", "Authorizations and authority rotation", "On-chain commitments and registry records", "Consistency between proofs and committed state"] { div { class: "scope-row", key: "{item}", span { class: "scope-symbol", "✓" }, span { "{item}" } } } }
                        div { class: "content-panel scope-panel scope-panel-muted", div { class: "panel-kicker", "WHAT WE DO NOT VERIFY" }, for item in ["The content of private memory", "Correctness or truthfulness of agent outputs", "Off-chain computation or model behavior", "Real-world actions taken by the agent"] { div { class: "scope-row", key: "{item}", span { class: "scope-symbol scope-symbol-muted", "×" }, span { "{item}" } } } }
                    }
                }
                aside { class: "inspect-sidebar",
                    section { class: "content-panel inspect-selected-panel",
                        div { class: "panel-title-row", div { p { class: "panel-kicker", "SELECTED STATE" } h2 { "State {head.delta.sequence}" } }, StatusBadge { label: "CURRENT / CANONICAL".to_owned(), tone: "verified".to_owned() } }
                        CopyValue { label: "state root".to_owned(), value: head.next_state_root.clone(), compact: false }
                        dl { class: "readout-list", ReadoutRow { label: "Status".to_owned(), value: "canonical".to_owned(), detail: "local Demo Space V2 head".to_owned() } ReadoutRow { label: "Transition ID".to_owned(), value: short_hash(&head.transition_id, 14, 8), detail: "commitment-bound".to_owned() } ReadoutRow { label: "Authorizer".to_owned(), value: data.v2.authorization_history.last().map(|authority| short_hash(&authority.authorizer, 14, 8)).unwrap_or_else(|| "NOT AVAILABLE".to_owned()), detail: "effective authority".to_owned() } ReadoutRow { label: "Registry".to_owned(), value: short_hash(&data.v2.registry.address, 14, 8), detail: "local Solidity instance".to_owned() } }
                        Link { class: "button button-secondary button-full", to: AppRoute::Transition { sequence: head.delta.sequence }, "View state details ", Icon { name: IconName::Arrow, size: 14 } }
                    }
                    section { class: "content-panel inspect-evidence-panel", div { class: "panel-title-row", div { p { class: "panel-kicker", "EVIDENCE SOURCE" } h3 { "{data.v2.source_class}" } }, StatusBadge { label: "OBSERVED".to_owned(), tone: "observed".to_owned() } }, ReadoutRow { label: "Source".to_owned(), value: "Demo Space V2".to_owned(), detail: "local Rust/revm evidence".to_owned() }, ReadoutRow { label: "Sepolia".to_owned(), value: format!("block {}", data.reread.block_number), detail: "separate read-only reread".to_owned() }, p { class: "small-note", "A local restore does not move the registry head. The website labels local evidence and Sepolia separately." } }
                    section { class: "content-panel inspect-actions-panel", div { class: "panel-kicker", "NEXT STEP" }, p { class: "inspect-action-help", "The head is the latest recorded state. Test an older restore to see what the registry accepts." }, Link { class: "button button-primary button-wide", to: AppRoute::LabScenario { scenario: "silent-rollback".to_owned() }, "Simulate Rollback ", Icon { name: IconName::History, size: 15 } } Link { class: "button button-secondary button-wide", to: AppRoute::Verify {}, "Export Evidence ", Icon { name: IconName::Open, size: 15 } } Link { class: "button button-secondary button-wide", to: AppRoute::Verify {}, "Verify Bundle ", Icon { name: IconName::Shield, size: 15 } } }
                }
            }
            section { class: "section-wrap content-panel restore-preflight",
                div { class: "panel-title-row",
                    div { p { class: "panel-kicker", "BEFORE RESUME / RESTORE PREFLIGHT" } h2 { "Does this snapshot match the recorded history?" } p { class: "preflight-help", "Choose a checkpoint. We compare it with the recorded head and explain what you can do next." } }
                    StatusBadge { label: assessment_label.to_owned(), tone: assessment_tone.to_owned() }
                }
                div { class: "preflight-grid",
                    div { class: "preflight-candidates",
                        p { class: "preflight-label", "CHOOSE A SNAPSHOT" }
                        p { class: "preflight-hint", "These are synthetic demo checkpoints; no private user data is loaded." }
                        div { class: "preflight-snapshot-list", role: "group", aria_label: "Select a demo snapshot for restore preflight",
                            for snapshot in data.fixture.snapshots.iter() {
                                PreflightSnapshotButton {
                                    sequence: snapshot.sequence,
                                    label: snapshot.visible_label.clone().unwrap_or_else(|| "Label unavailable".to_owned()),
                                    selected: snapshot.sequence == selected_sequence,
                                    on_select: move |sequence| {
                                        selected_snapshot.set(sequence);
                                        candidate_tampered.set(false);
                                    },
                                }
                            }
                        }
                    }
                    div { class: "preflight-assessment", aria_live: "polite".to_owned(),
                        p { class: "preflight-label", "CANDIDATE COMMITMENT" }
                        if let Some(commitment) = candidate_commitment.as_deref() {
                            CopyValue { label: format!("snapshot {selected_sequence} commitment"), value: commitment.to_owned(), compact: true }
                        } else {
                            code { "NOT AVAILABLE IN FIXTURE" }
                        }
                        p { class: "preflight-detail", "{assessment_detail}" }
                        if let Some(receipt) = recovery_receipt.as_ref() {
                            div { class: "preflight-receipt",
                                div { class: "panel-kicker", "RECOVERY DECISION RECEIPT" }
                                div { class: "preflight-receipt-grid",
                                    ReadoutRow { label: "Classification".to_owned(), value: receipt.decision.classification.clone(), detail: receipt.decision.reason_code.clone() }
                                    ReadoutRow { label: "Protected action".to_owned(), value: receipt.decision.recommended_action.clone(), detail: "decision is portable; runtime enforcement is not claimed".to_owned() }
                                    ReadoutRow { label: "Decision policy".to_owned(), value: receipt.policy_id.clone(), detail: "strict current-head policy is part of the receipt digest".to_owned() }
                                }
                                CopyValue { label: "recovery decision ID".to_owned(), value: receipt.decision_id.clone(), compact: true }
                                SourceLine { source: receipt.evidence.source_class.clone(), note: "receipt binds this candidate to the verified evidence bundle".to_owned() }
                            }
                        }
                        div { class: "action-row action-row-tight",
                            button {
                                class: "button button-secondary",
                                r#type: "button",
                                aria_label: if candidate_tampered_now { "Restore original snapshot commitment" } else { "Tamper snapshot commitment in browser copy" },
                                onclick: move |_| candidate_tampered.set(!candidate_tampered_now),
                                if candidate_tampered_now { "Restore original" } else { "Tamper candidate" }
                            }
                            if !candidate_tampered_now && matches!(assessment, RestoreAssessment::KnownHistoricalCheckpoint { sequence: 1, .. }) {
                            Link { class: "button button-primary", to: AppRoute::LabScenario { scenario: "silent-rollback".to_owned() }, "Open State 1 Recovery Rehearsal" }
                            }
                            button {
                                class: "button button-quiet",
                                r#type: "button",
                                disabled: !receipt_available,
                                onclick: move |_| {
                                    if let Some(json) = receipt_download.clone() {
                                        let _ = download_json("memorylineage-recovery-receipt-v1.json", &json);
                                    }
                                },
                                "Download decision receipt"
                            }
                        }
                        if candidate_tampered_now { p { class: "preflight-boundary", "Only the browser candidate copy changed. No SQLite file, evidence artifact, or chain state was modified." } }
                    }
                }
                p { class: "preflight-boundary", "Source: synthetic fixture + local replayed evidence. This prototype does not read the live chain, inspect memory meaning, or gate an external agent runtime." }
            }
            section { class: "section-wrap compact-section",
                div { class: "panel-title-row", div { p { class: "panel-kicker", "CANONICAL CUE RAIL" } h2 { "Committed succession" } }, StatusBadge { label: format!("{} / {} VERIFIED", data.v2.transitions.len(), data.v2.transitions.len()), tone: "verified".to_owned() } }
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
    let history_verdict_tone = if data.report.is_some() {
        "verified"
    } else {
        "warning"
    };
    rsx! {
        div { class: "page workspace-page",
            div { class: "history-context-bar", span { class: "context-icon", Icon { name: IconName::Database, size: 18 } }, span { class: "context-label", "SPACE" }, code { "{short_hash(&data.v2.registry.space_id, 10, 6)}" }, span { "•" }, span { "{data.v2.transitions.len()} committed states" }, span { "•" }, span { "local evidence head" } }
            PageHeader { kicker: "HISTORY".to_owned(), title: "Canonical committed lineage and authority events.".to_owned(), description: "Each accepted transition has a predecessor, an authority record, and an evidence source.".to_owned(), source: "DEMO SPACE V2 / LOCAL REVM".to_owned() }
            div { class: "history-workspace",
                main { class: "history-main-column",
                    section { class: "content-panel history-main-panel history-lineage-panel",
                        div { class: "panel-title-row history-panel-heading", div { p { class: "panel-kicker", "CANONICAL MEMORY LINEAGE" } h2 { "{data.v2.transitions.len()} observed states" } }, div { class: "history-view-controls", span { "View" }, span { class: "view-control view-control-active", "Linear" }, span { class: "view-control", "Tree" }, span { class: "view-control", "Fit" }, span { class: "view-control", "Filters" } } }
                        div { class: "history-lineage-wrap", LineageRail { data: data.clone(), compact: true } }
                        Link { class: "text-link", to: AppRoute::History {}, "View full lineage ", Icon { name: IconName::Arrow, size: 14 } }
                    }
                    section { class: "content-panel history-event-panel",
                        div { class: "history-event-log-heading",
                            div { p { class: "panel-kicker", "EVENT LOG" } p { class: "panel-subtitle", "Canonical events for this space, latest evidence first." } }
                            Link { class: "text-link", to: AppRoute::History {}, "View all events ", Icon { name: IconName::Arrow, size: 14 } }
                        }
                        HistoryEventTable { events: data.history_ledger() }
                        div { class: "source-note", "Raw memory, provenance content, and locator content are intentionally absent from this public bundle." }
                    }
                }
                aside { class: "history-sidebar",
                    div { class: "content-panel selected-state-panel",
                        div { class: "panel-title-row", div { p { class: "panel-kicker", "SELECTED STATE" } h2 { "State {head.delta.sequence}" } }, StatusBadge { label: "CURRENT / CANONICAL".to_owned(), tone: "verified".to_owned() } }
                        CopyValue { label: "current state root".to_owned(), value: head.next_state_root.clone(), compact: false }
                        dl { class: "sidebar-readout-list",
                            ReadoutRow { label: "Status".to_owned(), value: "canonical".to_owned(), detail: "local evidence head".to_owned() }
                            ReadoutRow { label: "State Root".to_owned(), value: short_hash(&head.next_state_root, 14, 8), detail: "committed root".to_owned() }
                            ReadoutRow { label: "Transition ID".to_owned(), value: short_hash(&head.transition_id, 14, 8), detail: "commitment-bound".to_owned() }
                            ReadoutRow { label: "Previous Root".to_owned(), value: short_hash(&head.delta.prev_state_root, 14, 8), detail: "canonical predecessor".to_owned() }
                            ReadoutRow { label: "Registry".to_owned(), value: short_hash(&data.v2.registry.address, 14, 8), detail: "local Solidity instance".to_owned() }
                            ReadoutRow { label: "Block / tx".to_owned(), value: "NOT AVAILABLE IN THIS BUNDLE".to_owned(), detail: "local revm observation".to_owned() }
                        }
                        Link { class: "button button-secondary button-full", to: AppRoute::Transition { sequence: head.delta.sequence }, "Open forensic detail ", Icon { name: IconName::Arrow, size: 15 } }
                    }
                    div { class: "content-panel authority-history-panel",
                        div { class: "panel-kicker", "AUTHORITY HISTORY" }
                        p { class: "panel-subtitle", "Authorizers and configuration changes over time." }
                        for authority in data.v2.authorization_history.iter() {
                            div { class: "authority-row", key: "{authority.config_nonce}-{authority.authorizer}", span { class: "authority-index", "{authority.config_nonce}" }, div { strong { "{authority.label.as_deref().unwrap_or(\"authority\")}" }, code { "{short_hash(&authority.authorizer, 10, 6)}" } } }
                        }
                        p { class: "small-note", "Rotation is represented by configNonce in the Demo Space V2 evidence. Transition 3 uses the rotated authorizer and carries separately recoverable EIP-712 proof material." }
                    }
                }
            }
            section { class: "section-wrap history-bottom-grid",
                div { class: "content-panel history-stats-panel", div { class: "panel-kicker", "LINEAGE STATISTICS" }, p { class: "panel-subtitle", "Key metrics for this space." }, div { class: "history-stat-grid", div { strong { "{data.v2.transitions.len()}" }, span { "observed states" } }, div { strong { "{data.v2.authorization_history.len().saturating_sub(1)}" }, span { "authority changes" } }, div { strong { "0" }, span { "lineage gaps" } }, div { strong { "0" }, span { "rejected forks" } } } }
                div { class: "content-panel history-verification-panel", div { class: "panel-kicker", "VERIFICATION SUMMARY" }, p { class: "panel-subtitle", "Continuous integrity checks on the canonical lineage." }, CheckRow { label: "Sequence continuity".to_owned(), value: "ALL STATES VALID".to_owned(), tone: history_verdict_tone.to_owned() } CheckRow { label: "Predecessor continuity".to_owned(), value: "MATCH".to_owned(), tone: history_verdict_tone.to_owned() } CheckRow { label: "Authority continuity".to_owned(), value: "TIMELINE BOUND".to_owned(), tone: history_verdict_tone.to_owned() } CheckRow { label: "Final head match".to_owned(), value: if data.report.is_some() { "MATCH".to_owned() } else { "NOT VERIFIED".to_owned() }, tone: history_verdict_tone.to_owned() } }
                div { class: "content-panel history-source-panel", div { class: "panel-kicker", "DATA SOURCE" }, StatusBadge { label: "LOCAL EVIDENCE".to_owned(), tone: "observed".to_owned() }, ReadoutRow { label: "Source".to_owned(), value: "Demo Space V2".to_owned(), detail: "Rust/revm replay".to_owned() }, ReadoutRow { label: "Sepolia".to_owned(), value: "OBSERVED SEPARATELY".to_owned(), detail: "not this local history".to_owned() } }
            }
        }
    }
}

#[component]
pub fn TransitionPage(data: UiData, sequence: u64) -> Element {
    let transition = data.transition(sequence);
    let report = data.report.as_ref();
    let authorization_proof = data
        .v2
        .authorization_proofs
        .iter()
        .find(|proof| proof.sequence == transition.delta.sequence);
    let active_authority = data
        .v2
        .authorization_history
        .iter()
        .filter_map(|authority| {
            authority
                .effective_from_sequence
                .filter(|effective| *effective <= transition.delta.sequence)
                .map(|effective| (effective, authority))
        })
        .max_by_key(|(effective, _)| *effective)
        .map(|(_, authority)| authority);
    let transition_check = if report.is_some() {
        "MATCH"
    } else {
        "NOT VERIFIED"
    };
    let transition_tone = if report.is_some() {
        "verified"
    } else {
        "neutral"
    };
    rsx! {
        div { class: "page workspace-page transition-page",
            div { class: "history-context-bar transition-context-bar", span { class: "context-label", "SPACE" }, code { "{short_hash(&data.v2.registry.space_id, 10, 6)}" }, span { "•" }, span { "Transition {transition.delta.sequence}" }, span { "•" }, span { "Demo Space V2 / local evidence" } }
            Link { class: "detail-back", to: AppRoute::History {}, "← Back to History" }
            PageHeader { kicker: "TRANSITION DETAIL".to_owned(), title: format!("Transition #{}", transition.delta.sequence), description: "Inspect the fields that made this transition part of the recorded history.".to_owned(), source: "DEMO SPACE V2 / LOCAL REVM".to_owned() }
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
                    CheckRow { label: "Sequence continuity".to_owned(), value: transition_check.to_owned(), tone: transition_tone.to_owned() }
                    CheckRow { label: "Predecessor continuity".to_owned(), value: transition_check.to_owned(), tone: transition_tone.to_owned() }
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
                        ReadoutRow { label: "Block / transaction".to_owned(), value: "NOT AVAILABLE IN THIS BUNDLE".to_owned(), detail: "local revm has no public block receipt".to_owned() }
                        ReadoutRow { label: "Authority".to_owned(), value: active_authority.map(|authority| short_hash(&authority.authorizer, 14, 8)).unwrap_or_else(|| "NOT AVAILABLE IN THIS BUNDLE".to_owned()), detail: active_authority.and_then(|authority| authority.effective_from_sequence).map(|effective| format!("configNonce window from sequence {effective}")).unwrap_or_else(|| "effective authority window unavailable".to_owned()) }
                        ReadoutRow { label: "Authorization".to_owned(), value: if authorization_proof.is_some() { "EOA_SIGNATURES_VERIFIED".to_owned() } else { "NOT_INCLUDED".to_owned() }, detail: "independent proof status for this sequence".to_owned() }
                    }
                }
            }
            section { class: "section-wrap content-panel",
                div { class: "panel-kicker", "AUTHORIZATION PROOF" }
                if let Some(proof) = authorization_proof {
                    div { class: "commitment-table",
                        CommitmentRow { label: "configNonce".to_owned(), value: proof.config_nonce.map(|nonce| nonce.to_string()).unwrap_or_else(|| "NOT INCLUDED".to_owned()) }
                        CommitmentRow { label: "domainSeparator".to_owned(), value: proof.domain_separator.clone() }
                        CommitmentRow { label: "signingDigest".to_owned(), value: proof.signing_digest.clone() }
                        CommitmentRow { label: "signature".to_owned(), value: proof.signature.clone() }
                    }
                    p { class: "small-note", "The Rust independent verifier recomputes the EIP-712 domain and digest, recovers the signer, and checks it against the effective authority window. This proof is scoped to the local Demo Space V2 bundle." }
                } else {
                    p { class: "small-note", "NOT AVAILABLE IN THIS BUNDLE. The protocol-corpus projection carries authority records without transition-level signatures." }
                }
            }
            section { class: "section-wrap detail-footer-row", Link { class: "button button-secondary", to: AppRoute::History {}, "← Back to History" }, Link { class: "button button-primary", to: AppRoute::LabScenario { scenario: "silent-rollback".to_owned() }, "Test a stale predecessor →" } }
        }
    }
}

#[component]
pub fn LabPage(data: UiData, initial_scenario: Scenario) -> Element {
    let mut selected = use_signal(|| initial_scenario);
    let mut attempted = use_signal(|| false);
    let busy = use_signal(|| false);
    let mut state = use_signal(|| RollbackUiState::Ready);
    let mut source = use_signal(|| "PUBLISHED LOCAL REVM".to_owned());
    let detail = use_signal(|| "PUBLISHED EVIDENCE / BAD_PREVIOUS_STATE".to_owned());
    let selected_scenario = selected();
    let was_attempted = attempted();
    let is_busy = busy();
    let current_state = state();
    let result_detail = detail();
    let current_source = source();
    let stale_predecessor = data
        .v2
        .attack
        .as_ref()
        .and_then(|attack| attack.stale_predecessor.clone())
        .unwrap_or_else(|| data.restored_snapshot().snapshot_commitment.clone());
    let attempt_sequence = data
        .v2
        .attack
        .as_ref()
        .and_then(|attack| attack.attempted_sequence)
        .unwrap_or(4);
    let local_evidence = data.evidence_json();
    let run_rollback = {
        let mut attempted = attempted;
        let state = state;
        let source_signal = source;
        let detail_signal = detail;
        let evidence = local_evidence.clone();
        let stale_root = stale_predecessor.clone();
        move |_| {
            attempted.set(true);
            match verify_evidence_json(&evidence) {
                Ok(report) if report.verdict == "VERIFIED" && report.attack_evidence == "PASS" => {
                    let mut state = state;
                    let mut source = source_signal;
                    let mut detail = detail_signal;
                    detail.set(format!(
                        "PUBLISHED LOCAL REVM / BAD_PREVIOUS_STATE / attempted sequence {attempt_sequence} / stale root {} from transition 1",
                        short_hash(&stale_root, 10, 8)
                    ));
                    source.set("PUBLISHED LOCAL REVM / RUST-WASM VERIFIED".to_owned());
                    state.set(RollbackUiState::EvidenceRejected);
                }
                Ok(_) => {
                    let mut state = state;
                    let mut source = source_signal;
                    let mut detail = detail_signal;
                    detail.set("LOCAL EVIDENCE / ATTACK RECORD NOT VERIFIED".to_owned());
                    source.set("LOCAL EVIDENCE / VERIFICATION FAILED".to_owned());
                    state.set(RollbackUiState::EvidenceInvalid);
                }
                Err(reason) => {
                    let mut state = state;
                    let mut source = source_signal;
                    let mut detail = detail_signal;
                    detail.set(format!("LOCAL EVIDENCE / REJECTED / {reason}"));
                    source.set("LOCAL EVIDENCE / VERIFICATION FAILED".to_owned());
                    state.set(RollbackUiState::EvidenceInvalid);
                }
            }
        }
    };
    let probe_sepolia = {
        let mut attempted = attempted;
        let mut busy = busy;
        let mut state = state;
        let source_signal = source;
        let detail_signal = detail;
        let stale_predecessor = stale_predecessor.clone();
        move |_| {
            attempted.set(true);
            busy.set(true);
            state.set(RollbackUiState::ProbingSepolia);
            let stale_predecessor = stale_predecessor.clone();
            let mut state = state;
            let mut source = source_signal;
            let mut detail = detail_signal;
            spawn(async move {
                match live_silent_rollback(&stale_predecessor).await {
                    LiveRollbackOutcome::Rejected {
                        sequence,
                        canonical_root,
                        block_tag,
                        block_number,
                        block_hash,
                    } => {
                        detail.set(format!(
                            "SEPOLIA / {SEPOLIA_PROVIDER_LABEL} / {} BLOCK {block_number} / {} / BAD_PREVIOUS_STATE / attempted sequence {sequence} / observed head {}",
                            block_tag.to_uppercase(),
                            short_hash(&block_hash, 10, 8),
                            short_hash(&canonical_root, 10, 8),
                        ));
                        source.set(format!(
                            "SEPOLIA / {SEPOLIA_PROVIDER_LABEL} / LIVE / {} BLOCK",
                            block_tag.to_uppercase()
                        ));
                        state.set(RollbackUiState::LiveRejected);
                    }
                    LiveRollbackOutcome::Unavailable { reason } => {
                        detail.set(format!(
                            "SEPARATE SEPOLIA PROBE / UNAVAILABLE / {reason} / local Demo Space V2 evidence remains unchanged"
                        ));
                        source.set(format!("SEPOLIA / {SEPOLIA_PROVIDER_LABEL} UNAVAILABLE"));
                        state.set(RollbackUiState::LiveUnavailable);
                    }
                    LiveRollbackOutcome::Unexpected { reason } => {
                        detail.set(format!("UNEXPECTED LIVE RESULT / {reason}"));
                        source.set("SEPOLIA / UNEXPECTED LIVE RESULT".to_owned());
                        state.set(RollbackUiState::UnexpectedLive);
                    }
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
    let page_source = if selected_scenario == Scenario::SilentRollback {
        current_source.as_str()
    } else {
        "PUBLISHED CORPUS"
    };
    let result_tone = if selected_scenario == Scenario::SemanticPoisoning {
        "warning"
    } else {
        match current_state {
            RollbackUiState::Ready => "observed",
            RollbackUiState::LiveRejected | RollbackUiState::EvidenceRejected => "danger",
            RollbackUiState::ProbingSepolia
            | RollbackUiState::LiveUnavailable
            | RollbackUiState::EvidenceInvalid
            | RollbackUiState::UnexpectedLive => "warning",
        }
    };
    let result_panel_class = match current_state {
        RollbackUiState::LiveRejected | RollbackUiState::EvidenceRejected => {
            "result-panel result-panel-rejected"
        }
        RollbackUiState::LiveUnavailable
        | RollbackUiState::EvidenceInvalid
        | RollbackUiState::UnexpectedLive => "result-panel result-panel-warning",
        RollbackUiState::Ready | RollbackUiState::ProbingSepolia => "result-panel",
    };
    let result_label = match selected_scenario {
        Scenario::SemanticPoisoning => "OUT OF SCOPE",
        Scenario::SilentRollback => match current_state {
            RollbackUiState::Ready => "READY / LOCAL EVIDENCE",
            RollbackUiState::ProbingSepolia => "PROBING SEPOLIA",
            RollbackUiState::LiveRejected => "LIVE RPC: REJECTED",
            RollbackUiState::EvidenceRejected => "LOCAL EVIDENCE: REJECTED",
            RollbackUiState::LiveUnavailable => "SEPOLIA PROBE: UNAVAILABLE",
            RollbackUiState::EvidenceInvalid => "LOCAL EVIDENCE: INVALID",
            RollbackUiState::UnexpectedLive => "UNEXPECTED LIVE RESULT",
        },
        _ => "REJECT EXPECTED",
    };
    let replay_class = if was_attempted {
        "audit-path-step audit-path-step-complete"
    } else {
        "audit-path-step"
    };
    let verdict_class = match current_state {
        RollbackUiState::EvidenceRejected | RollbackUiState::LiveRejected => {
            "audit-path-step audit-path-step-rejected"
        }
        RollbackUiState::LiveUnavailable
        | RollbackUiState::EvidenceInvalid
        | RollbackUiState::UnexpectedLive => "audit-path-step audit-path-step-warning",
        RollbackUiState::ProbingSepolia => "audit-path-step audit-path-step-active",
        RollbackUiState::Ready => "audit-path-step",
    };
    let verdict_label = match current_state {
        RollbackUiState::EvidenceRejected | RollbackUiState::LiveRejected => "REJECTED",
        RollbackUiState::LiveUnavailable => "UNAVAILABLE",
        RollbackUiState::EvidenceInvalid => "INVALID",
        RollbackUiState::UnexpectedLive => "UNEXPECTED",
        RollbackUiState::ProbingSepolia => "READING",
        RollbackUiState::Ready => "AWAITING RUN",
    };
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "TAMPERING LAB / FALSIFICATION WORKSPACE".to_owned(), title: "Tampering Lab".to_owned(), description: "Restore an older snapshot, then try to continue the newer history. The result explains exactly what the registry accepts or rejects.".to_owned(), source: page_source.to_owned() }
            section { class: "audit-path", aria_label: "Silent Rollback audit path",
                div { class: "audit-path-step audit-path-step-complete", span { "01" }, div { strong { "Restore" }, small { "Snapshot {data.restored_snapshot().sequence}" } } }
                span { class: "audit-path-arrow", aria_hidden: "true", "→" }
                div { class: "audit-path-step audit-path-step-complete", span { "02" }, div { strong { "Compare" }, small { "stale root / head" } } }
                span { class: "audit-path-arrow", aria_hidden: "true", "→" }
                div { class: replay_class, span { "03" }, div { strong { "Replay" }, small { "local Rust/revm" } } }
                span { class: "audit-path-arrow", aria_hidden: "true", "→" }
                div { class: verdict_class, span { "04" }, div { strong { "Verdict" }, small { "{verdict_label}" } } }
            }
            div { class: "lab-workspace",
                aside { class: "scenario-list content-panel",
                    div { class: "panel-kicker", "CHOOSE A TEST" }
                    for scenario in Scenario::ALL {
                        button { class: if scenario == selected_scenario { "scenario-item scenario-item-active" } else { "scenario-item" }, aria_pressed: scenario == selected_scenario, onclick: move |_| { selected.set(scenario); attempted.set(false); state.set(RollbackUiState::Ready); source.set("PUBLISHED LOCAL REVM".to_owned()); },
                            span { class: "scenario-index", "{scenario.evidence_kind()}" }
                            span { strong { "{scenario.title()}" }, small { "{scenario.description()}" } }
                            span { class: "scenario-chevron", "→" }
                        }
                    }
                }
                section { class: "attack-workspace content-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "WHAT WE ARE COMPARING" } h2 { "{selected_scenario.title()}" } }, StatusBadge { label: result_label.to_owned(), tone: result_tone.to_owned() } }
                    if selected_scenario == Scenario::SilentRollback {
                        div { class: "rollback-story",
                            div { class: "rollback-column", span { class: "panel-kicker", "SYNTHETIC FIXTURE / CANONICAL SNAPSHOT" }, div { class: "snapshot-stack", for snapshot in data.fixture.snapshots.iter() { if snapshot.sequence == data.canonical_snapshot().sequence { strong { "SNAPSHOT {snapshot.sequence} / COMMITTED" } } else { span { "SNAPSHOT {snapshot.sequence}" } } small { class: "snapshot-label", "{snapshot.visible_label.as_deref().unwrap_or(\"Label unavailable\")}" } if snapshot.sequence != data.canonical_snapshot().sequence { span { "↓" } } } }, code { "protocol root {short_hash(&data.head().next_state_root, 12, 8)}" } }
                            div { class: "rollback-operator", "→" }
                            div { class: "rollback-column rollback-column-attempt", span { class: "panel-kicker", "LOCAL RESTORE ATTEMPT" }, div { class: "snapshot-stack snapshot-stack-danger", strong { "RESTORE SNAPSHOT {data.restored_snapshot().sequence}" }, small { class: "snapshot-label", "{data.restored_snapshot().visible_label.as_deref().unwrap_or(\"Label unavailable\")}" }, span { "↓" }, span { "ATTEMPT TRANSITION {data.v2.attack.as_ref().and_then(|attack| attack.attempted_sequence).unwrap_or(4)}" } }, code { "stale predecessor {short_hash(&stale_predecessor, 12, 8)}" } }
                        }
                        div { class: "attack-action-row",
                            button { class: "button button-danger", onclick: run_rollback, disabled: is_busy, "Run Silent Rollback ", Icon { name: IconName::Arrow, size: 15 } }
                            span { class: "attack-action-source", "Runs published local evidence; no transaction is sent." }
                        }
                    }
                }
                aside { class: "lab-result-column",
                    if selected_scenario == Scenario::SilentRollback {
                        div { class: result_panel_class, aria_live: "polite",
                            if was_attempted && current_state != RollbackUiState::Ready && current_state != RollbackUiState::ProbingSepolia {
                                div { class: "result-heading", span { class: "result-symbol", if current_state == RollbackUiState::LiveRejected || current_state == RollbackUiState::EvidenceRejected { "×" } else { "!" } }, div { strong { "{result_label}" }, small { "{result_detail}" } } }
                                if current_state == RollbackUiState::EvidenceRejected {
                                    h3 { "The published Rust/revm run rejected transition {attempt_sequence}." }
                                    p { "The browser has independently replayed the evidence: the stale predecessor is transition 1's root, while the canonical local head is transition 3. Run `cargo xtask verify` to regenerate the SQLite fixture and execute the Solidity bytecode again." }
                                } else if current_state == RollbackUiState::LiveRejected {
                                    h3 { "The existing Sepolia registry rejected this separate stale-predecessor probe." }
                                    p { "This read-only observation uses the previously deployed Sepolia space. It is not the Demo Space V2 history shown here, and no transaction was broadcast." }
                                } else if current_state == RollbackUiState::LiveUnavailable {
                                    h3 { "The optional Sepolia read did not complete." }
                                    p { "No live result is claimed. The local Demo Space V2 evidence can still be replayed independently." }
                                } else if current_state == RollbackUiState::EvidenceInvalid {
                                    h3 { "The local rollback evidence did not pass verification." }
                                    p { "MemoryLineage does not display an expected rejection when the imported attack record or lineage fails its verifier." }
                                } else {
                                    h3 { "The live response did not match the expected rejection." }
                                    p { "MemoryLineage does not replace an unexpected RPC result with a success claim. Review the local evidence separately on Verify and Public Evidence." }
                                }
                                div { class: "action-row",
                                    button { class: "button button-quiet", onclick: move |_| { attempted.set(false); state.set(RollbackUiState::Ready); source.set("PUBLISHED LOCAL REVM".to_owned()); }, "Reset rehearsal" }
                                    if current_state == RollbackUiState::EvidenceRejected || current_state == RollbackUiState::LiveUnavailable {
                                        button { class: "button button-secondary", onclick: probe_sepolia, disabled: is_busy, if is_busy { "Reading Sepolia…" } else { "Probe separate Sepolia" } }
                                    }
                                }
                            } else if current_state == RollbackUiState::ProbingSepolia {
                                div { class: "result-heading result-heading-ready", span { class: "result-symbol", "◉" }, div { strong { "READING SEPOLIA" }, small { "No transaction is sent; this optional probe is separate from Demo Space V2." } } }
                                h3 { "Checking the existing registry's head and response." }
                            } else {
                                div { class: "result-heading result-heading-ready", span { class: "result-symbol", "◉" }, div { strong { "READY TO VERIFY" }, small { "The primary action replays the unified local Rust/revm evidence. A separate control reads the existing Sepolia space." } } }
                                h3 { "The older snapshot should not become the next state." }
                                p { "The sample fixture is synthetic and public. The bundle contains commitments only; the local contract execution is reproducible with the Rust CLI." }
                                button { class: "button button-secondary button-full", onclick: probe_sepolia, disabled: is_busy, if is_busy { "Reading Sepolia…" } else { "Probe separate Sepolia" } }
                            }
                        }
                        div { class: "content-panel lab-proof-panel",
                            div { class: "panel-kicker", "WHAT THIS PROVES" }
                            div { class: "lab-proof-row", strong { "Stale predecessor rejected" }, span { "The attempted root is not the canonical head." } }
                            div { class: "lab-proof-row", strong { "Canonical succession enforced" }, span { "Sequence and parent state are checked together." } }
                            div { class: "lab-proof-row", strong { "Raw memory stays private" }, span { "Only commitments and metadata enter the evidence." } }
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
                        div { class: "content-panel content-panel-muted", div { class: "panel-kicker", "NEXT STEP" }, h3 { "Replay it elsewhere." }, p { "A browser rejection matters when the same bundle can be checked independently." }, Link { class: "text-link", to: AppRoute::Verify {}, "Open the Verify workspace →" } }
            }
        }
    }
}

#[component]
pub fn VerifyPage(data: UiData) -> Element {
    let default_json = data.evidence_json();
    let tampered_json = data.tampered_json();
    let default_receipt_json = data.recovery_receipt_json();
    let tampered_receipt_json = data.tampered_recovery_receipt_json();
    let report = use_signal(|| data.report.clone());
    let error = use_signal(|| None::<String>);
    let source = use_signal(|| "PUBLISHED V2 EVIDENCE".to_owned());
    let imported_name = use_signal(|| None::<String>);
    let announcement =
        use_signal(|| "Published evidence is loaded in the Rust/WASM verifier.".to_owned());
    let receipt_report = use_signal(|| data.verify_recovery_receipt().ok());
    let receipt_error = use_signal(|| None::<String>);
    let receipt_source = use_signal(|| "PUBLISHED RECOVERY RECEIPT".to_owned());
    let receipt_file = use_signal(|| None::<String>);
    let receipt_announcement = use_signal(|| {
        "The current snapshot decision is bound to the published V2 bundle.".to_owned()
    });
    let current_report = report();
    let current_error = error();
    let current_source = source();
    let current_file = imported_name();
    let current_receipt_report = receipt_report();
    let current_receipt_error = receipt_error();
    let current_receipt_source = receipt_source();
    let current_receipt_file = receipt_file();
    let current_receipt_verified =
        current_receipt_error.is_none() && current_receipt_report.is_some();
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
    let receipt_summary = if let Some(reason) = current_receipt_error.as_deref() {
        format!("Independent Rust receipt verification rejected the decision: {reason}.")
    } else if let Some(receipt_report) = current_receipt_report.as_ref() {
        format!(
            "{} / {} / decision {}.",
            receipt_report.classification,
            receipt_report.recommended_action,
            short_hash(&receipt_report.decision_id, 14, 8)
        )
    } else {
        "No recovery decision result is available.".to_owned()
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
    let export_receipt_action = {
        let receipt_json = default_receipt_json.clone();
        let mut receipt_report = receipt_report;
        let mut receipt_error = receipt_error;
        let mut receipt_source = receipt_source;
        let mut receipt_file = receipt_file;
        let mut receipt_announcement = receipt_announcement;
        let evidence_json = default_json.clone();
        move |_| {
            let result = download_json("memorylineage-recovery-receipt-v1.json", &receipt_json);
            match verify_recovery_receipt_json(&receipt_json, &evidence_json) {
                Ok(value) => {
                    receipt_report.set(Some(value));
                    receipt_error.set(None);
                    receipt_source.set("EXPORTED RECOVERY RECEIPT".to_owned());
                    receipt_file.set(None);
                    receipt_announcement.set(match result {
                        Ok(()) => "Recovery decision receipt exported and verified in Rust/WASM."
                            .to_owned(),
                        Err(reason) => {
                            format!("Receipt is ready; browser download unavailable: {reason}.")
                        }
                    });
                }
                Err(reason) => {
                    receipt_report.set(None);
                    receipt_error.set(Some(reason.clone()));
                    receipt_announcement.set(format!("Receipt verification failed: {reason}"));
                }
            }
        }
    };
    let tamper_receipt_action = {
        let receipt_json = tampered_receipt_json.clone();
        let evidence_json = default_json.clone();
        let mut receipt_report = receipt_report;
        let mut receipt_error = receipt_error;
        let mut receipt_source = receipt_source;
        let mut receipt_file = receipt_file;
        let mut receipt_announcement = receipt_announcement;
        move |_| match verify_recovery_receipt_json(&receipt_json, &evidence_json) {
            Ok(value) => {
                receipt_report.set(Some(value));
                receipt_error.set(None);
                receipt_source.set("TAMPERED RECEIPT / UNEXPECTED PASS".to_owned());
                receipt_file.set(None);
                receipt_announcement.set("Tampered receipt unexpectedly passed.".to_owned());
            }
            Err(reason) => {
                receipt_report.set(None);
                receipt_error.set(Some(reason.clone()));
                receipt_source.set("TAMPERED RECEIPT".to_owned());
                receipt_file.set(None);
                receipt_announcement.set(format!("Receipt tamper check: REJECTED / {reason}"));
            }
        }
    };
    let restore_receipt_action = {
        let original_report = data.verify_recovery_receipt().ok();
        let mut receipt_report = receipt_report;
        let mut receipt_error = receipt_error;
        let mut receipt_source = receipt_source;
        let mut receipt_file = receipt_file;
        let mut receipt_announcement = receipt_announcement;
        move |_| {
            receipt_report.set(original_report.clone());
            receipt_error.set(None);
            receipt_source.set("PUBLISHED RECOVERY RECEIPT".to_owned());
            receipt_file.set(None);
            receipt_announcement.set("Original recovery receipt restored and verified.".to_owned());
        }
    };
    let import_receipt_action = {
        let evidence_json = default_json.clone();
        let mut receipt_report = receipt_report;
        let mut receipt_error = receipt_error;
        let mut receipt_source = receipt_source;
        let mut receipt_file = receipt_file;
        let mut receipt_announcement = receipt_announcement;
        move |event: FormEvent| {
            if let Some(file) = event.files().into_iter().next() {
                let name = file.name();
                receipt_file.set(Some(name.clone()));
                let evidence_json = evidence_json.clone();
                spawn(async move {
                    match file.read_string().await {
                        Ok(json) => match verify_recovery_receipt_json(&json, &evidence_json) {
                            Ok(value) => {
                                receipt_report.set(Some(value));
                                receipt_error.set(None);
                                receipt_source.set("IMPORTED RECOVERY RECEIPT".to_owned());
                                receipt_announcement.set(format!("Imported {name}: VERIFIED."));
                            }
                            Err(reason) => {
                                receipt_report.set(None);
                                receipt_error.set(Some(reason.clone()));
                                receipt_source.set("IMPORTED RECOVERY RECEIPT".to_owned());
                                receipt_announcement
                                    .set(format!("Imported {name}: REJECTED / {reason}"));
                            }
                        },
                        Err(_) => {
                            receipt_report.set(None);
                            receipt_error.set(Some("FILE_READ_FAILED".to_owned()));
                            receipt_announcement.set(format!("Imported {name}: file read failed."));
                        }
                    }
                });
            }
        }
    };
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "VERIFY / PORTABLE EVIDENCE".to_owned(), title: "Verify Evidence".to_owned(), description: "Export a bundle, change one commitment, and confirm that verification catches the change.".to_owned(), source: current_source.clone() }
            div { class: "verify-workspace",
                aside { class: "content-panel evidence-actions-panel",
                    div { class: "panel-kicker", "EVIDENCE WORKBENCH" }
                    h2 { "Start with the evidence bundle." }
                    p { "It contains the history fingerprints and approval records needed for replay. It never contains raw memory." }
                    div { class: "evidence-facts", div { span { "TRANSITIONS" }, strong { "{data.v2.transitions.len()}" } }, div { span { "SCHEMA" }, strong { "V2" } }, div { span { "RAW MEMORY" }, strong { "OFF-CHAIN" } } }
                    button { class: "button button-primary button-wide", onclick: export_action, "Export evidence.json" }
                    label { class: "file-drop", strong { "Import a saved evidence bundle" }, span { "V1 or V2 public evidence bundle" }, input { r#type: "file", accept: "application/json,.json", onchange: import_action } }
                    div { class: "source-line source-line-vertical", span { class: "source-dot source-dot-blue" }, strong { "{current_source}" }, if let Some(name) = current_file.as_deref() { span { "{name}" } } }
                    div { class: "verify-guide", aria_label: "How to verify evidence",
                        div { span { "1" }, strong { "Load" }, small { "use the published or imported bundle" } }
                        div { span { "2" }, strong { "Tamper" }, small { "change one commitment" } }
                        div { span { "3" }, strong { "Restore" }, small { "confirm the original is VERIFIED" } }
                    }
                }
                section { class: "content-panel verification-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "VERIFICATION SUMMARY" }, h2 { "Independent replay" } }, StatusBadge { label: if is_verified { "VERIFIED".to_owned() } else { "REJECTED".to_owned() }, tone: if is_verified { "verified".to_owned() } else { "danger".to_owned() } } }
                    div { class: if is_verified { "verification-banner verification-banner-pass" } else { "verification-banner verification-banner-fail" }, aria_live: "polite", span { class: "verification-icon", if is_verified { "✓" } else { "×" } }, div { strong { if is_verified { "VERIFIED" } else { "REJECTED" } }, p { "{verification_summary}" } } }
                    div { class: "check-table",
                        if let Some(current_report) = current_report.as_ref() {
                            CheckRow { label: "Schema".to_owned(), value: current_report.schema.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Privacy boundary".to_owned(), value: current_report.privacy_boundary.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Attack evidence".to_owned(), value: current_report.attack_evidence.clone(), tone: if current_report.attack_evidence == "PASS" { "verified".to_owned() } else { "neutral".to_owned() } }
                        } else { CheckRow { label: "Imported bundle".to_owned(), value: current_error.clone().unwrap_or_else(|| "NO RESULT".to_owned()), tone: "danger".to_owned() } }
                    }
                }
            }
            div { class: "verify-detail-grid",
                section { class: "content-panel verification-detail-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "VERIFICATION RESULTS" }, h2 { "Detailed checks" } }, StatusBadge { label: if is_verified { "ALL CHECKS PASSED".to_owned() } else { "VERIFICATION FAILED".to_owned() }, tone: if is_verified { "verified".to_owned() } else { "danger".to_owned() } } }
                    p { class: "panel-subtitle", "Detailed checks performed on the loaded evidence bundle." }
                    div { class: "check-table",
                        if let Some(current_report) = current_report.as_ref() {
                            CheckRow { label: "Schema".to_owned(), value: current_report.schema.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Registry identity".to_owned(), value: current_report.registry_identity.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Transition IDs".to_owned(), value: current_report.transition_ids.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "State roots".to_owned(), value: current_report.state_roots.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Sequence continuity".to_owned(), value: current_report.sequence_continuity.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Predecessor continuity".to_owned(), value: current_report.predecessor_continuity.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Authority timeline".to_owned(), value: current_report.authority_history.clone(), tone: if current_report.authority_history == "TIMELINE_BOUND" { "verified".to_owned() } else { "observed".to_owned() } }
                            CheckRow { label: "Transition authorization proof".to_owned(), value: current_report.authorization_proof.clone(), tone: if current_report.authorization_proof == "EOA_SIGNATURES_VERIFIED" { "verified".to_owned() } else { "warning".to_owned() } }
                            CheckRow { label: "Privacy boundary".to_owned(), value: current_report.privacy_boundary.clone(), tone: "verified".to_owned() }
                            CheckRow { label: "Attack evidence".to_owned(), value: current_report.attack_evidence.clone(), tone: if current_report.attack_evidence == "PASS" { "verified".to_owned() } else { "neutral".to_owned() } }
                        } else { CheckRow { label: "Imported bundle".to_owned(), value: current_error.clone().unwrap_or_else(|| "NO RESULT".to_owned()), tone: "danger".to_owned() } }
                    }
                }
                div { class: "verify-tamper-column",
            section { class: "tamper-workbench content-panel",
                div { class: "panel-title-row", div { p { class: "panel-kicker", "TAMPER WORKBENCH" }, h2 { "Change one commitment." } }, StatusBadge { label: "FALSIFIABLE".to_owned(), tone: "warning".to_owned() } }
                p { "The browser changes the first locatorCommitment in a copy of the loaded evidence. A valid bundle must fail closed before it can be trusted." }
                div { class: "tamper-grid", div { class: "tamper-value", span { "ORIGINAL" }, code { "locatorCommitment / {short_hash(&data.v2.transitions[0].delta.locator_commitment, 14, 8)}" } }, div { class: "tamper-arrow", "→" }, div { class: "tamper-value tamper-value-danger", span { "TAMPERED COPY" }, code { "locatorCommitment / 0xffff…ffff" } } }
                div { class: "action-row", button { class: "button button-danger", onclick: tamper_action, "Tamper one field" }, button { class: "button button-secondary", onclick: restore_action, "Restore original" } }
                p { class: "announcement", "{announcement}" }
            }
            section { class: "content-panel verify-results-detail",
                div { class: "panel-kicker", "BROWSER RESULT" }
                if let Some(reason) = current_error.as_deref() {
                    div { class: "verification-banner verification-banner-fail", aria_live: "polite", span { class: "verification-icon", "×" }, div { strong { "VERIFICATION FAILED" }, p { "{reason}" } } }
                } else {
                    div { class: "verification-banner verification-banner-pass", aria_live: "polite", span { class: "verification-icon", "✓" }, div { strong { "VERIFIED" }, p { "The original bundle remains valid in the Rust/WASM verifier." } } }
                }
                p { "The browser verifier recomputes the bundle. A changed commitment cannot silently preserve the original transition identity." }
                Link { class: "button button-secondary button-full", to: AppRoute::Evidence {}, "Open Public Evidence ", Icon { name: IconName::Arrow, size: 15 } }
            }
                }
            }
            section { class: "section-wrap recovery-receipt-workbench",
                div { class: "panel-title-row",
                    div { p { class: "panel-kicker", "RECOVERY DECISION RECEIPT" }, h2 { "Can another process verify the resume decision?" } }
                    StatusBadge { label: if current_receipt_verified { "VERIFIED".to_owned() } else { "REJECTED".to_owned() }, tone: if current_receipt_verified { "verified".to_owned() } else { "danger".to_owned() } }
                }
                p { "A Recovery Decision Receipt binds one private snapshot commitment to the replayed canonical head and records the safe action. It contains no raw memory and does not claim to enforce a production runtime." }
                div { class: "recovery-receipt-grid",
                    div { class: "content-panel recovery-receipt-summary",
                        div { class: "panel-kicker", "DECISION READOUT" }
                        div { class: if current_receipt_verified { "verification-banner verification-banner-pass" } else { "verification-banner verification-banner-fail" }, aria_live: "polite", span { class: "verification-icon", if current_receipt_verified { "✓" } else { "×" } }, div { strong { if current_receipt_verified { "RECEIPT VERIFIED" } else { "RECEIPT REJECTED" } }, p { "{receipt_summary}" } } }
                        if let Some(receipt_report) = current_receipt_report.as_ref() {
                            div { class: "check-table",
                                CheckRow { label: "Receipt integrity".to_owned(), value: receipt_report.receipt_integrity.clone(), tone: "verified".to_owned() }
                                CheckRow { label: "Evidence bundle".to_owned(), value: receipt_report.evidence_bundle.clone(), tone: "verified".to_owned() }
                                CheckRow { label: "Decision policy".to_owned(), value: receipt_report.policy_id.clone(), tone: "observed".to_owned() }
                                CheckRow { label: "Source class".to_owned(), value: receipt_report.source_class.clone(), tone: "observed".to_owned() }
                                CheckRow { label: "Authority timeline".to_owned(), value: receipt_report.authority_history.clone(), tone: if receipt_report.authority_history == "TIMELINE_BOUND" { "verified".to_owned() } else { "observed".to_owned() } }
                                CheckRow { label: "Transition authorization".to_owned(), value: receipt_report.transition_authorization.clone(), tone: if receipt_report.transition_authorization == "EOA_SIGNATURES_VERIFIED" { "verified".to_owned() } else { "warning".to_owned() } }
                                CheckRow { label: "Protected action".to_owned(), value: receipt_report.recommended_action.clone(), tone: if receipt_report.recommended_action == "RESUME_ALLOWED" { "verified".to_owned() } else { "warning".to_owned() } }
                            }
                            CopyValue { label: "decision ID".to_owned(), value: receipt_report.decision_id.clone(), compact: true }
                        } else if let Some(reason) = current_receipt_error.as_deref() {
                            CheckRow { label: "Failure".to_owned(), value: reason.to_owned(), tone: "danger".to_owned() }
                        }
                    }
                    aside { class: "content-panel recovery-receipt-actions",
                        div { class: "panel-kicker", "PORTABLE ARTIFACT" }
                        h3 { "Export the decision, then falsify it." }
                        p { "The Rust CLI can verify the same receipt outside the browser against the same V2 bundle." }
                        div { class: "action-row action-row-tight",
                            button { class: "button button-primary", onclick: export_receipt_action, "Export receipt" }
                            button { class: "button button-danger", onclick: tamper_receipt_action, "Tamper decision" }
                            button { class: "button button-secondary", onclick: restore_receipt_action, "Restore" }
                        }
                        label { class: "file-drop", strong { "Import recovery receipt" }, span { "Must match the loaded Demo Space V2 bundle" }, input { r#type: "file", accept: "application/json,.json", onchange: import_receipt_action } }
                        div { class: "source-line source-line-vertical", span { class: "source-dot source-dot-blue" }, strong { "{current_receipt_source}" }, if let Some(name) = current_receipt_file.as_deref() { span { "{name}" } } }
                        p { class: "announcement", "{receipt_announcement}" }
                    }
                }
            }
            section { class: "section-wrap verify-bottom-grid",
                div { class: "content-panel", div { class: "panel-kicker", "SECONDARY AUDITOR" }, h3 { "Rust CLI" }, p { "The same evidence can be checked outside the browser with the independent Rust verifier." }, code { class: "command-block", "cargo run -q -p ml-cli -- verify evidence.json" } }
                div { class: "content-panel content-panel-muted", div { class: "panel-kicker", "TRUST MODEL" }, h3 { "Do not trust the dashboard." }, p { "Trust the deterministic bundle, the pinned semantics, and a verifier you can run separately." }, Link { class: "text-link", to: AppRoute::Security {}, "Read the boundary →" } }
            }
        }
    }
}

#[component]
pub fn EvidencePage(data: UiData) -> Element {
    rsx! {
        div { class: "page workspace-page",
            div { class: "evidence-intro-grid",
                PageHeader { kicker: "PUBLIC TECHNICAL EVIDENCE / DEMO SUBMISSION ARTIFACTS".to_owned(), title: "Public Evidence".to_owned(), description: "Trace each visible claim to a deployment, replay, or published artifact.".to_owned(), source: "REPOSITORY EVIDENCE".to_owned() }
                aside { class: "evidence-intro-aside", p { class: "panel-kicker", "EVIDENCE SOURCES" }, p { "Each status points to a repository artifact or a labeled observation." } }
            }
            section { class: "section-wrap evidence-grid evidence-overview-grid",
                div { class: "content-panel evidence-deployment-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "SEPOLIA DEPLOYMENT" }, h2 { "Workspace-owned trust anchor" } }, StatusBadge { label: "PASS".to_owned(), tone: "verified".to_owned() } }
                    dl { class: "readout-list", ReadoutRow { label: "Network".to_owned(), value: format!("Ethereum Sepolia / {}", data.deployment.chain_id), detail: "read-only browser target".to_owned() }, ReadoutRow { label: "Registry".to_owned(), value: short_hash(&data.deployment.registry_address, 14, 8), detail: "contract address".to_owned() }, ReadoutRow { label: "Deployment tx".to_owned(), value: short_hash(&data.deployment.deployment_tx_hash, 14, 8), detail: "receipt recorded".to_owned() }, ReadoutRow { label: "Code hash".to_owned(), value: short_hash(&data.deployment.deployed_code_hash, 14, 8), detail: "deployed bytecode observation".to_owned() }, ReadoutRow { label: "Raw payload".to_owned(), value: if data.deployment.raw_payload_stored { "PRESENT".to_owned() } else { "NOT STORED".to_owned() }, detail: "fixed-size commitments only".to_owned() } }
                }
                div { class: "content-panel evidence-observation-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "CURRENT OBSERVATION" }, h2 { "Second RPC reread" } }, StatusBadge { label: data.reread.verdict.clone(), tone: "verified".to_owned() } }
                    dl { class: "readout-list", ReadoutRow { label: "Provider".to_owned(), value: data.reread.rpc_url.clone(), detail: "published observation".to_owned() }, ReadoutRow { label: "Block".to_owned(), value: data.reread.block_number.to_string(), detail: "observed head block".to_owned() }, ReadoutRow { label: "Observed sequence".to_owned(), value: data.reread.head.sequence.to_string(), detail: "Sepolia reread".to_owned() }, ReadoutRow { label: "Head root".to_owned(), value: short_hash(&data.reread.head.state_root, 14, 8), detail: "headMatches = true".to_owned() } }
                    p { class: "small-note", "This page does not claim provider consensus; it labels the independent reread that is actually published." }
                }
                div { class: "content-panel evidence-rpc-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "RPC OBSERVATION" } h2 { "Read-only source" } }, StatusBadge { label: "MATCH".to_owned(), tone: "verified".to_owned() } }
                    div { class: "rpc-observation-table",
                        div { class: "rpc-observation-row", strong { "Published reread" }, code { "{short_hash(&data.reread.rpc_url, 18, 8)}" }, span { "head {data.reread.head.sequence}" }, StatusBadge { label: data.reread.verdict.clone(), tone: "verified".to_owned() } }
                        div { class: "rpc-observation-row", strong { "Observed block" }, code { "{data.reread.block_number}" }, span { "sequence {data.reread.head.sequence}" }, StatusBadge { label: "OBSERVED".to_owned(), tone: "observed".to_owned() } }
                    }
                    p { class: "small-note", "One published reread is shown here. Multiple-provider consensus is not claimed." }
                }
            }
            div { class: "section-wrap evidence-middle-grid",
                section { class: "content-panel evidence-corpus-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "PUBLISHED ADVERSARIAL CORPUS" } h2 { "Deterministic negative paths" } }, StatusBadge { label: format!("{}/{} REJECTED", data.mutation_rejected, data.mutation_count), tone: "danger".to_owned() } }
                    p { class: "panel-subtitle", "Corpus rows are published execution evidence, separate from the Demo Space V2 incident." }
                    div { class: "corpus-summary-row", div { strong { "{data.bundle.valid_history.len()}" }, span { "valid vectors" } }, div { strong { "{data.mutation_rejected}" }, span { "invalid mutations" } } }
                    div { class: "evidence-table evidence-table-compact",
                        for mutation in data.bundle.mutation_matrix.iter().filter(|mutation| mutation.expected == "REJECT").take(4) {
                            EvidenceTableRow { label: mutation.name.clone(), value: mutation.observed.reason.clone().unwrap_or_else(|| "REJECTED".to_owned()), note: mutation.observed.status.clone() }
                        }
                    }
                    Link { class: "text-link", to: AppRoute::Lab {}, "View full corpus ", Icon { name: IconName::Arrow, size: 14 } }
                }
                section { class: "content-panel evidence-conformance-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "CROSS-IMPLEMENTATION CONFORMANCE" } h2 { "Same rules. Same results." } }, StatusBadge { label: "MATCH".to_owned(), tone: "verified".to_owned() } }
                    div { class: "conformance-table",
                        EvidenceTableRow { label: "transitionId".to_owned(), value: data.conformance.published_evidence.clone(), note: "Solidity / Rust reference / Rust verifier".to_owned() }
                        EvidenceTableRow { label: "nextStateRoot".to_owned(), value: data.conformance.rust_reference.clone(), note: "canonical vector agreement".to_owned() }
                        EvidenceTableRow { label: "Silent Rollback".to_owned(), value: data.conformance.revm_silent_rollback.clone(), note: "BAD_PREVIOUS_STATE".to_owned() }
                        EvidenceTableRow { label: "Authority rotation".to_owned(), value: data.conformance.revm_authority_rotation.clone(), note: "configNonce and authorizer window".to_owned() }
                    }
                    p { class: "small-note", "Pinned vector: {data.conformance.pinned_vector}. Independent verifier: {data.conformance.independent_verifier}." }
                }
            }
            section { class: "section-wrap evidence-submission-grid",
                div { class: "content-panel evidence-submission-card",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "INDEPENDENT CLEAN-CHECKOUT REPRODUCTION" }, h3 { "Run the claims from source." } }, StatusBadge { label: "NOT YET DEMONSTRATED".to_owned(), tone: "neutral".to_owned() } }
                    p { "Automated repository gates are published. An external human rerun is intentionally still unclaimed." }
                    ReadoutRow { label: "Required input".to_owned(), value: "repository URL + README".to_owned(), detail: "no personal walkthrough".to_owned() }
                    ReadoutRow { label: "Result".to_owned(), value: "NOT YET DEMONSTRATED".to_owned(), detail: "human evidence pending".to_owned() }
                }
                div { class: "content-panel evidence-submission-card evidence-artifact-card",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "AVAILABLE ARTIFACTS" }, h3 { "Download and verify." } }, StatusBadge { label: "PUBLISHED".to_owned(), tone: "observed".to_owned() } }
                    div { class: "compact-artifact-list", div { code { "demo_space_v2_evidence.json" }, span { "Demo V2" } }, div { code { "rust_revm_conformance.json" }, span { "CONFORMANCE" } }, div { code { "rust_revm_mutation_matrix.json" }, span { "CORPUS" } }, div { code { "security_assurance_report.json" }, span { "BOUNDED" } } }
                }
                div { class: "content-panel evidence-submission-card evidence-next-card",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "NEXT ACTIONS" }, h3 { "Continue the audit." } }, StatusBadge { label: "OPEN".to_owned(), tone: "verified".to_owned() } }
                    Link { class: "button button-primary button-full", to: AppRoute::Verify {}, "Download Evidence " , Icon { name: IconName::Open, size: 14 } }
                    Link { class: "button button-secondary button-full", to: AppRoute::Reproduce {}, "Open Reproduce " , Icon { name: IconName::Arrow, size: 14 } }
                    Link { class: "button button-secondary button-full", to: AppRoute::PriorWork {}, "View Source Boundary " , Icon { name: IconName::Open, size: 14 } }
                }
            }
            section { class: "section-wrap content-panel",
                div { class: "panel-title-row", div { p { class: "panel-kicker", "PORTABILITY / LOCAL REHEARSAL" }, h2 { "Same invariants, separate chain context" } }, StatusBadge { label: data.portability.status.clone(), tone: "verified".to_owned() } }
                p { "The same checked-in Solidity bytecode is exercised through local revm with the Polkadot Hub TestNet chain context. This prepares portability without presenting an undeployed network as observed." }
                dl { class: "readout-list", ReadoutRow { label: "Target".to_owned(), value: format!("{} / chain {}", data.portability.target.network, data.portability.target.chain_id), detail: data.portability.target.execution_backend.clone() }, ReadoutRow { label: "Canonical transitions".to_owned(), value: data.portability.comparison.canonical_transitions.clone(), detail: "Ethereum-local vs target-context projection".to_owned() }, ReadoutRow { label: "State roots".to_owned(), value: data.portability.comparison.state_roots.clone(), detail: "same fixture commitments".to_owned() }, ReadoutRow { label: "Rollback reason".to_owned(), value: data.portability.comparison.stale_predecessor_rejection.clone(), detail: "exact Solidity machine reason".to_owned() }, ReadoutRow { label: "EIP-712 domain".to_owned(), value: data.portability.comparison.eip712_domain.clone(), detail: "signatures remain chain-bound".to_owned() }, ReadoutRow { label: "Deployment / RPC".to_owned(), value: format!("{} / {}", data.portability.target.deployment, data.portability.target.public_rpc_observation), detail: "no public Polkadot claim".to_owned() } }
                p { class: "small-note", "This is a local portability rehearsal, not a public deployment, RPC observation, consensus proof, or cross-chain authorization." }
            }
            section { class: "section-wrap evidence-grid",
                div { class: "content-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "REFERENCE AGENT RUNTIME" }, h2 { "A real loader boundary, exercised locally" } }, StatusBadge { label: if data.reference_runtime.all_expected { "VERIFIED".to_owned() } else { "FAILED".to_owned() }, tone: if data.reference_runtime.all_expected { "verified".to_owned() } else { "danger".to_owned() } } }
                    p { "The reference runtime loads the current head into an actual in-memory session, holds a historical checkpoint before the loader, holds a diverged snapshot for review, and fails closed on invalid evidence." }
                    dl { class: "readout-list", ReadoutRow { label: "Current head".to_owned(), value: format!("{} / loader {}", data.reference_runtime.cases.current_head.status, if data.reference_runtime.cases.current_head.loader_invoked { "invoked" } else { "not invoked" }), detail: "snapshot 3 / RESUME_ALLOWED".to_owned() }, ReadoutRow { label: "Historical checkpoint".to_owned(), value: format!("{} / {}", data.reference_runtime.cases.historical_checkpoint.status, data.reference_runtime.cases.historical_checkpoint.recommended_action.as_deref().unwrap_or("HOLD")), detail: "snapshot 1 is not loaded".to_owned() }, ReadoutRow { label: "Diverged snapshot".to_owned(), value: format!("{} / {}", data.reference_runtime.cases.diverged_snapshot.status, data.reference_runtime.cases.diverged_snapshot.recommended_action.as_deref().unwrap_or("HOLD")), detail: "unknown commitment is held".to_owned() }, ReadoutRow { label: "Invalid evidence".to_owned(), value: data.reference_runtime.cases.invalid_evidence.status.clone(), detail: "loader not invoked".to_owned() }, ReadoutRow { label: "Raw memory export".to_owned(), value: data.reference_runtime.raw_memory_exported.to_string(), detail: "runtime report contains outcomes, not private values".to_owned() } }
                    p { class: "small-note", "This proves a framework-neutral local reference integration. It is not evidence of external developer adoption or a production agent framework integration." }
                }
                div { class: "content-panel",
                    div { class: "panel-title-row", div { p { class: "panel-kicker", "BOUNDED SECURITY ASSURANCE" }, h2 { "Executable negative-path coverage" } }, StatusBadge { label: data.security_assurance.status.clone(), tone: "verified".to_owned() } }
                    p { "The published Solidity creation artifact was exercised through an independent Rust/revm lane across valid histories, stale predecessors, sequence gaps, signature boundaries, ERC-1271, and authority rotation." }
                    dl { class: "readout-list", ReadoutRow { label: "Valid history".to_owned(), value: format!("{} transitions", data.security_assurance.valid_transitions), detail: "Rust/Solidity roots match".to_owned() }, ReadoutRow { label: "Stale predecessors".to_owned(), value: format!("{}/{} expected", data.security_assurance.stale_predecessor_cases.iter().filter(|case| case.status == "REJECTED" && case.observed == case.expected).count(), data.security_assurance.stale_predecessor_cases.len()), detail: "BAD_PREVIOUS_STATE".to_owned() }, ReadoutRow { label: "Mutation matrix".to_owned(), value: format!("{}/{} rejected", data.security_assurance.mutation_matrix.rejected, data.security_assurance.mutation_matrix.total), detail: "expected contract reasons".to_owned() }, ReadoutRow { label: "Formal status".to_owned(), value: data.security_assurance.formal_status.clone(), detail: "bounded assurance is not formal verification".to_owned() } }
                    p { class: "small-note", "The report is deliberately bounded. It is not a third-party security audit, certification, or proof over all possible inputs." }
                }
            }
            section { class: "section-wrap evidence-lower-grid",
                div { class: "content-panel", div { class: "panel-kicker", "UNIFIED DEMO SPACE V2" }, h3 { "One incident, one portable bundle." }, p { "The local Demo Space V2 ties commitments derived from synthetic SQLite snapshots to three Solidity transitions, one authority rotation, and the stale-predecessor rejection. Sample values are excluded from the bundle." }, dl { class: "readout-list", ReadoutRow { label: "Canonical head".to_owned(), value: format!("sequence {}", data.v2.head.sequence), detail: "Rust/revm execution".to_owned() }, ReadoutRow { label: "Rollback result".to_owned(), value: data.v2.attack.as_ref().and_then(|attack| attack.reason.clone()).unwrap_or_else(|| "NOT RECORDED".to_owned()), detail: "Transition 1 root used as stale predecessor".to_owned() }, ReadoutRow { label: "Authority".to_owned(), value: format!("{} records", data.v2.authorization_history.len()), detail: "configNonce 0 → 1".to_owned() } } }
                div { class: "content-panel", div { class: "panel-kicker", "PUBLISHED ADVERSARIAL CORPUS" }, h3 { "{data.mutation_rejected}/{data.mutation_count} expected rejection cases observed" }, div { class: "reason-grid reason-grid-wide", for mutation in data.bundle.mutation_matrix.iter().filter(|mutation| mutation.expected == "REJECT").take(8) { div { class: "reason-cell", key: "{mutation.name}", code { "{mutation.observed.reason.as_deref().unwrap_or(\"NO_REASON\")}" }, span { "{mutation.name}" } } } }, Link { class: "text-link", to: AppRoute::Lab {}, "Open Tampering Lab →" } }
                div { class: "content-panel content-panel-muted", div { class: "panel-kicker", "EXTERNAL REPRODUCTION" }, h3 { "NOT YET DEMONSTRATED" }, p { "No independent human clean-checkout result is claimed in this workspace yet. The repository's own gates remain separate from that future evidence." }, Link { class: "text-link", to: AppRoute::Reproduce {}, "See the reproducible commands →" } }
            }
            section { class: "section-wrap artifact-list content-panel",
                div { class: "panel-kicker", "AVAILABLE ARTIFACTS" }
                div { class: "artifact-row", span { "evidence/local/rust_revm_conformance.json" }, StatusBadge { label: "PUBLISHED".to_owned(), tone: "observed".to_owned() } }
                div { class: "artifact-row", span { "evidence/local/demo_space_v2_evidence.json" }, StatusBadge { label: "DEMO V2".to_owned(), tone: "verified".to_owned() } }
                div { class: "artifact-row", span { "evidence/local/demo_space_v2_recovery_receipt.json" }, StatusBadge { label: "RECEIPT".to_owned(), tone: "verified".to_owned() } }
                div { class: "artifact-row", span { "evidence/local/demo_space_v2_historical_recovery_receipt.json" }, StatusBadge { label: "HOLD RECEIPT".to_owned(), tone: "warning".to_owned() } }
                div { class: "artifact-row", span { "fixtures/silent-rollback-v2/manifest.json" }, StatusBadge { label: "FIXTURE".to_owned(), tone: "observed".to_owned() } }
                div { class: "artifact-row", span { "evidence/local/rust_revm_mutation_matrix.json" }, StatusBadge { label: "PUBLISHED".to_owned(), tone: "observed".to_owned() } }
                div { class: "artifact-row", span { "evidence/local/reference_agent_runtime.json" }, StatusBadge { label: "RUNTIME".to_owned(), tone: "verified".to_owned() } }
                div { class: "artifact-row", span { "evidence/local/security_assurance_report.json" }, StatusBadge { label: "BOUNDED ASSURANCE".to_owned(), tone: "warning".to_owned() } }
                div { class: "artifact-row", span { "evidence/local/polkadot_hub_portability_rehearsal.json" }, StatusBadge { label: "LOCAL REHEARSAL".to_owned(), tone: "observed".to_owned() } }
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
        div { class: "page workspace-page architecture-page",
            PageHeader { kicker: "SYSTEM OVERVIEW / PROTOCOL ARCHITECTURE".to_owned(), title: "How It Works".to_owned(), description: "Private memory stays off-chain; its succession is checked against a shared registry.".to_owned(), source: "CURRENT RUST STACK".to_owned() }
            div { class: "architecture-layout",
                main {
                    section { class: "content-panel architecture-flow",
                        div { class: "panel-title-row", div { p { class: "panel-kicker", "MEMORYLINEAGE ARCHITECTURE" } h2 { "From private agent memory to public, independently verifiable lineage." } } }
                        div { class: "architecture-domain-row", span { "PRIVATE DOMAIN" }, span { "Sensitive data stays off-chain" }, span { "PUBLIC VERIFICATION DOMAIN" }, span { "Only commitments and metadata on-chain" } }
                        div { class: "architecture-pipeline",
                            div { class: "architecture-node architecture-private", strong { "PRIVATE AGENT MEMORY" }, span { "in-memory state, thoughts, and context" }, small { "PRIVATE" } }
                            div { class: "architecture-arrow", "→" }
                            div { class: "architecture-node", strong { "SQLITE SNAPSHOT STORE" }, span { "versioned local memory snapshots" }, small { "OFF-CHAIN" } }
                            div { class: "architecture-arrow", "→" }
                            div { class: "architecture-node", strong { "RUST CANONICALIZATION" }, span { "deterministic state normalization" }, small { "DETERMINISTIC" } }
                            div { class: "architecture-arrow", "→" }
                            div { class: "architecture-node", strong { "COMMITMENT ENGINE" }, span { "state roots and cryptographic commitments" }, small { "IMMUTABLE" } }
                            div { class: "architecture-arrow", "→" }
                            div { class: "architecture-node architecture-chain", strong { "SOLIDITY REGISTRY" }, span { "Ethereum Sepolia / EIP-712 / ERC-1271" }, small { "ON-CHAIN" } }
                        }
                        div { class: "architecture-branches", div { class: "architecture-branch", strong { "Rust/WASM Inspector" }, span { "Explore and verify lineage with read-only RPC." } }, div { class: "architecture-branch", strong { "Portable Evidence" }, span { "Machine-readable proof package for replay." } }, div { class: "architecture-branch", strong { "Independent Rust Verifier" }, span { "Anyone can verify without raw memory." } } }
                    }
                    section { class: "section-wrap architecture-card-grid",
                        div { class: "content-panel", div { class: "panel-kicker", "WHY BLOCKCHAIN?" }, h2 { "The operator should not be the only historian." }, p { "The registry enforces a public append-only boundary for the commitments that define succession. An external verifier can compare a candidate against one committed head." }, Link { class: "text-link", to: AppRoute::Security {}, "Read the trust boundary ", Icon { name: IconName::Arrow, size: 14 } } }
                        div { class: "content-panel", div { class: "panel-kicker", "WHY RAW MEMORY STAYS OFF-CHAIN?" }, h2 { "Commit the history, keep the content private." }, p { "Raw agent memory can be sensitive, personal, or proprietary. MemoryLineage publishes fixed-size commitments and metadata, not the underlying snapshot." }, ul { class: "scope-list scope-list-positive", li { "Privacy for raw memory" }, li { "Size and cost remain bounded" }, li { "Independent lineage checks remain possible" } } }
                        div { class: "content-panel", div { class: "panel-kicker", "WHY RUST + SOLIDITY?" }, h2 { "Use each tool at the right trust boundary." }, p { "Rust supplies deterministic replay, native and WASM tooling; Solidity supplies the Ethereum-native enforcement anchor." }, dl { class: "readout-list", ReadoutRow { label: "Rust".to_owned(), value: "1.97.1".to_owned(), detail: "pinned workspace".to_owned() }, ReadoutRow { label: "Solidity".to_owned(), value: "registry".to_owned(), detail: "Ethereum trust anchor".to_owned() }, ReadoutRow { label: "Head".to_owned(), value: data.v2.head.sequence.to_string(), detail: "local evidence".to_owned() } } }
                    }
                }
                aside { class: "architecture-sidebar",
                    section { class: "content-panel architecture-trust-panel", div { class: "panel-kicker", "TRUST BOUNDARY" }, p { class: "panel-subtitle", "Private operation and public verification remain separate." }, div { class: "domain-cards", div { class: "domain-card domain-card-private", strong { "PRIVATE DOMAIN" }, span { "Agent operator controlled" }, ul { li { "raw memory and context" }, li { "local snapshot storage" }, li { "canonicalization process" } }, small { "OFF-CHAIN / PRIVATE" } }, div { class: "domain-card domain-card-public", strong { "PUBLIC VERIFICATION DOMAIN" }, span { "Open and independently verifiable" }, ul { li { "state commitments" }, li { "authorization metadata" }, li { "registry observations" } }, small { "ON-CHAIN / PUBLIC" } } } }
                    section { class: "content-panel architecture-data-panel", div { class: "panel-kicker", "DATA TYPES" }, p { class: "panel-subtitle", "Core structures in the protocol." }, div { class: "architecture-data-list", div { strong { "Sequence" }, span { "Monotonic order for each committed state." } }, div { strong { "Transition ID" }, span { "Unique identifier bound to the delta." } }, div { strong { "State Root" }, span { "Cryptographic root of the canonicalized state." } }, div { strong { "Authorization" }, span { "Who authorized the transition and under which config." } } } }
                    section { class: "content-panel architecture-invariants-panel", div { class: "panel-kicker", "CORE INVARIANTS" }, p { class: "panel-subtitle", "Properties checked by the evidence paths." }, for invariant in ["Sequence continuity", "Predecessor continuity", "Configured authorization", "Canonical head consistency"] { div { class: "invariant-row", span { class: "invariant-check", "✓" }, strong { "{invariant}" } } } }
                }
            }
        }
    }
}

#[component]
pub fn SecurityPage(data: UiData) -> Element {
    let verified = [
        "committed ordering",
        "sequence continuity",
        "predecessor continuity",
        "configured-authorizer checks in tested contract executions",
        "authority rotation behavior in the Rust/revm evidence",
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
        "historical transition signatures for bundles that do not carry signed proof material",
    ];
    rsx! {
        div { class: "page workspace-page security-page",
            PageHeader { kicker: "SECURITY BOUNDARY / THREAT MODEL AND GUARANTEES".to_owned(), title: "Security & Scope".to_owned(), description: "The guarantees, assumptions, and questions this evidence cannot answer.".to_owned(), source: "EXPLICIT SCOPE".to_owned() }
            div { class: "security-layout",
                main {
                    section { class: "security-scope-grid",
                        div { class: "content-panel scope-panel scope-panel-positive", div { class: "panel-kicker", "WHAT MEMORYLINEAGE VERIFIES" }, h2 { "Committed history integrity" }, for item in verified { div { class: "scope-row", key: "{item}", span { class: "scope-symbol", "✓" }, span { "{item}" } } } }
                        div { class: "content-panel scope-panel scope-panel-muted", div { class: "panel-kicker", "WHAT MEMORYLINEAGE DOES NOT VERIFY" }, h2 { "Semantic or behavioral truth" }, for item in not_verified { div { class: "scope-row", key: "{item}", span { class: "scope-symbol scope-symbol-muted", "×" }, span { "{item}" } } } }
                    }
                    section { class: "section-wrap content-panel threat-model-panel", div { class: "panel-kicker", "THREAT MODEL" }, p { class: "panel-subtitle", "Adversarial scenarios we test and the expected system boundary." }, div { class: "threat-table", div { class: "threat-table-head", span { "THREAT" }, span { "INVARIANT TESTED" }, span { "EXPECTED RESULT" } }, div { class: "threat-row", span { "Stale restored snapshot" }, span { "Predecessor continuity" }, StatusBadge { label: "REJECT".to_owned(), tone: "danger".to_owned() } }, div { class: "threat-row", span { "Skipped sequence" }, span { "Sequence continuity" }, StatusBadge { label: "REJECT".to_owned(), tone: "danger".to_owned() } }, div { class: "threat-row", span { "Parallel successor" }, span { "Canonical uniqueness" }, StatusBadge { label: "REJECT".to_owned(), tone: "danger".to_owned() } }, div { class: "threat-row", span { "Wrong authorizer" }, span { "Authorization" }, StatusBadge { label: "REJECT".to_owned(), tone: "danger".to_owned() } }, div { class: "threat-row", span { "Modified committed field" }, span { "Commitment integrity" }, StatusBadge { label: "REJECT".to_owned(), tone: "danger".to_owned() } }, div { class: "threat-row", span { "Malicious semantic content" }, span { "Semantic scope" }, StatusBadge { label: "OUT OF SCOPE".to_owned(), tone: "warning".to_owned() } } } }
                    section { class: "section-wrap content-panel security-assurance-panel", div { class: "panel-title-row", div { p { class: "panel-kicker", "BOUNDED ASSURANCE / ACTUAL ARTIFACT" } h2 { "Executable negative-path coverage" } }, StatusBadge { label: data.security_assurance.status.clone(), tone: if data.security_assurance.status == "BOUNDED_ASSURANCE_PASS" { "verified".to_owned() } else { "danger".to_owned() } } }, p { "The published Solidity creation artifact was exercised through an independent Rust/revm lane across valid histories, stale predecessors, sequence gaps, signature boundaries, ERC-1271, and authority rotation." }, div { class: "security-assurance-grid", ReadoutRow { label: "Execution".to_owned(), value: "Rust/revm".to_owned(), detail: "published Solidity bytecode".to_owned() } ReadoutRow { label: "Formal verification".to_owned(), value: data.security_assurance.formal_status.clone(), detail: "not claimed".to_owned() } ReadoutRow { label: "Third-party audit".to_owned(), value: "NOT CLAIMED".to_owned(), detail: "no external auditor report".to_owned() } ReadoutRow { label: "Production adoption".to_owned(), value: "NOT YET DEMONSTRATED".to_owned(), detail: "reference runtime is local".to_owned() } } }
                    section { class: "section-wrap content-panel", div { class: "panel-kicker", "STATUS VOCABULARY" }, div { class: "status-vocabulary", StatusBadge { label: "VERIFIED".to_owned(), tone: "verified".to_owned() }, span { "relevant deterministic checks passed" }, StatusBadge { label: "OBSERVED".to_owned(), tone: "observed".to_owned() }, span { "read from a public source" }, StatusBadge { label: "REJECTED".to_owned(), tone: "danger".to_owned() }, span { "actual verification/simulation failed as expected" }, StatusBadge { label: "OUT OF SCOPE".to_owned(), tone: "warning".to_owned() }, span { "not evaluated by this product" }, StatusBadge { label: "NOT YET DEMONSTRATED".to_owned(), tone: "neutral".to_owned() }, span { "no evidence is being implied" } } }
                }
                aside { class: "security-sidebar",
                    section { class: "security-trust-list", p { class: "panel-kicker", "PUBLIC CHECKPOINT" }, div { class: "trust-item", Icon { name: IconName::Lock, size: 24 }, div { strong { "Private memory stays out" }, span { "Raw memory never enters the registry." } } } div { class: "trust-item", Icon { name: IconName::History, size: 24 }, div { strong { "Succession is recorded" }, span { "Each accepted state names its predecessor." } } } div { class: "trust-item", Icon { name: IconName::Authority, size: 24 }, div { strong { "Authority changes are visible" }, span { "Rotation is part of the recorded configuration." } } } div { class: "trust-item", Icon { name: IconName::Open, size: 24 }, div { strong { "Evidence is replayable" }, span { "The bundle can be checked independently." } } } }
                    section { class: "content-panel security-assumptions", div { class: "panel-kicker", "TRUST ASSUMPTIONS" }, p { class: "panel-subtitle", "MemoryLineage relies on the following assumptions." }, for (index, assumption) in ["Ethereum semantics and finality", "Standard cryptographic primitives", "Correctly deployed registry", "Evidence remains available", "Accurate source labeling"].iter().enumerate() { div { class: "assumption-row", span { "{index + 1}" }, strong { "{assumption}" } } } }
                    section { class: "content-panel", div { class: "panel-kicker", "ERC-1271 BOUNDARY" }, h3 { "On-chain acceptance is not offline re-execution." }, p { "The published lane demonstrates registry acceptance and rejection behavior for contract-based authorizers. It does not claim full historical signer-contract replay." } }
                    section { class: "content-panel", div { class: "panel-kicker", "SECURITY REPORTING" }, p { "Please report suspected security issues responsibly." }, Link { class: "button button-secondary button-full", to: AppRoute::PriorWork {}, "Review provenance ", Icon { name: IconName::Arrow, size: 14 } } }
                }
            }
        }
    }
}

#[component]
pub fn ReproducePage(data: UiData) -> Element {
    rsx! {
        div { class: "page workspace-page reproduce-page",
            div { class: "reproduce-intro-grid",
                PageHeader { kicker: "CLEAN-CHECKOUT REPRODUCTION / DEVELOPER GUIDE".to_owned(), title: "Reproduce the Submission".to_owned(), description: "Run the same checks from a clean checkout.".to_owned(), source: "REPOSITORY COMMANDS".to_owned() }
                aside { class: "reproduce-intro-aside", p { class: "panel-kicker", "WHAT CAN BE REPRODUCED" }, div { class: "trust-item", Icon { name: IconName::Database, size: 22 }, div { strong { "Deterministic builds" }, span { "Same results from a clean checkout" } } } div { class: "trust-item", Icon { name: IconName::Shield, size: 22 }, div { strong { "End-to-end verification" }, span { "Replay, attack, and verify independently" } } } div { class: "trust-item", Icon { name: IconName::Open, size: 22 }, div { strong { "Open source" }, span { "Commands, scripts, and CI are publicly available" } } } }
            }
            div { class: "reproduce-top-grid",
                section { class: "content-panel reproduce-environment",
                    div { class: "panel-kicker", "ENVIRONMENT / SUBMISSION SNAPSHOT" }
                    div { class: "fingerprint-grid",
                        ReadoutRow { label: "Rust version".to_owned(), value: "1.97.1".to_owned(), detail: "rust-toolchain.toml".to_owned() }
                        ReadoutRow { label: "Dioxus".to_owned(), value: "0.8.0-alpha.1".to_owned(), detail: "WASM frontend".to_owned() }
                        ReadoutRow { label: "Registry".to_owned(), value: short_hash(&data.deployment.registry_address, 14, 8), detail: "published Sepolia anchor".to_owned() }
                        ReadoutRow { label: "Evidence".to_owned(), value: data.v2.evidence_type.clone(), detail: "Demo Space V2".to_owned() }
                    }
                }
                section { class: "content-panel reproduce-command-panel",
                    div { class: "panel-kicker", "PRIMARY COMMAND" }
                    code { class: "command-block", "$ cargo xtask reproduce\n$ cargo xtask verify" }
                    p { "Environment, Rust verification, static website build, browser smoke, and package boundary." }
                    Link { class: "button button-primary", to: AppRoute::Verify {}, "Verify evidence in the browser ", Icon { name: IconName::Arrow, size: 14 } }
                }
            }
            section { class: "section-wrap reproduce-gate-panel content-panel",
                div { class: "panel-title-row reproduce-gate-heading",
                    div { p { class: "panel-kicker", "JUDGE IN 90 SECONDS" }, h2 { "One incident, four checks" } }
                    StatusBadge { label: "REPLAYABLE".to_owned(), tone: "observed".to_owned() }
                }
                div { class: "panel-kicker", "VERIFICATION GATE OUTPUT" }
                code { class: "command-block command-block-gate", "$ cargo xtask verify\n[PASS] Rust formatting\n[PASS] Workspace tests\n[PASS] Protocol conformance\n[PASS] Rust/revm Silent Rollback\n[PASS] Mutation corpus\n[PASS] Independent verifier\n[PASS] Dioxus static build\n[PASS] Package boundary\n\nVERIFICATION GATE PASS" }
            }
            section { class: "section-wrap reproduce-triad",
                div { class: "content-panel reproduce-check-card",
                    div { class: "panel-title-row", div { class: "panel-kicker", "RUN THE WEBSITE" }, StatusBadge { label: "PASS".to_owned(), tone: "verified".to_owned() } }
                    h3 { "Open the Inspector" }
                    p { "Build the static Rust/WASM site and run the browser acceptance smoke." }
                    code { class: "command-block", "cargo xtask build-web" }
                    Link { class: "text-link", to: AppRoute::Inspect {}, "Open Inspect ", Icon { name: IconName::Arrow, size: 14 } }
                }
                div { class: "content-panel reproduce-check-card",
                    div { class: "panel-title-row", div { class: "panel-kicker", "REHEARSE SILENT ROLLBACK" }, StatusBadge { label: "PASS".to_owned(), tone: "verified".to_owned() } }
                    h3 { "Replay the stale-root case" }
                    p { "Restore snapshot 1 against the state-3 canonical head." }
                    code { class: "command-block", "cargo run -q -p ml-cli -- demo silent-rollback" }
                    Link { class: "text-link", to: AppRoute::LabScenario { scenario: "silent-rollback".to_owned() }, "Run the lab ", Icon { name: IconName::Arrow, size: 14 } }
                }
                div { class: "content-panel reproduce-check-card",
                    div { class: "panel-title-row", div { class: "panel-kicker", "VERIFY A BUNDLE" }, StatusBadge { label: "PASS".to_owned(), tone: "verified".to_owned() } }
                    h3 { "Replay portable evidence" }
                    p { "Tamper one field, observe failure, restore the original, then run the CLI verifier." }
                    code { class: "command-block", "cargo run -q -p ml-cli -- verify evidence.json" }
                    Link { class: "text-link", to: AppRoute::Verify {}, "Verify the bundle ", Icon { name: IconName::Arrow, size: 14 } }
                }
            }
            section { class: "section-wrap reproduce-bottom-grid",
                div { class: "content-panel",
                    div { class: "panel-kicker", "EXTERNAL REPRODUCTION / HUMAN REPORT" }
                    h2 { "NOT YET DEMONSTRATED" }
                    p { "Give a reviewer the repository URL and runbook without a personal walkthrough. Store their exact commit, environment, commands, output, and comprehension answers before claiming human reproduction." }
                    div { class: "reproduce-report-fields", ReadoutRow { label: "Report template".to_owned(), value: "external-developer-report.md".to_owned(), detail: "repository reproduction packet".to_owned() }, ReadoutRow { label: "Required answers".to_owned(), value: "proves / does not prove".to_owned(), detail: "checks product comprehension".to_owned() } }
                }
                div { class: "content-panel",
                    div { class: "panel-kicker", "WHAT THIS PROVES" }
                    div { class: "reproduce-proof-grid", div { strong { "Same source" }, span { "A clean checkout can execute the published gates." } }, div { strong { "Same evidence" }, span { "Browser and CLI consume portable JSON." } }, div { strong { "Same boundaries" }, span { "Unproven claims remain visibly unproven." } } }
                    Link { class: "text-link", to: AppRoute::PriorWork {}, "Review provenance ", Icon { name: IconName::Arrow, size: 14 } }
                }
            }
            section { class: "section-wrap content-panel",
                div { class: "panel-kicker", "LOCAL COMMANDS" }
                CommandRow { command: "cargo xtask verify".to_owned(), note: "complete deterministic verification gate".to_owned() }
                CommandRow { command: "cargo run -q -p ml-cli -- verify evidence/local/demo_space_v2_evidence.json".to_owned(), note: "independent Demo Space V2 evidence replay".to_owned() }
                CommandRow { command: "cargo xtask reviewer-reproduce".to_owned(), note: "clean archive reproduction path".to_owned() }
                p { class: "small-note", "Current Demo Space bundle: {data.v2.evidence_type}. Automated reproduction is recorded locally; an independent human clean-checkout result remains NOT YET DEMONSTRATED." }
            }
        }
    }
}

#[component]
pub fn PriorWorkPage(data: UiData) -> Element {
    rsx! {
        div { class: "page workspace-page",
            PageHeader { kicker: "PROVENANCE / SUBMISSION RECORD".to_owned(), title: "What existed before the hackathon?".to_owned(), description: "A clear boundary between prior standards and the evidence-backed work in this repository.".to_owned(), source: "PROVENANCE LEDGER".to_owned() }
            section { class: "section-wrap provenance-grid",
                div { class: "content-panel provenance-column", div { class: "panel-kicker", "BEFORE 3RD-WEB-HACK" }, h2 { "Standards and primitives" }, ProvenanceRow { label: "ERC-8350 draft semantics".to_owned(), detail: "pinned as an external standard snapshot".to_owned(), tone: "observed".to_owned() }, ProvenanceRow { label: "EIP-712".to_owned(), detail: "Ethereum typed-data authorization primitive".to_owned(), tone: "observed".to_owned() }, ProvenanceRow { label: "ERC-1271".to_owned(), detail: "contract-based signature validation boundary".to_owned(), tone: "observed".to_owned() }, ProvenanceRow { label: "Ethereum Sepolia".to_owned(), detail: "public testnet used as trust anchor".to_owned(), tone: "observed".to_owned() } }
                div { class: "provenance-divider", "→" }
                    div { class: "content-panel provenance-column provenance-column-built", div { class: "panel-kicker", "BUILT / EVIDENCED FOR THIS SUBMISSION" }, h2 { "Independent audit product" }, ProvenanceRow { label: "MemoryLineage Inspector".to_owned(), detail: "Rust/WASM evidence workspace".to_owned(), tone: "verified".to_owned() }, ProvenanceRow { label: "Rust reference + verifier".to_owned(), detail: "separate deterministic replay paths".to_owned(), tone: "verified".to_owned() }, ProvenanceRow { label: "Silent Rollback fixture".to_owned(), detail: format!("synthetic SQLite snapshots {} / {} / {}", data.fixture.snapshots[0].sequence, data.fixture.snapshots[1].sequence, data.fixture.snapshots[2].sequence), tone: "verified".to_owned() }, ProvenanceRow { label: "Tampering corpus".to_owned(), detail: format!("{}/{} expected rejection cases", data.mutation_rejected, data.mutation_count), tone: "verified".to_owned() }, ProvenanceRow { label: "Sepolia evidence".to_owned(), detail: "deployment + second RPC reread".to_owned(), tone: "verified".to_owned() } }
            }
            section { class: "section-wrap content-panel provenance-ledger",
                div { class: "panel-title-row", div { p { class: "panel-kicker", "SUBMISSION IDENTITY" }, h2 { "Only finalized values are listed." } }, StatusBadge { label: "TRANSPARENT".to_owned(), tone: "warning".to_owned() } }
                dl { class: "readout-list", ReadoutRow { label: "Contract address".to_owned(), value: short_hash(&data.deployment.registry_address, 14, 8), detail: "workspace-owned Sepolia deployment".to_owned() }, ReadoutRow { label: "Spec pin".to_owned(), value: "ERC-8350 / v1-pinned-vector-2026-09-18".to_owned(), detail: "published vector snapshot".to_owned() }, ReadoutRow { label: "GitHub URL".to_owned(), value: "AndroLay/MemoryLineage".to_owned(), detail: "private repository / configured".to_owned() }, ReadoutRow { label: "Submission tag".to_owned(), value: "NOT YET CREATED".to_owned(), detail: "release identity remains pending".to_owned() } }
                p { class: "small-note", "MemoryLineage does not claim to have invented ERC-8350 or to be the first AI memory lineage system. Its distinction is the independent, authorization-bound audit surface and reproducible evidence around the pinned semantics." }
            }
            section { class: "section-wrap content-panel", div { class: "panel-kicker", "RELATED SURFACES" }, div { class: "related-link-grid", Link { to: AppRoute::Architecture {}, "Architecture →" }, Link { to: AppRoute::Evidence {}, "Public evidence →" }, Link { to: AppRoute::Security {}, "Security boundary →" }, Link { to: AppRoute::Reproduce {}, "Reproduce →" }, Link { to: AppRoute::PriorWork {}, "Prior work →" } } }
        }
    }
}
