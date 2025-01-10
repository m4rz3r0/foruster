// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;

use crate::{get_disks, get_profiles, AppState};

#[component]
pub fn Home() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    let mut disks_loading_status = use_signal(|| String::from("Cargando discos"));
    let future_disks = use_resource(move || async move {
        match tokio::task::spawn_blocking(get_disks).await {
            Ok(disks_result) => match disks_result {
                Ok(disks) => {
                    *disks_loading_status.write() = String::from("Discos cargados");
                    disks
                }
                Err(_) => Vec::new(),
            },
            Err(_) => Vec::new(),
        }
    });

    let mut profiles_loading_status = use_signal(|| String::from("Cargando perfiles"));
    let future_profiles = use_resource(move || async move {
        match tokio::task::spawn_blocking(get_profiles).await {
            Ok(profiles_result) => match profiles_result {
                Ok(profiles) => {
                    *profiles_loading_status.write() = String::from("Perfiles cargados");
                    profiles
                }
                Err(_) => Vec::new(),
            },
            Err(_) => Vec::new(),
        }
    });

    use_effect(move || {
        if let Some(disks) = future_disks.read().as_ref() {
            if app_state.peek().disks.is_empty() {
                app_state.write().disks.clone_from(disks);
            }
        }
    });

    use_effect(move || {
        if let Some(profiles) = future_profiles.read().as_ref() {
            if app_state.peek().profiles.is_empty() {
                app_state.write().profiles.clone_from(profiles);
            }
        }
    });

    use_effect(move || {
        if !app_state.read().profiles.is_empty() && !app_state.read().disks.is_empty() {
            navigator().push(crate::Route::Disks {  });
        }
    });

    match (
        future_disks.read_unchecked().as_ref(),
        future_profiles.read_unchecked().as_ref(),
    ) {
        (Some(_), Some(_)) => {
            rsx! {
                div {
                    class: "flex items-center justify-center h-full",
                    Link {
                        to: crate::Route::Disks {},
                        class: "btn btn-primary m-4",
                        "Ver discos disponibles"
                    }
                }
            }
        }
        (f_disks, f_profiles) => {
            rsx! {
                div {
                    class: "flex flex-col items-center justify-center h-screen",
                    div {
                        class: "flex items-center justify-center m-4",
                        if f_disks.is_none() {
                            div {
                                class: "loading loading-spinner text-primary"
                            }
                        }
                        h1 {
                            class: "font-semibold text-lg",
                            { disks_loading_status }
                        }
                    }
                    div {
                        class: "flex items-center justify-center m-4",
                        if f_profiles.is_none() {
                            div {
                                class: "loading loading-spinner text-primary"
                            }
                        }
                        h1 {
                            class: "font-semibold text-lg",
                            { profiles_loading_status }
                        }
                    }
                }
            }
        }
    }
}
