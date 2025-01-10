// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;
use tokio::time::sleep;

use crate::{calculate_hashes, AppState, Report, Route};

#[component]
pub fn FilterFiles() -> Element {
    let nav = navigator();

    let app_state = use_context::<Signal<AppState>>();
    let report = use_context::<Signal<Report>>();

    let loading_status = use_signal(|| String::from("Cargando archivos..."));

    let future =
        use_resource(move || async move { filter_files(app_state, report, loading_status).await });

    if future.read_unchecked().as_ref().is_some() {
        spawn_forever(calculate_hashes(app_state, report));
        nav.push(Route::Results {});
    }

    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            div {
                class: "loading loading-spinner loading-lg text-primary"
            }
            p {
                class: "font-semibold text-lg",
                { loading_status }
            }
        }
    }
}

async fn filter_files(
    app_state: Signal<AppState>,
    mut report: Signal<Report>,
    mut loading_status: Signal<String>,
) {
    let classiying_progress = app_state.peek().classifying_progress;

    while classiying_progress() != 100 {
        *loading_status.write() =
            format!("Clasificando archivos ({} %)", classiying_progress.peek());
        sleep(std::time::Duration::from_secs(1)).await;
    }

    let profiles = app_state.peek().profiles.clone();
    let mut report_profiles = report.peek().selected_profiles.clone();

    let profiles_len = report_profiles.len();
    for (i, profile) in report_profiles.iter_mut().enumerate() {
        *loading_status.write() = format!(
            "Cargando archivos del perfil: {} ({}/{})",
            profile.name(),
            i + 1,
            profiles_len
        );

        let filtered_files = match profiles.iter().find(|p| p.id() == profile.id()) {
            Some(p) => p.files(),
            None => {
                continue;
            }
        };

        profile.set_files(filtered_files.clone());
    }

    *loading_status.write() = String::from("Aplicando parámetros de filtrado");
    let general_filter = report.peek().general_filter.clone();
    let specific_filters = report.peek().specific_filters.clone();

    for profile in report_profiles.iter_mut() {
        if let Some(filter_options) = specific_filters.get(&profile.id()) {
            profile.set_filter_options(filter_options);
        } else if let Some(filter_options) = &general_filter {
            profile.set_filter_options(filter_options);
        }

        profile.filter_files();
    }

    report.write().selected_profiles = report_profiles;
}
