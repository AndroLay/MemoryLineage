#![forbid(unsafe_code)]

use crate::AppRoute;
use crate::browser::copy_text;
use crate::data::{HistoryLedgerEvent, Route, UiData, short_hash};
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconName {
    Arrow,
    Authority,
    Database,
    History,
    Lock,
    Open,
    Shield,
}

#[component]
pub fn Icon(name: IconName, size: u32) -> Element {
    let path = match name {
        IconName::Arrow => "M5 12h14M13 6l6 6-6 6",
        IconName::Authority => {
            "M6 20v-2a4 4 0 0 1 8 0v2M10 10a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM16 11a2.5 2.5 0 1 0 0-5M18 20v-1a3 3 0 0 0-2-2.8"
        }
        IconName::Database => {
            "M4 6c0-2 16-2 16 0v12c0 2-16 2-16 0V6zM4 6c0 2 16 2 16 0M4 12c0 2 16 2 16 0"
        }
        IconName::History => "M3 12a9 9 0 1 0 3-6.7M3 4v5h5M12 7v5l3 2",
        IconName::Lock => "M6 10V8a6 6 0 0 1 12 0v2M5 10h14v10H5V10z",
        IconName::Open => {
            "M4 5h10a2 2 0 0 1 2 2v12H6a2 2 0 0 1-2-2V5zM16 8h4v11a2 2 0 0 1-2 2h-2M8 9h5M8 13h5"
        }
        IconName::Shield => "M12 3l7 3v5c0 4.5-2.9 8.2-7 10-4.1-1.8-7-5.5-7-10V6l7-3z",
    };
    rsx! {
        svg {
            class: "icon",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.8",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "{path}" }
        }
    }
}

#[component]
pub fn TopNavigation(route: Route, show_primary: bool, landing: bool) -> Element {
    let mut guided_tour = use_context::<Signal<Option<crate::TourStep>>>();
    let mut tour_progress = use_context::<Signal<crate::TourProgress>>();
    let inspect_surface = matches!(route, Route::Inspect | Route::Transition(_));
    let active = |candidate: Route| {
        if route == candidate || (candidate == Route::Inspect && inspect_surface) {
            "nav-link nav-link-active"
        } else {
            "nav-link"
        }
    };
    let lab_class = if matches!(route, Route::Lab(_)) {
        "nav-link nav-link-active"
    } else {
        "nav-link"
    };
    rsx! {
        header { class: if landing { "topbar topbar-landing" } else { "topbar" },
            Link { class: "brand", to: AppRoute::Welcome {}, aria_label: "MemoryLineage home", onclick: move |_| guided_tour.set(None),
                span { class: "brand-mark",
                    img { class: "brand-logo", src: asset!("/assets/memorylineage-mark-reversed.svg"), alt: "MemoryLineage logo" }
                }
                span { strong { "MEMORYLINEAGE" } }
            }
            if landing {
                nav { class: "landing-nav", aria_label: "Landing page sections",
                    div { class: "landing-nav-wide",
                        a { href: "#problem", "The problem" }
                        a { href: "#how-it-works", "How it works" }
                        a { href: "#who-its-for", "Who it's for" }
                        a { href: "#evidence-boundary", "Evidence & limits" }
                        a { href: "#inside-inspector", "Inside the Inspector" }
                        a { href: "#questions", "Questions" }
                    }
                    details { class: "landing-nav-compact",
                        summary { "Sections" }
                        div { class: "landing-nav-menu",
                            a { href: "#problem", "The problem" }
                            a { href: "#how-it-works", "How it works" }
                            a { href: "#who-its-for", "Who it's for" }
                            a { href: "#evidence-boundary", "Evidence & limits" }
                            a { href: "#inside-inspector", "Inside the Inspector" }
                            a { href: "#questions", "Questions" }
                        }
                    }
                }
            } else if show_primary {
                nav { class: "primary-nav", aria_label: "Primary navigation",
                    Link { class: active(Route::Inspect), to: AppRoute::Inspect {}, "Inspect" }
                    Link { class: active(Route::History), to: AppRoute::History {}, "History" }
                    Link { class: lab_class, to: AppRoute::Lab {}, "Tampering Lab" }
                    Link { class: active(Route::Verify), to: AppRoute::Verify {}, "Verify" }
                }
            }
            if landing {
                Link {
                    class: "landing-nav-cta",
                    to: AppRoute::Home {},
                    aria_label: "Get started with the guided MemoryLineage walkthrough",
                    onclick: move |_| {
                        guided_tour.set(Some(crate::TourStep::Incident));
                        tour_progress.set(crate::TourProgress::default());
                    },
                    span { class: "landing-nav-cta-wide", "Get started" }
                    span { class: "landing-nav-cta-compact", "Start" }
                    Icon { name: IconName::Arrow, size: 15 }
                }
            }
            div { class: "topbar-meta",
                a { class: "github-button", href: "https://github.com/AndroLay/MemoryLineage", target: "_blank", rel: "noopener noreferrer", aria_label: "Open MemoryLineage public source on GitHub in a new tab", title: "Open public source on GitHub in a new tab",
                    svg { class: "github-mark", view_box: "0 0 24 24",
                        path { fill: "currentColor", d: "M12 .9a11.1 11.1 0 0 0-3.51 21.63c.55.1.76-.24.76-.54v-2.1c-3.1.67-3.76-1.32-3.76-1.32-.5-1.3-1.24-1.65-1.24-1.65-1.02-.7.08-.69.08-.69 1.12.08 1.71 1.15 1.71 1.15 1 .1.8 2.16 3.02 1.62.1-.72.4-1.2.7-1.48-2.47-.28-5.07-1.24-5.07-5.5 0-1.22.44-2.22 1.15-3-.12-.28-.5-1.42.11-2.96 0 0 .94-.3 3.05 1.15a10.6 10.6 0 0 1 5.55 0c2.11-1.45 3.05-1.15 3.05-1.15.61 1.54.23 2.68.11 2.96.72.78 1.15 1.78 1.15 3 0 4.27-2.6 5.21-5.08 5.49.4.35.75 1.02.75 2.06v3.04c0 .3.2.65.77.54A11.1 11.1 0 0 0 12 .9Z" }
                    }
                    span { "GitHub" }
                    span { class: "github-button-hint", "Source" }
                }
            }
        }
    }
}

