#![forbid(unsafe_code)]

mod browser;
mod components;
mod data;
mod pages;

use components::{Footer, PageHeader, TopNavigation};
use data::{Route as NavRoute, Scenario, UiData};
use dioxus::prelude::*;
use pages::{
    ArchitecturePage, ChallengePage, EvidencePage, HistoryPage, HomePage, InspectPage, LabPage,
    OverviewPage, PriorWorkPage, ReproducePage, SecurityPage, TransitionPage, VerifyPage,
    WelcomePage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TourStep {
    Incident,
    Challenge,
    Inspect,
    History,
    Lab,
    Verify,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct TourProgress {
    pub(crate) challenge_checked: bool,
    pub(crate) rollback_replayed: bool,
}

impl TourStep {
    const ALL: [Self; 6] = [
        Self::Incident,
        Self::Challenge,
        Self::Inspect,
        Self::History,
        Self::Lab,
        Self::Verify,
    ];

    fn index(self) -> usize {
        Self::ALL.iter().position(|step| *step == self).unwrap_or(0)
    }

    fn previous(self) -> Option<Self> {
        self.index().checked_sub(1).map(|index| Self::ALL[index])
    }

    fn next(self) -> Option<Self> {
        Self::ALL.get(self.index() + 1).copied()
    }

    fn route(self) -> AppRoute {
        match self {
            Self::Incident => AppRoute::Home {},
            Self::Challenge => AppRoute::Challenge {},
            Self::Inspect => AppRoute::Inspect {},
            Self::History => AppRoute::History {},
            Self::Lab => AppRoute::Lab {},
            Self::Verify => AppRoute::Verify {},
        }
    }

    fn for_route(route: &AppRoute) -> Option<Self> {
        match route {
            AppRoute::Home {} => Some(Self::Incident),
            AppRoute::Challenge {} => Some(Self::Challenge),
            AppRoute::Inspect {} => Some(Self::Inspect),
            AppRoute::History {} => Some(Self::History),
            AppRoute::Lab {} | AppRoute::LabScenario { .. } => Some(Self::Lab),
            AppRoute::Verify {} => Some(Self::Verify),
            AppRoute::Welcome {}
            | AppRoute::Overview {}
            | AppRoute::Transition { .. }
            | AppRoute::Evidence {}
            | AppRoute::Architecture {}
            | AppRoute::Security {}
            | AppRoute::Reproduce {}
            | AppRoute::PriorWork {}
            | AppRoute::NotFound { .. } => None,
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Incident => "Start with the restore question",
            Self::Challenge => "Make a quick decision",
            Self::Inspect => "Check the current backup",
            Self::History => "Follow the shared history",
            Self::Lab => "Replay the old restore attempt",
            Self::Verify => "Review the evidence",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Incident => {
                "Compare the backup that was restored with the latest shared backup. This example uses synthetic local evidence."
            }
            Self::Challenge => {
                "Choose whether the restored backup can continue, then check your answer against the evidence."
            }
            Self::Inspect => {
                "Inspect the current head and see how an older checkpoint is classified before recovery."
            }
            Self::History => {
                "Follow the recorded checkpoints in order and see which one represents the latest shared state."
            }
            Self::Lab => {
                "Replay the stale-restore case. The check succeeds; the attempt to continue from an old backup is rejected."
            }
            Self::Verify => {
                "Review the portable evidence and its replay result. This completes the main MemoryLineage flow."
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum AppRoute {
    #[layout(AppShell)]
        #[route("/")]
        Welcome {},
        #[route("/app")]
        Home {},
        #[route("/challenge")]
        Challenge {},
        #[route("/overview")]
        Overview {},
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
    use_context_provider(|| Signal::new(None::<TourStep>));
    use_context_provider(|| Signal::new(TourProgress::default()));
    rsx! {
        style { "{include_str!(\"../assets/style.css\")}" }
        Router::<AppRoute> {}
    }
}

#[component]
fn AppShell() -> Element {
    let route = use_route::<AppRoute>();
    let mut guided_tour = use_context::<Signal<Option<TourStep>>>();
    let tour_step = guided_tour();
    let show_primary_navigation = !matches!(&route, AppRoute::Welcome {} | AppRoute::Challenge {});
    let route_for_tour = route.clone();
    use_effect(move || {
        let current_step = guided_tour();
        let route_step = TourStep::for_route(&route_for_tour);
        if current_step.is_some() && current_step != route_step {
            guided_tour.set(route_step);
        }
    });
    let nav_route = match &route {
        AppRoute::Welcome {} => NavRoute::Home,
        AppRoute::Home {} => NavRoute::Home,
        AppRoute::Challenge {} | AppRoute::Overview {} => NavRoute::Home,
        AppRoute::Inspect {} => NavRoute::Inspect,
        AppRoute::History {} => NavRoute::History,
        AppRoute::Transition { sequence } => NavRoute::Transition(*sequence),
        AppRoute::Lab {} => NavRoute::Lab(Scenario::SilentRollback),
        AppRoute::LabScenario { scenario } => Scenario::from_id(scenario)
            .map(NavRoute::Lab)
            .unwrap_or(NavRoute::Home),
        AppRoute::Verify {} => NavRoute::Verify,
        AppRoute::Evidence {} => NavRoute::Evidence,
        AppRoute::Architecture {} => NavRoute::Architecture,
        AppRoute::Security {} => NavRoute::Security,
        AppRoute::Reproduce {} => NavRoute::Reproduce,
        AppRoute::PriorWork {} => NavRoute::PriorWork,
        AppRoute::NotFound { .. } => NavRoute::Home,
    };

    rsx! {
        div { class: if tour_step.is_some() { "site-shell guided-tour-active" } else { "site-shell" },
            TopNavigation {
                route: nav_route,
                show_primary: show_primary_navigation && tour_step.is_none(),
                landing: matches!(&route, AppRoute::Welcome {}),
            }
            main { class: "site-main",
                Outlet::<AppRoute> {}
            }
            if tour_step.is_none() && !matches!(&route, AppRoute::Welcome {}) { Footer {} }
            if let Some(step) = tour_step {
                GuidedTourPanel { step }
            }
        }
    }
}

#[component]
fn Welcome() -> Element {
    let data = use_context::<UiData>();
    rsx! { WelcomePage { data } }
}

#[component]
fn GuidedTourPanel(step: TourStep) -> Element {
    let mut guided_tour = use_context::<Signal<Option<TourStep>>>();
    let navigator = use_navigator();
    let tour_progress = use_context::<Signal<TourProgress>>();
    let progress = tour_progress();
    let index = step.index();
    let total = TourStep::ALL.len();
    let previous = step.previous();
    let next = step.next();
    let can_continue = match step {
        TourStep::Challenge => progress.challenge_checked,
        TourStep::Lab => progress.rollback_replayed,
        _ => true,
    };
    let continue_hint = match step {
        TourStep::Challenge if !progress.challenge_checked => {
            Some("Choose an answer and check it to continue.")
        }
        TourStep::Lab if !progress.rollback_replayed => {
            Some("Run the local rollback check to continue.")
        }
        _ => None,
    };
    let progress_scale = (index + 1) as f64 / total as f64;
    rsx! {
        aside { class: "guided-tour-panel", role: "region", aria_label: "Guided MemoryLineage tour",
            div { class: "guided-tour-progress-row",
                span { class: "guided-tour-kicker", "GUIDED TOUR" }
                span { class: "guided-tour-count", "STEP {index + 1} OF {total}" }
            }
            div { class: "guided-tour-progress", role: "progressbar", aria_label: "Tour progress", aria_valuemin: "1", aria_valuemax: "{total}", aria_valuenow: "{index + 1}",
                span { style: "transform: scaleX({progress_scale})" }
            }
            h2 { "{step.title()}" }
            p { "{step.description()}" }
            if let Some(hint) = continue_hint {
                p { class: "guided-tour-hint", "{hint}" }
            }
            div { class: "guided-tour-actions",
                button { class: "button button-quiet guided-tour-exit", r#type: "button", onclick: move |_| guided_tour.set(None), "Exit guide" }
                div { class: "guided-tour-step-actions",
                    if let Some(previous_step) = previous {
                        button { class: "button button-secondary", r#type: "button", onclick: move |_| { guided_tour.set(Some(previous_step)); navigator.push(previous_step.route()); }, "Back" }
                    }
                    if let Some(next_step) = next {
                        button { class: "button button-primary", r#type: "button", disabled: !can_continue, onclick: move |_| { guided_tour.set(Some(next_step)); navigator.push(next_step.route()); }, "Next" }
                    } else {
                        button { class: "button button-primary", r#type: "button", onclick: move |_| guided_tour.set(None), "Finish tour" }
                    }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let data = use_context::<UiData>();
    rsx! { HomePage { data } }
}

#[component]
fn Challenge() -> Element {
    let data = use_context::<UiData>();
    rsx! { ChallengePage { data } }
}

#[component]
fn Overview() -> Element {
    let data = use_context::<UiData>();
    rsx! { OverviewPage { data } }
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
    match Scenario::from_id(&scenario) {
        Some(initial_scenario) => rsx! { LabPage { data, initial_scenario } },
        None => rsx! {
            div { class: "page workspace-page",
                PageHeader {
                    kicker: "TAMPERING LAB / NOT FOUND".to_owned(),
                    title: "Unknown tampering scenario".to_owned(),
                    description: format!("No lab scenario is registered for the slug '{scenario}'."),
                    source: "UNKNOWN SCENARIO".to_owned(),
                }
                Link { class: "button button-primary", to: AppRoute::Lab {}, "Open the Tampering Lab →" }
            }
        },
    }
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
            ("/", AppRoute::Welcome {}),
            ("/app", AppRoute::Home {}),
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
            (
                "/lab/unknown-case",
                AppRoute::LabScenario {
                    scenario: "unknown-case".to_owned(),
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
            Some(Scenario::SemanticPoisoning)
        );
    }
}
