// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;
use native_dialog::FileDialog;

use std::path::PathBuf;

use crate::{
    bytes_to_mb, create_thumbnail_base64, get_suspicious_files, show_error, show_success, AppState, ModalInfo, Report, SUSPICIOUS_FILES_ID, THUMBNAIL_SIZE
};

#[component]
pub fn DetailedResults(id: usize) -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let report = use_context::<Signal<Report>>();

    rsx! {
        div {
            class: "w-full h-full",

            div { class: "w-full h-1/6",
                button {
                    class: "btn btn-primary m-2 p-4",

                    onclick: move |_| {
                        app_state.write().selected_file = None;
                        navigator().go_back();
                    },
                    "Volver atrás"
                }
            }
            div {
                class: "flex flex-row border-t h-5/6",
                div { class: "w-3/4",
                    TableResults { report, app_state, id }
                }
                div { class: "w-1/4 border-l p-4 break-words",
                    DetailPanel { app_state }
                }
            }
        }
    }
}

#[component]
pub fn TableResults(report: Signal<Report>, app_state: Signal<AppState>, id: usize) -> Element {
    let files_per_page = 20;

    let files: Vec<_> = if id == SUSPICIOUS_FILES_ID {
        get_suspicious_files(report)
    } else {
        let selected_profile = report
            .read()
            .selected_profiles
            .iter()
            .find(|p| p.id() == id)
            .unwrap()
            .clone();
        selected_profile.files().clone()
    };

    let selected_file = app_state.read().selected_file.clone();

    // Pagination
    let mut current_page = use_signal(|| 0);
    let total_pages = (files.len() + files_per_page - 1) / files_per_page;
    let start_index = current_page() * files_per_page;
    let end_index = start_index + files_per_page;
    let paginated_files = files[start_index..end_index.min(files.len())].to_vec();

    rsx! {
        table { class: "w-full text-center",
            thead {
                tr {
                    th { "Nombre" }
                    th { "Tamaño" }
                    th { "Fecha modificación" }
                    th { "Tipo" }
                }
            }
            tbody {
                {paginated_files.iter().cloned().map(|file| {
                    let is_selected = selected_file.as_ref().map_or(false, |f| f.path() == file.path());
                    let row_classes = if is_selected {
                        "cursor-pointer bg-blue-400 hover:bg-blue-500 text-white"
                    } else {
                        "cursor-pointer hover:bg-gray-200"
                    };

                    // File data
                    let name = file.name().to_string_lossy();
                    let size = format!("{:.2} MB", bytes_to_mb(file.size()));
                    let modified = file.modified().format("%d/%m/%Y").to_string();
                    let mime_type = match infer::get(file.magic_bytes()) {
                        Some(kind) => kind.mime_type(),
                        None => "Desconocido",
                    };

                    rsx! {
                        tr {
                            class: "{row_classes}",
                            onclick: move |_| {
                                app_state.write().thumbnail_generated = false;
                                app_state.write().selected_file = Some(file.clone());
                            },
                            td { { name } }
                            td { { size } }
                            td { { modified } }
                            td { { mime_type } }
                        }
                    }
                })}
            }
        }
        div { class: "flex justify-between mt-4",
            button {
                class: "btn btn-accent m-2",
                disabled: current_page() == 0,
                onclick: move |_| current_page.set(current_page() - 1),
                "Anterior"
            }
            p { class: "text-center", "Página {current_page + 1} de {total_pages}" }
            button {
                class: "btn btn-accent m-2",
                disabled: current_page() == total_pages - 1,
                onclick: move |_| current_page.set(current_page() + 1),
                "Siguiente"
            }
        }
    }
}