#[component]
pub fn PageHeader(kicker: String, title: String, description: String, source: String) -> Element {
    rsx! {
        div { class: "page-header",
            div {
                p { class: "eyebrow", "{kicker}" }
                h1 { "{title}" }
                p { class: "page-lede", "{description}" }
            }
            span { class: "source-chip source-published", "{source}" }
        }
    }
}

#[component]
pub fn StatusBadge(label: String, tone: String) -> Element {
    rsx! { span { class: "status-badge status-{tone}", span { class: "status-dot" }, "{label}" } }
}

#[component]
pub fn CopyValue(label: String, value: String, compact: bool) -> Element {
    let copy = value.clone();
    let class = if compact {
        "hash-value hash-value-compact"
    } else {
        "hash-value"
    };
    rsx! {
        span { class: class,
            code { title: "{value}", "{short_hash(&value, 12, 8)}" }
            button { class: "copy-button", aria_label: "Copy {label}", title: "Copy {label}", onclick: move |_| copy_text(copy.clone()), "COPY" }
        }
    }
}

#[component]
pub fn ReadoutRow(label: String, value: String, detail: String) -> Element {
    rsx! {
        div { class: "readout-row",
            dt { "{label}" }
            dd { strong { "{value}" } span { class: "readout-detail", "{detail}" } }
        }
    }
}

#[component]
pub fn LineageRail(data: UiData, compact: bool) -> Element {
    let class = if compact {
        "lineage-rail lineage-rail-compact"
    } else {
        "lineage-rail"
    };
    let head_sequence = data.head().delta.sequence;
    rsx! {
        div { class: class, aria_label: "Canonical committed history",
            div { class: "lineage-line" }
            for transition in data.v2.transitions.iter() {
                Link { class: if transition.delta.sequence == head_sequence { "lineage-node lineage-node-current" } else { "lineage-node lineage-node-history" }, to: AppRoute::Transition { sequence: transition.delta.sequence }, key: "{transition.transition_id}",
                    span { class: if transition.delta.sequence == head_sequence { "node-number node-number-current" } else { "node-number node-number-history" }, "{transition.delta.sequence:02}" }
                    span { class: "node-copy", strong { "SEQ {transition.delta.sequence}" }, small { "COMMITTED" } }
                    span { class: "node-snapshot-label", "{data.snapshot_label(transition.delta.sequence).unwrap_or(\"Label unavailable\")}" }
                    code { "{short_hash(&transition.next_state_root, 8, 6)}" }
                }
            }
        }
    }
}

