// SPDX-License-Identifier: GPL-3.0-or-later
#![allow(non_snake_case)]
// Modo release Windows
#![windows_subsystem = "windows"]

mod resources;
use std::collections::HashMap;

use resources::*;

mod components;
use components::*;

mod models;
pub use models::*;

mod utils;
pub use utils::*;

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/disks")]
    Disks {},
    #[route("/load_files")]
    LoadFiles {},
    #[route("/profiles")]
    Profiles {},
    #[route("/profile_filter")]
    ProfileFilter {},
    #[route("/add_filter/:profile_id")]
    ModifyFilter { profile_id: usize },
    #[route("/filter_files")]
    FilterFiles {},
    #[route("/results")]
    Results {},
    #[route("/results/:id")]
    DetailedResults { id: usize },
}

pub struct AppState {
    disks: Vec<Disk>,
    files: Vec<FileEntry>,
    profiles: Vec<Profile>,
    classifying_progress: SyncSignal<u8>,
    hashing_progress: SyncSignal<u8>,
    selected_file: Option<FileEntry>,
    thumbnail_generated: bool
}

pub struct ModalInfo {
    title: String,
    content: String,
}

pub struct Report {
    selected_disks: Vec<Disk>,
    selected_profiles: Vec<Profile>,
    general_filter: Option<FilterOptions>,
    specific_filters: HashMap<usize, FilterOptions>
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    // Launch the app
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| {
        Signal::new(AppState {
            disks: vec![],
            files: vec![],
            profiles: vec![],
            classifying_progress: SyncSignal::new_maybe_sync(0),
            hashing_progress: SyncSignal::new_maybe_sync(0),
            selected_file: None,
            thumbnail_generated: false,
        })
    });
    use_context_provider(|| {
        Signal::new(Report {
            selected_disks: vec![],
            selected_profiles: vec![],
            general_filter: None,
            specific_filters: HashMap::new()
        })
    });
    use_context_provider(|| {
        Signal::new(ModalInfo {
            title: "".to_string(),
            content: "".to_string(),
        })
    });

    let modal_info = use_context::<Signal<ModalInfo>>();
    let modal_info_title = modal_info.read().title.clone();
    let modal_info_content = modal_info.read().content.clone();

    rsx! {
        style {
            { TAILWIND_CSS_URL }
        }

        main {
            class: "h-dvh w-dvh",
            Router::<Route> { }
        }

        dialog {
            class: "modal modal-bottom sm:modal-middle",
            id: "error_modal",
            div {
                role: "alert",
                class: "modal-box alert-error bg-red-400",
                div {
                    h3 {
                        class: "font-bold text-lg",
                        { modal_info_title.clone() }
                    }
                }
                div {
                    class: "flex my-4",
                    svg {
                        "fill": "none",
                        "xmlns": "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        class: "stroke-current shrink-0 h-6 w-6 mr-1",
                        path {
                            "d": "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2"
                        }
                    }
                    span {
                        { modal_info_content.clone() }
                    }
                }
                div {
                    class: "modal-action",
                    form {
                        onsubmit: |_| {
                            eval(r#"error_modal.close()"#);
                        },
                        button {
                            class: "btn btn-neutral",
                            "Cerrar"
                        }
                    }
                }
            }
        }

        dialog {
            class: "modal modal-bottom sm:modal-middle",
            id: "success_modal",
            div {
                role: "alert",
                class: "modal-box alert-success",
                div {
                    h3 {
                        class: "font-bold text-lg",
                        { modal_info_title }
                    }
                }
                div {
                    class: "flex my-4",
                    svg {
                        "xmlns": "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        "fill": "none",
                        class: "h-6 w-6 shrink-0 stroke-current",
                        path {
                            "stroke-linejoin": "round",
                            "stroke-linecap": "round",
                            "d": "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
                            "stroke-width": "2"
                        }
                    }
                    span {
                        { modal_info_content }
                    }
                }
                div {
                    class: "modal-action",
                    form {
                        onsubmit: |_| {
                            eval(r#"success_modal.close()"#);
                        },
                        button {
                            class: "btn btn-neutral",
                            "Cerrar"
                        }
                    }
                }
            }
        }
    }
}
