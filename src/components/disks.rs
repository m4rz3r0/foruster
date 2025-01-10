// SPDX-License-Identifier: GPL-3.0-or-later
use crate::{
    format_size, show_error, AppState, Disk, DiskKind, ModalInfo, Report, HARD_DRIVE_IMAGE,
    PENDRIVE_USB_IMAGE,
};
use dioxus::prelude::*;

#[component]
pub fn Disks() -> Element {
    let nav = navigator();

    let app_state = use_context::<Signal<AppState>>();
    let report = use_context::<Signal<Report>>();

    let disks = app_state.peek().disks.clone();
    let disks_info = disks.clone().into_iter().map(|disk| {
        let id = disk.device_name().to_string();
        rsx! {
            label {
                r#for: id.to_string(),
                class: "card lg:card-side shadow-xl m-8 cursor-pointer w-1/2 lg:w-4/5 has-[:checked]:ring-indigo-600 ring-2 ring-transparent glass",
                input {
                    class: "checkbox m-4 card-actions justify-end hidden peer",
                    r#type: "checkbox",
                    id: id.to_string(),
                    name: id.to_string()
                }
                div {
                    class: "invisible peer-checked:visible m-4",
                    svg {
                        "fill": "none",
                        class: "h-5 w-5 flex-none",
                        path {
                            "d": "M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16Zm3.707-9.293a1 1 0 0 0-1.414-1.414L9 10.586 7.707 9.293a1 1 0 0 0-1.414 1.414l2 2a1 1 0 0 0 1.414 0l4-4Z",
                            "clip-rule": "evenodd",
                            "fill-rule": "evenodd",
                            "fill": "#4F46E5"
                        }
                    }
                }
                DiskCard {
                    disk
                }
            }
        }
        })
    ;

    rsx! {
        div {
            class: "flex flex-col items-center justify-center w-full",
            h1 {
                class: "text-4xl font-bold m-4",
                "Seleccione los discos a analizar:"
            }
            form {
                onsubmit: move |event| {
                    match save_disks(report, disks.clone(), event) {
                        Ok(_) => {
                            nav.push(crate::Route::LoadFiles {});
                        }
                        Err(err) => {
                            show_error(use_context::<Signal<ModalInfo>>(), &err);
                        }
                    }
                },
                class: "flex flex-col items-center justify-center w-full",
                { disks_info },
                button {
                    class: "btn btn-primary m-4",
                    r#type: "submit",
                    "Analizar"
                }
            }
        }
    }
}

#[component]
pub fn DiskCard(disk: Disk) -> Element {
    rsx! {
        figure {
            img {
                alt: "Disco duro",
                src: if disk.removable() { PENDRIVE_USB_IMAGE } else { HARD_DRIVE_IMAGE },
                class: "w-32 h-32 m-4"
            }
        }
        div {
            class: "card-body w-4/5",
            h2 {
                class: "card-title",
                { disk.model() }
            }
            div {
                class: "truncate",
                strong {
                    "Nº de serie: "
                }
                { disk.serial() }
            }
            div {
                strong {
                    "Tamaño: "
                }
                { format_size(disk.size()) }
            }
            div {
                strong {
                    "Tipo: "
                }
                { match disk.kind() {
                    DiskKind::HDD => "HDD",
                    DiskKind::SSD => "SSD",
                    DiskKind::SCM => "SCM",
                    DiskKind::Unknown => "Desconocido",
                }
                }
            }
        }
    }
}

fn save_disks(
    mut report: Signal<Report>,
    disks: Vec<Disk>,
    event: FormEvent,
) -> Result<(), String> {
    let selected = event
        .values()
        .iter()
        .filter_map(|(key, value)| {
            if value == "on" {
                Some(key.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    if selected.is_empty() {
        return Err(String::from("No se ha seleccionado ningún disco"));
    }

    let selected_disks = disks
        .into_iter()
        .filter(|disk| selected.contains(&disk.device_name().to_string()))
        .collect::<Vec<Disk>>();

    report.write().selected_disks = selected_disks;

    Ok(())
}
