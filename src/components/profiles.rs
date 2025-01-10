// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;
use dioxus_free_icons::Icon;

use crate::{
    bytes_to_mb, mb_to_bytes, show_error, show_success, AppState, FilterOptions, ModalInfo, Profile, ProfileType, Report
};

#[component]
pub fn Profiles() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let report = use_context::<Signal<Report>>();
    let modal_info = use_context::<Signal<ModalInfo>>();

    let classifying_progress = app_state.peek().classifying_progress;
    let profiles = app_state.peek().profiles.clone();
    let profiles_info = profiles.iter().map(|profile| {
        rsx! {
            ProfileCard {
                profile: profile.clone(),
                is_form: true
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-row w-full h-full",

            // Contenido principal a la izquierda
            div {
                class: "flex flex-col w-3/4 text-center",
                div {
                    class: "flex flex-col text-center justify-center",
                    p {
                        class: "text-center font-bold m-4",
                        "Progreso del clasificado ({classifying_progress} %)"
                    }
                    progress { max: "100", value: "{classifying_progress}", class: "progress progress-primary w-full" }
                }

                h1 {
                    class: "text-center text-4xl font-bold m-4",
                    "Seleccione los perfiles a ejecutar"
                }

                form {
                    onsubmit: move |event| {
                        match save_profiles(report, profiles.clone(), event) {
                            Ok(_) => {
                                navigator().push(crate::Route::FilterFiles {  });
                            }
                            Err(err) => {
                                show_error(use_context::<Signal<ModalInfo>>(), &err);
                            }
                        }
                    },
                    div {
                        class: "flex flex-wrap justify-center",
                        { profiles_info }
                    }
                    button {
                        class: "btn btn-primary m-4",
                        r#type: "submit",
                        "Ejecutar perfiles"
                    }
                }
            }

            // Menú lateral derecho
            div {
                class: "w-1/4 border-l p-4",
                h2 {
                    class: "text-xl font-bold mb-4",
                    "Opciones de Filtrado"
                }
                form {
                    onsubmit: move |event| {
                        save_general_filter(report, event);
                        show_success(modal_info, "Filtros guardados correctamente");
                    },
                    div {
                        class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700",
                            "Tamaño mínimo (MB):"
                        }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm p-1",
                            r#type: "number",
                            name: "min_size",
                            step: "0.01",
                            placeholder: "Tamaño minimo (MB)",
                            min: 0,
                        }
                    }
                    div {
                        class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700",
                            "Tamaño máximo (MB):"
                        }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm p-1",
                            r#type: "number",
                            name: "max_size",
                            step: "0.01",
                            placeholder: "Tamaño máximo (MB)",
                            min: 0,
                        }
                    }
                    button {
                        class: "btn btn-secondary",
                        r#type: "submit",
                        "Aplicar Filtros"
                    }
                }
            }
        }
    }
}

fn profile_form_input(id: i64, is_form: bool) -> Element {
    rsx! {
        input {
            id,
            name: id,
            r#type: "checkbox",
            class: "hidden peer",
            checked: !is_form,
            disabled: !is_form
        }
    }
}

pub fn ProfileCardBody(profile: &Profile) -> Element {
    let name = profile.name();
    let extensions = profile.extensions();

    rsx! {
        div {
            class: "card-body",
            h2 {
                class: "card-title justify-center",
                { name }
            }
            if !extensions.is_empty() {
                p {
                    strong {
                        "Extensiones: "
                    }
                    { extensions.join(", ") }
                }
            }
            if let Some(filter_options) = profile.filter_options() {
                div {
                    class: "card-footer",
                    h2 {
                        class: "text-xl font-bold",
                        "Parámetros de filtrado:"
                    }
                    if let Some(min_size) = filter_options.min_size() {
                        p {
                            strong {
                                "Tamaño mínimo del archivo: "
                            }
                            { bytes_to_mb(min_size).to_string() },
                            " MB"
                        }
                    }
                    if let Some(max_size) = filter_options.max_size() {
                        p {
                            strong {
                                "Tamaño máximo del archivo: "
                            }
                            { bytes_to_mb(max_size).to_string() },
                            " MB"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProfileCard(profile: Profile, is_form: bool) -> Element {
    let checked_bg_color;
    let icon;

    let id = profile.id() as i64;
    let card_body = ProfileCardBody(&profile);

    match profile.profile_type() {
        ProfileType::Image => {
            checked_bg_color = "has-[:checked]:bg-blue-500";
            icon = rsx! {
                Icon {
                icon: dioxus_free_icons::icons::bs_icons::BsImage
                }
            };
        },
        ProfileType::Video => {
            checked_bg_color = "has-[:checked]:bg-black";
            icon = rsx! {
                Icon {
                    icon: dioxus_free_icons::icons::bs_icons::BsFilm
                }
            };
        },
        ProfileType::Audio => {
            checked_bg_color = "has-[:checked]:bg-green-500";
            icon = rsx! {
                Icon {
                    icon: dioxus_free_icons::icons::bs_icons::BsMusicNote
                }
            };
        },
        ProfileType::Archive => {
            checked_bg_color = "has-[:checked]:bg-yellow-900";
            icon = rsx! {
                Icon {
                    icon: dioxus_free_icons::icons::bs_icons::BsArchive
                }
            };
        },
        ProfileType::Book => {
            checked_bg_color = "has-[:checked]:bg-purple-500";
            icon = rsx! {
                Icon {
                    icon: dioxus_free_icons::icons::bs_icons::BsBook
                }
            };
        },
        ProfileType::Document => {
            checked_bg_color = "has-[:checked]:bg-yellow-500";
            icon = rsx! {
                Icon {
                    icon: dioxus_free_icons::icons::bs_icons::BsFileEarmarkText
                }
            };
        },
        ProfileType::Application => {
            checked_bg_color = "has-[:checked]:bg-cyan-500";
            icon = rsx! {
                Icon {
                    icon: dioxus_free_icons::icons::bs_icons::BsWindow
                }
            };
        },
        ProfileType::Custom => {
            checked_bg_color = "has-[:checked]:bg-pink-500";
            icon = rsx! {
                Icon {
                    icon: dioxus_free_icons::icons::bs_icons::BsGear
                }
            };
        },
    }

    rsx! {
        label {
            r#for: id,
            class: "card w-48 shadow-xl m-4 cursor-pointer {checked_bg_color} has-[:checked]:text-white glass",
            { profile_form_input(id, is_form) }
            div {
                class: "card-actions justify-start mt-4 ml-4",
                { icon }
            }
            { card_body }
        }
    }
}

fn save_profiles(
    mut report: Signal<Report>,
    profiles: Vec<Profile>,
    event: FormEvent,
) -> Result<(), String> {
    let selected = event
        .values()
        .iter()
        .filter_map(|(key, value)| {
            if value == "on" {
                key.parse::<usize>().ok()
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    if selected.is_empty() {
        return Err("No se ha seleccionado ningún perfil".to_string());
    }

    let selected_profiles = profiles
        .into_iter()
        .filter(|profile| selected.contains(&profile.id()))
        .collect::<Vec<Profile>>();

    report.write().selected_profiles = selected_profiles;

    Ok(())
}

fn save_general_filter(mut report: Signal<Report>, event: FormEvent) {
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

    if min_size.is_none() && max_size.is_none() {
        return;
    }
    let general_filter = Some(FilterOptions::with_more_details(min_size, max_size));

    report.write().general_filter = general_filter;
}
