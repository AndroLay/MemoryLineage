#![forbid(unsafe_code)]

mod browser;
mod components;
mod data;
mod pages;

use components::{Footer, PageHeader, TopNavigation};
use data::{Route as NavRoute, Scenario, UiData};
use dioxus::prelude::*;
use pages::{
    ArchitecturePage, EvidencePage, HistoryPage, HomePage, InspectPage, LabPage, PriorWorkPage,
    ReproducePage, SecurityPage, TransitionPage, VerifyPage,
};

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum AppRoute {
    #[layout(AppShell)]
        #[route("/")]
        Home {},
        #[route("/inspect")]
        Inspect {},
        #[route("/history")]
        History {},
        #[route("/history/:sequence")]
        Transition { sequence: u64 },
        #[route("/lab")]
        Lab {},
        #[route("/lab/:scenario")]
        LabScenario { scenario: String },
        #[route("/verify")]
        Verify {},
        #[route("/evidence")]
        Evidence {},
        #[route("/architecture")]
        Architecture {},
        #[route("/security")]
        Security {},
        #[route("/reproduce")]
        Reproduce {},
        #[route("/prior-work")]
        PriorWork {},
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    use_context_provider(UiData::load);
    rsx! {
        style { "{include_str!(\"../assets/style.css\")}" }
        Router::<AppRoute> {}
    }
}

#[component]
fn AppShell() -> Element {
    let route = use_route::<AppRoute>();
    let nav_route = match &route {
        AppRoute::Home {} => NavRoute::Home,
        AppRoute::Inspect {} => NavRoute::Inspect,
        AppRoute::History {} => NavRoute::History,
        AppRoute::Transition { sequence } => NavRoute::Transition(*sequence),
        AppRoute::Lab {} => NavRoute::Lab(Scenario::SilentRollback),
        AppRoute::LabScenario { scenario } => NavRoute::Lab(Scenario::from_id(scenario)),
        AppRoute::Verify {} => NavRoute::Verify,
        AppRoute::Evidence {} => NavRoute::Evidence,
        AppRoute::Architecture {} => NavRoute::Architecture,
        AppRoute::Security {} => NavRoute::Security,
        AppRoute::Reproduce {} => NavRoute::Reproduce,
        AppRoute::PriorWork {} => NavRoute::PriorWork,
        AppRoute::NotFound { .. } => NavRoute::Home,
    };

    rsx! {
        div { class: "site-shell",
            TopNavigation { route: nav_route }
            main { class: "site-main",
                Outlet::<AppRoute> {}
            }
            Footer {}
        }
    }
}

#[component]
fn Home() -> Element {
    let data = use_context::<UiData>();
    rsx! { HomePage { data } }
}

#[component]
fn Inspect() -> Element {
    let data = use_context::<UiData>();
    rsx! { InspectPage { data } }
}

#[component]
fn History() -> Element {
    let data = use_context::<UiData>();
    rsx! { HistoryPage { data } }
}

#[component]
fn Transition(sequence: u64) -> Element {
    let data = use_context::<UiData>();
    rsx! { TransitionPage { data, sequence } }
}

#[component]
fn Lab() -> Element {
    let data = use_context::<UiData>();
    rsx! { LabPage { data, initial_scenario: Scenario::SilentRollback } }
}

#[component]
fn LabScenario(scenario: String) -> Element {
    let data = use_context::<UiData>();
    let initial_scenario = Scenario::from_id(&scenario);
    rsx! { LabPage { data, initial_scenario } }
}

#[component]
fn Verify() -> Element {
    let data = use_context::<UiData>();
    rsx! { VerifyPage { data } }
}

#[component]
fn Evidence() -> Element {
    let data = use_context::<UiData>();
    rsx! { EvidencePage { data } }
}

#[component]
fn Architecture() -> Element {
    let data = use_context::<UiData>();
    rsx! { ArchitecturePage { data } }
}

#[component]
fn Security() -> Element {
    let data = use_context::<UiData>();
    rsx! { SecurityPage { data } }
}

#[component]
fn Reproduce() -> Element {
    let data = use_context::<UiData>();
    rsx! { ReproducePage { data } }
}

#[component]
fn PriorWork() -> Element {
    let data = use_context::<UiData>();
    rsx! { PriorWorkPage { data } }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));
    rsx! {
        div { class: "page workspace-page",
            PageHeader {
                kicker: "ROUTE / NOT FOUND".to_owned(),
                title: "No matching audit surface".to_owned(),
                description: format!("The requested path {path} does not match a MemoryLineage page."),
                source: "UNKNOWN ROUTE".to_owned(),
            }
            Link { class: "button button-primary", to: AppRoute::Home {}, "Return to MemoryLineage →" }
        }
    }
}

#[cfg(test)]
mod route_tests {
    use super::AppRoute;
    use crate::data::Scenario;

    #[test]
    fn dioxus_routes_parse_all_required_paths() {
        let cases = [
            ("/", AppRoute::Home {}),
            ("/inspect", AppRoute::Inspect {}),
            ("/history", AppRoute::History {}),
            ("/history/3", AppRoute::Transition { sequence: 3 }),
            ("/lab", AppRoute::Lab {}),
            (
                "/lab/silent-rollback",
                AppRoute::LabScenario {
                    scenario: "silent-rollback".to_owned(),
                },
            ),
            (
                "/lab/sequence-gap",
                AppRoute::LabScenario {
                    scenario: "sequence-gap".to_owned(),
                },
            ),
            (
                "/lab/parallel-history",
                AppRoute::LabScenario {
                    scenario: "parallel-history".to_owned(),
                },
            ),
            (
                "/lab/wrong-eoa-signer",
                AppRoute::LabScenario {
                    scenario: "wrong-eoa-signer".to_owned(),
                },
            ),
            (
                "/lab/locator-binding",
                AppRoute::LabScenario {
                    scenario: "locator-binding".to_owned(),
                },
            ),
            (
                "/lab/wrong-chain-domain",
                AppRoute::LabScenario {
                    scenario: "wrong-chain-domain".to_owned(),
                },
            ),
            (
                "/lab/semantic-poisoning",
                AppRoute::LabScenario {
                    scenario: "semantic-poisoning".to_owned(),
                },
            ),
            ("/verify", AppRoute::Verify {}),
            ("/evidence", AppRoute::Evidence {}),
            ("/architecture", AppRoute::Architecture {}),
            ("/security", AppRoute::Security {}),
            ("/reproduce", AppRoute::Reproduce {}),
            ("/prior-work", AppRoute::PriorWork {}),
        ];

        for (path, expected) in cases {
            assert_eq!(path.parse::<AppRoute>().ok(), Some(expected), "{path}");
        }
    }

    #[test]
    fn semantic_poisoning_route_stays_out_of_scope_scenario() {
        assert_eq!(
            Scenario::from_id("semantic-poisoning"),
            Scenario::SemanticPoisoning
        );
    }
}