#[component]
pub fn DetailPanel(app_state: Signal<AppState>) -> Element {
    let modal_info = use_context::<Signal<ModalInfo>>();
    let selected_file = app_state.read().selected_file.clone();

    match selected_file {
        Some(file) => {
            let name = file.name().to_string_lossy().to_string();
            let path = file.into_path();
            let folder_path = match file.path().parent() {
                Some(p) => p.to_path_buf(),
                None => PathBuf::new(),
            };
            let size = format!("{:.2} MB", bytes_to_mb(file.size()));
            let modified = file.modified().format("%d/%m/%Y").to_string();
            let extension = match file.extension() {
                Some(ext) => &ext.to_string_lossy(),
                None => "VACÍA",
            };
            let kind = infer::get(file.magic_bytes());

            let (true_extension, mime_type) = match kind {
                Some(kind) => (kind.extension(), kind.mime_type()),
                None => ("Desconocido", "Desconocido"),
            };
            let hash = match file.hash() {
                Some(hash) => hash,
                None => "Calculando...",
            };
            let suspicious = rsx! {
                if file.suspicious() {
                    p {
                        class: "text-red-500",

                        "Es un archivo sospechoso"
                    }
                    p {
                        "El archivo es de tipo " strong {{mime_type}} " y tiene la extensión " strong {{extension}} " pero debería tener la extensión " strong {{true_extension}}
                    }
                } else {
                    p {
                        class: "text-green-500",

                        "No es sospechoso"
                    }
                }
            };

            rsx! {
                h2 { class: "text-xl font-bold", "Detalles del archivo" }
                ThumbnailViewer { app_state, image_path: path.clone() }
                p { strong {"Nombre: "} { name.to_string() } }
                p { strong {"Ruta: "}   { path.to_string_lossy() } }
                p { strong {"Tamaño: "} { size } }
                p { strong {"Fecha modificación: "} { modified } }
                p { strong {"Hash SHA256: "} { hash } }
                p { strong {"Tipo de archivo: "} { mime_type } }
                p { strong {"Extensión: "} { extension } }
                { suspicious }

                div {
                    class: "flex flex-row card-actions justify-between m-2",
                    button {
                        onclick: move |_| {
                            let destination_path = match FileDialog::new().set_title("Seleccione la ruta donde se guardará el archivo").set_filename(&name).show_save_single_file() {
                                Ok(Some(path)) => path,
                                _ => {
                                    show_error(
                                        use_context::<Signal<ModalInfo>>(),
                                        "No se ha seleccionado una carpeta válida",
                                    );
                                    return;
                                }
                            };

                            match std::fs::copy(&path, &destination_path) {
                                Ok(_) => show_success(modal_info, "El archivo se ha guardado correctamente"),
                                Err(e) => show_error(modal_info, &format!("Ocurrió un error al guardar el archivo: {e}")),
                            }
                        },
                        class: "btn",
                        "Guardar archivo"
                    }
                    button {
                        onclick: move |_| {
                            if let Err(e) = open::that(&folder_path) {
                                show_error(modal_info, &e.to_string())
                            }
                        },
                        class: "btn",
                        "Abrir carpeta contenedora"
                    }
                }
            }
        }
        None => rsx! {
            h2 { class: "text-xl font-bold", "Detalles del archivo" }
            p { "Seleccione un archivo de la tabla para ver los detalles." }
        },
    }
}

#[component]
fn ThumbnailViewer(app_state: Signal<AppState>, image_path: PathBuf) -> Element {
    let mut future_thumbnail = use_resource(move || {
        let path = image_path.clone();
        async move {
            match tokio::task::spawn_blocking(|| {
                create_thumbnail_base64(path, THUMBNAIL_SIZE, THUMBNAIL_SIZE)
            })
            .await
            .unwrap()
            {
                Ok(thumbnail) => {
                    app_state.write().thumbnail_generated = true;
                    thumbnail
                }
                Err(_) => String::new(),
            }
        }
    });

    use_effect(move || {
        if !app_state.read().thumbnail_generated {
            future_thumbnail.clear();
            future_thumbnail.restart();
        }
    });

    let result = future_thumbnail.read().clone();

    match result {
        Some(base64_thumbnail) => rsx! {
            div { class: "w-full text-center p-4",
                if !base64_thumbnail.is_empty() {
                    img { src: "data:image/png;base64,{base64_thumbnail}", alt: "Miniatura", class: "mx-auto rounded", width: "{THUMBNAIL_SIZE}", height: "{THUMBNAIL_SIZE}" }
                }
            }
        },
        None => rsx! {
            div { class: "container mx-auto p-4",
                p { "Cargando miniatura..." }
            }
        },
    }
}
