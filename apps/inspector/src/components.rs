#![forbid(unsafe_code)]

use crate::browser::copy_text;
use crate::data::{Route, UiData, short_hash};
use dioxus::prelude::*;

#[component]
pub fn TopNavigation(route: Route) -> Element {
    let active = |candidate: Route| {
        if route == candidate {
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
        header { class: "topbar",
            a { class: "brand", href: "/", aria_label: "MemoryLineage home",
                span { class: "brand-mark",
                    img { class: "brand-logo", src: asset!("/assets/memorylineage-logo.png"), alt: "MemoryLineage logo" }
                }
                span { strong { "MEMORYLINEAGE" } small { "INDEPENDENT MEMORY AUDITOR" } }
            }
            nav { class: "primary-nav", aria_label: "Primary navigation",
                a { class: active(Route::Inspect), href: "/inspect", "Inspect" }
                a { class: active(Route::History), href: "/history", "History" }
                a { class: lab_class, href: "/lab", "Tampering Lab" }
                a { class: active(Route::Verify), href: "/verify", "Verify" }
            }
            div { class: "topbar-meta",
                span { class: "source-chip source-observed", span { class: "source-dot" }, "SEPOLIA / READ-ONLY" }
                span { class: "topbar-muted", "GITHUB / NOT CONFIGURED" }
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
    rsx! {
        div { class: class, aria_label: "Canonical committed history",
            div { class: "lineage-line" }
            for transition in data.bundle.valid_history.iter() {
                a { class: "lineage-node", href: "/history/{transition.delta.sequence}", key: "{transition.transition_id}",
                    span { class: "node-number", "{transition.delta.sequence:02}" }
                    span { class: "node-copy", strong { "SEQ {transition.delta.sequence}" }, small { "COMMITTED" } }
                    code { "{short_hash(&transition.next_state_root, 8, 6)}" }
                }
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
pub fn Footer() -> Element {
    rsx! {
        footer { class: "site-footer",
            span { "MEMORYLINEAGE / INSPECTOR" }
            span { "We prove history integrity. We do not claim semantic truth." }
            div { class: "footer-links", a { href: "/evidence", "Evidence" }, a { href: "/security", "Security" }, a { href: "/reproduce", "Reproduce" } }
        }
    }
}