#[component]
pub fn HomeLineageRail(data: UiData) -> Element {
    let head_sequence = data.head().delta.sequence;
    let attack = data.v2.attack.as_ref();
    let attempted_sequence = attack
        .and_then(|value| value.attempted_sequence)
        .unwrap_or(head_sequence.saturating_add(1));
    let stale_root = attack
        .and_then(|value| value.stale_predecessor.as_deref())
        .unwrap_or("NOT AVAILABLE");
    rsx! {
        div { class: "lineage-rail home-lineage-rail", aria_label: "Canonical committed history and rejected continuation",
            div { class: "lineage-line" }
            for transition in data.v2.transitions.iter() {
                Link { class: if transition.delta.sequence == head_sequence { "lineage-node lineage-node-current" } else { "lineage-node lineage-node-history" }, to: AppRoute::Transition { sequence: transition.delta.sequence }, key: "home-{transition.transition_id}",
                    span { class: if transition.delta.sequence == head_sequence { "node-number node-number-current" } else { "node-number node-number-history" }, "{transition.delta.sequence:02}" }
                    span { class: "node-copy", strong { "SEQ {transition.delta.sequence}" }, small { "COMMITTED" } }
                    code { "{short_hash(&transition.next_state_root, 8, 6)}" }
                }
            }
            div { class: "lineage-node lineage-node-rejected",
                span { class: "node-number node-number-rejected", "{attempted_sequence:02}" }
                span { class: "node-copy", strong { "SEQ {attempted_sequence}" }, small { "REJECTED" } }
                span { class: "node-snapshot-label node-snapshot-label-rejected", "stale predecessor root" }
                code { "{short_hash(stale_root, 8, 6)}" }
            }
        }
    }
}

#[component]
pub fn SourceLine(source: String, note: String) -> Element {
    rsx! {
        div { class: "source-line", span { class: "source-dot source-dot-blue" }, strong { "{source}" }, span { "{note}" } }
    }
}

#[component]
pub fn CheckRow(label: String, value: String, tone: String) -> Element {
    rsx! { div { class: "check-row", span { "{label}" }, StatusBadge { label: value, tone } } }
}

#[component]
pub fn CommitmentRow(label: String, value: String) -> Element {
    rsx! { div { class: "commitment-row", span { "{label}" }, CopyValue { label: label.clone(), value, compact: false } } }
}

#[component]
pub fn EvidenceTableRow(label: String, value: String, note: String) -> Element {
    rsx! { div { class: "evidence-table-row", div { strong { "{label}" }, span { "{note}" } }, StatusBadge { label: value, tone: "verified".to_owned() } } }
}

#[component]
pub fn CommandRow(command: String, note: String) -> Element {
    rsx! { div { class: "command-row", code { "{command}" }, span { "{note}" } } }
}

#[component]
pub fn ProvenanceRow(label: String, detail: String, tone: String) -> Element {
    let mark = if tone == "verified" { "✓" } else { "•" };
    rsx! { div { class: "provenance-row", span { class: "provenance-mark provenance-mark-{tone}", "{mark}" }, div { strong { "{label}" }, span { "{detail}" } } } }
}

#[component]
pub fn HistoryEventTable(events: Vec<HistoryLedgerEvent>) -> Element {
    rsx! {
        div { class: "history-event-table-wrap",
            table { class: "history-event-table",
                thead {
                    tr {
                        th { "#" }
                        th { "Event" }
                        th { "Sequence" }
                        th { "Evidence source" }
                        th { "Root / result" }
                        th { "Status" }
                    }
                }
                tbody {
                    for event in events.iter() {
                        tr { key: "history-event-{event.order}",
                            td { class: "history-event-order", "{event.order:02}" }
                            td { strong { "{event.event}" } }
                            td { if let Some(sequence) = event.sequence { code { "{sequence}" } } else { "—" } }
                            td { class: "history-event-source", "{event.source}" }
                            td { code { title: "{event.reference}", "{short_hash(&event.reference, 10, 8)}" } }
                            td {
                                if event.status.starts_with("REJECTED") {
                                    StatusBadge { label: event.status.clone(), tone: "danger".to_owned() }
                                } else {
                                    StatusBadge { label: event.status.clone(), tone: "verified".to_owned() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "site-footer",
            span { "MEMORYLINEAGE / INSPECTOR" }
            span { "We prove history integrity. We do not claim semantic truth." }
            div { class: "footer-links",
                Link { to: AppRoute::Evidence {}, "Evidence" }
                Link { to: AppRoute::Security {}, "Security" }
                Link { to: AppRoute::Reproduce {}, "Reproduce" }
                Link { to: AppRoute::PriorWork {}, "Prior work" }
            }
        }
    }
}
