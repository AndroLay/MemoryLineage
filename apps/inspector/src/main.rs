#![forbid(unsafe_code)]

mod browser;
mod components;
mod data;
mod pages;

use components::{Footer, TopNavigation};
use data::{Route, UiData};
use dioxus::prelude::*;
use pages::{
    ArchitecturePage, EvidencePage, HistoryPage, HomePage, InspectPage, LabPage, PriorWorkPage,
    ReproducePage, SecurityPage, TransitionPage, VerifyPage,
};

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let data = UiData::load();
    let route = use_signal(|| Route::from_path(&browser::browser_path()));
    let current_route = route();
    rsx! {
        style { "{include_str!(\"../assets/style.css\")}" }
        div { class: "site-shell",
            TopNavigation { route: current_route }
            main { class: "site-main",
                {match current_route {
                    Route::Home => rsx! { HomePage { data: data.clone() } },
                    Route::Inspect => rsx! { InspectPage { data: data.clone() } },
                    Route::History => rsx! { HistoryPage { data: data.clone() } },
                    Route::Transition(sequence) => rsx! { TransitionPage { data: data.clone(), sequence } },
                    Route::Lab(scenario) => rsx! { LabPage { data: data.clone(), initial_scenario: scenario } },
                    Route::Verify => rsx! { VerifyPage { data: data.clone() } },
                    Route::Evidence => rsx! { EvidencePage { data: data.clone() } },
                    Route::Architecture => rsx! { ArchitecturePage { data: data.clone() } },
                    Route::Security => rsx! { SecurityPage {} },
                    Route::Reproduce => rsx! { ReproducePage { data: data.clone() } },
                    Route::PriorWork => rsx! { PriorWorkPage { data: data.clone() } },
                }}
            }
            Footer {}
        }
    }
}
