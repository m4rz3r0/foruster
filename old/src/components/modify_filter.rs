// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;

use crate::{bytes_to_mb, mb_to_bytes, FilterOptions, Report};

#[component]
pub fn ModifyFilter(profile_id: usize) -> Element {
    let nav = navigator();

    let report = use_context::<Signal<Report>>();
    let profiles = report.peek().selected_profiles.clone();
    let selected_profile = profiles.iter().find(|p| p.id() == profile_id).unwrap();

    let name = selected_profile.name();
    let filter_options = match selected_profile.filter_options() {
        Some(options) => options.clone(),
        None => FilterOptions::new(),
    };

    let min_size_value = match filter_options.min_size() {
        Some(size) => bytes_to_mb(size).to_string(),
        None => String::new(),
    };

    let max_size_value = match filter_options.max_size() {
        Some(size) => bytes_to_mb(size).to_string(),
        None => String::new(),
    };

    rsx! {
        div {
            class: "flex flex-col w-full",
            h1 {
                class: "text-center text-2xl font-bold",
                "Configura los parámetros de filtrado para el perfil {name}:"
            }
            form {
                class: "flex flex-col justify-evenly items-center w-full h-auto",
                onsubmit: move |event| {
                    update_profile_filter(report, profile_id, event);
                    nav.go_back();
                },
                label {
                    class: "input input-bordered flex items-center gap-2 m-4",
                    input {
                        class: "grow",
                        r#type: "number",
                        name: "min_value",
                        step: "0.01",
                        placeholder: "Tamaño minimo (MB)",
                        min: 0,
                        value: "{min_size_value}"
                    }
                    span {
                        class: "badge badge-info",
                        "Opcional"
                    }
                }
                label {
                    class: "input input-bordered flex items-center gap-2 m-4",
                    input {
                        class: "grow",
                        r#type: "number",
                        name: "max_value",
                        step: "0.01",
                        placeholder: "Tamaño máximo (MB)",
                        min: 0,
                        value: "{max_size_value}"
                    }
                    span {
                        class: "badge badge-info",
                        "Opcional"
                    }
                }

                button {
                    class: "btn btn-primary",
                    r#type: "submit",
                    "Guardar"
                }
            }
        }
    }
}

fn update_profile_filter(mut report: Signal<Report>, profile_id: usize, event: FormEvent) {
    let mut specific_filters = report.peek().specific_filters.clone();

    let values = event.values();

    let min_size = values.get("min_size").and_then(|form_value| {
        form_value
            .first()
            .and_then(|value| match value.parse::<f32>() {
                Ok(size) => Some(mb_to_bytes(size)),
                Err(_) => None,
            })
    });

    let max_size = values.get("max_size").and_then(|form_value| {
        form_value
            .first()
            .and_then(|value| match value.parse::<f32>() {
                Ok(size) => Some(mb_to_bytes(size)),
                Err(_) => None,
            })
    });

    let filter_options = FilterOptions::with_more_details(min_size, max_size);

    if let Some(filter) = specific_filters.get_mut(&profile_id) {
        *filter = filter_options;
    }

    report.write().specific_filters = specific_filters;
}
