// SPDX-License-Identifier: GPL-3.0-or-later
use crate::ui::{
    AnalysisResultBridge, File, FileDetailsBridge, MainWindow, PathManagementBridge, Profile,
    ProfileMenuBridge,
};
use analysis::AnalysisState;
use api::{AnalysisAPI, ProfileAPI, StorageAPI};
use slint::{ComponentHandle, Model, ModelRc, Rgba8Pixel, SharedString, VecModel, Weak};
use std::cell::RefCell;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

use crate::cache::thumbnail_cache;
use crate::cache::thumbnail_cache::CachedThumbnail;
use app_core::format_size;
use image::ImageReader;
use reporting::pdf_report;
use slint::Image as SlintImage;
use slint::SharedPixelBuffer;
use std::io::Cursor;

fn generate_thumbnail(path: &Path) -> Option<SlintImage> {
    // 1. Check the cache for the raw, thread-safe data first.
    if let Some(cached_data) = thumbnail_cache::get(path) {
        // Found it! Re-create the UI-specific slint::Image from the raw data.
        // This is cheap and safe to do on the UI thread.
        return Some(SlintImage::from_rgba8(cached_data.buffer));
    }

    // 2. If not in cache, generate it from the file.
    const THUMBNAIL_SIZE: u32 = 64;

    let img_bytes = std::fs::read(path).ok()?;
    let img_reader = ImageReader::new(Cursor::new(img_bytes))
        .with_guessed_format()
        .ok()?;

    if let Ok(img) = img_reader.decode() {
        let thumbnail = img.thumbnail_exact(THUMBNAIL_SIZE, THUMBNAIL_SIZE);
        let rgba_image = thumbnail.to_rgba8();
        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            rgba_image.as_raw(),
            rgba_image.width(),
            rgba_image.height(),
        );

        // 3. Create our cacheable, thread-safe struct.
        let thumbnail_to_cache = CachedThumbnail {
            buffer: buffer.clone(), // Clone the buffer for the cache
        };

        // 4. Add the raw data struct to the cache.
        thumbnail_cache::insert(path.to_path_buf(), thumbnail_to_cache);

        // 5. Return a new slint::Image created from the buffer for immediate use.
        Some(SlintImage::from_rgba8(buffer))
    } else {
        None
    }
}

fn get_selected_profiles(window_weak: Weak<MainWindow>) -> Vec<Profile> {
    let window = window_weak.upgrade().unwrap();
    let profile_bridge = window.global::<ProfileMenuBridge>().get_profiles();

    profile_bridge
        .iter()
        .filter(|profile| profile.selected)
        .collect()
}

pub fn initialize(
    window: &Weak<MainWindow>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    profile_api: &Rc<RefCell<ProfileAPI>>,
) {
    let window = window.upgrade().unwrap();
    let bridge = window.global::<AnalysisResultBridge>();

    let selected_profiles_labels: Vec<_> = get_selected_profiles(window.as_weak())
        .iter()
        .map(|profile| profile.label.clone())
        .collect();

    let selected_profiles_labels_str = selected_profiles_labels.join(", ");
    bridge.set_used_profiles(ModelRc::new(VecModel::from(
        std::iter::once(SharedString::from("Todos"))
            .chain(selected_profiles_labels.clone())
            .collect::<Vec<_>>(),
    )));
    bridge.set_used_profiles_str(SharedString::from(selected_profiles_labels_str));

    let paths = window
        .global::<PathManagementBridge>()
        .get_paths()
        .iter()
        .map(|path| (&path.path).into())
        .collect();

    let selected_profiles = selected_profiles_labels
        .iter()
        .flat_map(|label| profile_api.borrow().get_by_label(label.to_string()))
        .collect();

    if let Err(e) = analysis_api
        .borrow_mut()
        .deref()
        .initialize(selected_profiles, paths)
    {
        eprintln!("Error inicializando análisis: {}", e);
        return;
    }

    if let Err(e) = analysis_api.borrow_mut().deref().start_analysis() {
        eprintln!("Error iniciando análisis: {}", e);
    }
}

pub fn setup(
    window: &MainWindow,
    analysis_api: Rc<RefCell<AnalysisAPI>>,
    profile_api: Rc<RefCell<ProfileAPI>>,
    storage_api: Rc<RefCell<StorageAPI>>,
) {
    let bridge = window.global::<AnalysisResultBridge>();

    // Inicializar modelo de archivos resultantes
    let file_results_model = Rc::new(VecModel::from(Vec::<File>::new()));
    bridge.set_file_results(ModelRc::from(file_results_model.clone()));
    let suspicious_files_model = Rc::new(VecModel::from(Vec::<File>::new()));
    bridge.set_suspicious_file_results(ModelRc::from(suspicious_files_model.clone()));

    // Modelo para almacenar TODOS los archivos filtrados (sin paginación)
    let all_filtered_files = Rc::new(RefCell::new(Vec::<File>::new()));

    // Inicializar propiedades de paginación
    bridge.set_current_page(1);
    bridge.set_total_pages(1);
    bridge.set_total_items(0);
    bridge.set_items_per_page(50);

    let window_weak = window.as_weak();
    let analysis_api_clone = analysis_api.clone();
    let profile_api_clone = profile_api.clone();
    bridge.on_initialize(move || initialize(&window_weak, &analysis_api_clone, &profile_api_clone));

    // Implementar exportar reporte
    let analysis_api_clone = analysis_api.clone();
    let storage_api_clone = storage_api.clone(); // <-- Clone storage_api
    bridge.on_export_report(move || {
        if let Err(e) =
            pdf_report::generate_pdf_report(analysis_api_clone.clone(), storage_api_clone.clone())
        {
            eprintln!("Error generating PDF report: {}", e);
        }
    });

    // Implementar filtrado por perfil
    let file_results_model_clone = file_results_model.clone();
    let analysis_api_clone = analysis_api.clone();
    let window_weak_filter = window.as_weak();
    let all_filtered_files_clone = all_filtered_files.clone();
    bridge.on_filter_by_profile(move |profile_name| {
        filter_files_by_profile(
            &file_results_model_clone,
            &analysis_api_clone,
            profile_name.as_ref(),
            &window_weak_filter,
            &all_filtered_files_clone,
        );
    });

    // Implementar búsqueda de archivos
    let file_results_model_clone = file_results_model.clone();
    let analysis_api_clone = analysis_api.clone();
    let window_weak_search = window.as_weak();
    let all_filtered_files_clone = all_filtered_files.clone();
    bridge.on_search_files(move |search_term| {
        search_files(
            &file_results_model_clone,
            &analysis_api_clone,
            search_term.as_ref(),
            &window_weak_search,
            &all_filtered_files_clone,
        );
    });

    // Implementar guardar análisis
    let analysis_api_clone = analysis_api.clone();
    bridge.on_save_analysis(move || {
        if let Ok(progress) = analysis_api_clone.borrow().deref().get_progress() {
            save_analysis_state(&progress);
        }
    });

    // Implementar navegación hacia atrás
    let window_weak = window.as_weak();
    bridge.on_go_back(move || {
        if let Some(_window) = window_weak.upgrade() {
            // TODO: Implement back navigation logic
        }
    });

    // Implementar paginación
    let window_weak_next = window.as_weak();
    let file_results_model_next = file_results_model.clone();
    let all_filtered_files_next = all_filtered_files.clone();
    bridge.on_load_next_page(move || {
        if let Some(window) = window_weak_next.upgrade() {
            let bridge = window.global::<AnalysisResultBridge>();
            let current_page = bridge.get_current_page();
            let total_pages = bridge.get_total_pages();

            if current_page < total_pages {
                bridge.set_current_page(current_page + 1);
                update_page_results(
                    &window.as_weak(),
                    &file_results_model_next,
                    &all_filtered_files_next,
                );
            }
        }
    });

    let window_weak_prev = window.as_weak();
    let file_results_model_prev = file_results_model.clone();
    let all_filtered_files_prev = all_filtered_files.clone();
    bridge.on_load_previous_page(move || {
        if let Some(window) = window_weak_prev.upgrade() {
            let bridge = window.global::<AnalysisResultBridge>();
            let current_page = bridge.get_current_page();

            if current_page > 1 {
                bridge.set_current_page(current_page - 1);
                update_page_results(
                    &window.as_weak(),
                    &file_results_model_prev,
                    &all_filtered_files_prev,
                );
            }
        }
    });

    let window_weak_goto = window.as_weak();
    let file_results_model_goto = file_results_model.clone();
    let all_filtered_files_goto = all_filtered_files.clone();
    bridge.on_go_to_page(move |page| {
        if let Some(window) = window_weak_goto.upgrade() {
            let bridge = window.global::<AnalysisResultBridge>();
            let total_pages = bridge.get_total_pages();

            if page >= 1 && page <= total_pages {
                bridge.set_current_page(page);
                update_page_results(
                    &window.as_weak(),
                    &file_results_model_goto,
                    &all_filtered_files_goto,
                );
            }
        }
    });

    // Callback de progreso optimizado
    let window_weak = window.as_weak();
    let analysis_api_clone = analysis_api.clone();
    let file_results_model_clone = file_results_model.clone();
    let suspicious_files_model_clone = suspicious_files_model.clone();
    bridge.on_update_progress(move || {
        if let Some(window) = window_weak.upgrade()
            && let Ok(progress) = analysis_api_clone.borrow().deref().get_progress()
        {
            let bridge = window.global::<AnalysisResultBridge>();

            // Actualizar contadores
            bridge.set_analyzed_files(progress.analyzed_files.to_string().into());
            bridge.set_total_files(progress.total_files.to_string().into());
            bridge.set_matched_files(progress.matched_files.to_string().into());
            bridge.set_suspicious_files(progress.suspicious_files.to_string().into());
            bridge.set_analysis_time(format_duration(progress.elapsed_time).into());

            // Actualizar estado
            let state_text = match progress.state {
                AnalysisState::Idle => "Inactivo",
                AnalysisState::Walking => "Escaneando archivos...",
                AnalysisState::Analyzing => "Analizando archivos...",
                AnalysisState::Done => "Análisis completado",
            };
            bridge.set_analysis_state(state_text.into());

            bridge.set_progress_percentage(progress.overall_percentage() as i32);

            let error_files = progress.total_files.saturating_sub(progress.analyzed_files);
            bridge.set_error_files(error_files.to_string().into());

            if matches!(progress.state, AnalysisState::Done) {
                update_file_results(
                    &file_results_model_clone,
                    &analysis_api_clone,
                    &window.as_weak(),
                );
                update_suspicious_files_results(
                    &suspicious_files_model_clone,
                    &analysis_api_clone,
                    &window.as_weak(),
                );
            }
        }
    });

    let details_bridge = window.global::<FileDetailsBridge>();

    let window_weak_details = window.as_weak();
    window
        .global::<AnalysisResultBridge>()
        .on_show_file_details(move |file| {
            if let Some(window) = window_weak_details.upgrade() {
                let bridge = window.global::<FileDetailsBridge>();
                bridge.set_file_name(file.name.clone());
                // Combine path and name for the full path
                let full_path = std::path::Path::new(file.path.as_str()).join(file.name.as_str());
                bridge.set_full_path(full_path.to_string_lossy().into_owned().into());
                bridge.set_file_type(file.r#type.clone());
                bridge.set_file_size(file.size.clone());
                bridge.set_suspicion_details(file.suspicion_details.clone());
                bridge.set_visible(true);
            }
        });

    details_bridge.on_open_containing_folder(move |path_str| {
        let path = Path::new(path_str.as_str());
        if let Some(parent) = path.parent() {
            if let Err(e) = opener::open(parent) {
                eprintln!("Failed to open folder for {}: {}", path_str, e);
            }
        } else {
            eprintln!("Could not get parent folder for: {}", path_str);
        }
    });
}

// Funciones auxiliares necesarias

fn generate_analysis_report(progress: &api::AnalysisProgress) -> String {
    let report = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "analysis_summary": {
            "total_files": progress.total_files,
            "analyzed_files": progress.analyzed_files,
            "matched_files": progress.matched_files,
            "error_files": progress.total_files.saturating_sub(progress.analyzed_files),
            "elapsed_time_seconds": progress.elapsed_time.as_secs(),
            "match_percentage": if progress.analyzed_files > 0 {
                (progress.matched_files as f32 / progress.analyzed_files as f32 * 100.0).round()
            } else {
                0.0
            }
        },
        "state": format!("{:?}", progress.state)
    });

    serde_json::to_string_pretty(&report).unwrap_or_else(|_| {
        format!(
            "Reporte de Análisis\n\
            ==================\n\
            Fecha: {}\n\
            Archivos totales: {}\n\
            Archivos analizados: {}\n\
            Archivos coincidentes: {}\n\
            Archivos con error: {}\n\
            Tiempo transcurrido: {}\n\
            Porcentaje de coincidencias: {:.1}%",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            progress.total_files,
            progress.analyzed_files,
            progress.matched_files,
            progress.total_files.saturating_sub(progress.analyzed_files),
            format_duration(progress.elapsed_time),
            if progress.analyzed_files > 0 {
                progress.matched_files as f32 / progress.analyzed_files as f32 * 100.0
            } else {
                0.0
            }
        )
    })
}

fn update_suspicious_files_results(
    file_model: &Rc<VecModel<File>>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    _window_weak: &Weak<MainWindow>,
) {
    let suspicious_items = analysis_api.borrow().get_suspicious_files();

    let suspicious_files: Vec<File> = suspicious_items
        .iter()
        .map(|(p, reason)| {
            let mut file_ui = create_file_from_path(p);
            file_ui.suspicion_details = reason.to_string().into(); // Set the reason here
            file_ui
        })
        .collect();

    file_model.set_vec(suspicious_files);
}

fn filter_files_by_profile(
    file_model: &Rc<VecModel<File>>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    profile_name: &str,
    window_weak: &Weak<MainWindow>,
    all_filtered_files: &Rc<RefCell<Vec<File>>>,
) {
    let filtered_paths = if profile_name == "Todos" {
        analysis_api.borrow().get_all_matched_files()
    } else {
        analysis_api
            .borrow()
            .deref()
            .get_files_by_profile(profile_name)
    };

    let filtered_files: Vec<File> = filtered_paths
        .iter()
        .map(|p| create_file_from_path(p))
        .collect();

    all_filtered_files.borrow_mut().clear();
    all_filtered_files.borrow_mut().extend(filtered_files);

    // Actualizar paginación basada en archivos filtrados
    if let Some(window) = window_weak.upgrade() {
        let bridge = window.global::<AnalysisResultBridge>();
        let total_filtered = all_filtered_files.borrow().len() as i32;
        bridge.set_total_items(total_filtered);
        bridge.set_current_page(1);
        update_pagination(&window.as_weak());

        update_page_results(&window.as_weak(), file_model, all_filtered_files);
    }
}

fn create_file_from_path(path: &Path) -> File {
    thread_local! {
        static EXTENSION_CACHE: RefCell<std::collections::HashMap<String, String>> = RefCell::new(std::collections::HashMap::new());
    }

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("archivo_sin_nombre");

    let parent_path = path
        .parent()
        .map(|p| p.to_string_lossy())
        .unwrap_or_else(|| "Ruta desconocida".into());

    let file_type = if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
        let ext_lower = ext.to_lowercase();

        EXTENSION_CACHE.with(|cache| {
            cache
                .borrow_mut()
                .entry(ext_lower.clone())
                .or_insert_with(|| {
                    match ext_lower.as_str() {
                        "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" => "Documento",
                        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" => "Imagen",
                        "mp3" | "wav" | "flac" | "ogg" | "aac" | "m4a" => "Audio",
                        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" => "Video",
                        "exe" | "msi" | "deb" | "rpm" | "dmg" | "app" => "Aplicación",
                        "blend" | "obj" | "fbx" | "dae" | "3ds" | "max" => "Modelo 3D",
                        _ => "Archivo",
                    }
                    .to_string()
                })
                .clone()
        })
    } else {
        "Archivo".to_string()
    };

    let thumbnail = if file_type == "Imagen" {
        generate_thumbnail(path).unwrap_or_default()
    } else {
        SlintImage::default()
    };

    let size_str = match fs::metadata(path) {
        Ok(metadata) => format_size(metadata.len() as usize),
        Err(_) => "Error".into(),
    };

    File {
        name: file_name.into(),
        path: parent_path.to_string().into(),
        r#type: file_type.into(),
        size: size_str.into(),
        match_score: "".into(),
        profile: "General".into(),
        thumbnail, // Pass the generated thumbnail
        suspicion_details: "".into(),
    }
}

fn search_files(
    file_model: &Rc<VecModel<File>>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    search_term: &str,
    window_weak: &Weak<MainWindow>,
    all_filtered_files: &Rc<RefCell<Vec<File>>>,
) {
    if search_term.is_empty() {
        filter_files_by_profile(
            file_model,
            analysis_api,
            "Todos",
            window_weak,
            all_filtered_files,
        );
        return;
    }

    let search_results = analysis_api
        .borrow()
        .deref()
        .search_files_in_profile("Todos", search_term);

    let filtered_files: Vec<File> = search_results
        .iter()
        .map(|p| create_file_from_path(p))
        .collect();

    // Guardar TODOS los archivos de búsqueda (sin paginación)
    all_filtered_files.borrow_mut().clear();
    all_filtered_files.borrow_mut().extend(filtered_files);

    // Actualizar paginación basada en resultados de búsqueda
    if let Some(window) = window_weak.upgrade() {
        let bridge = window.global::<AnalysisResultBridge>();
        let total_search_results = all_filtered_files.borrow().len() as i32;
        bridge.set_total_items(total_search_results);
        bridge.set_current_page(1);
        update_pagination(&window.as_weak());

        // Mostrar solo la primera página de los resultados de búsqueda
        update_page_results(&window.as_weak(), file_model, all_filtered_files);
    }
}

fn save_analysis_state(progress: &api::AnalysisProgress) {
    let state_data = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "progress": {
            "total_files": progress.total_files,
            "analyzed_files": progress.analyzed_files,
            "matched_files": progress.matched_files,
            "elapsed_time": progress.elapsed_time.as_secs(),
            "state": format!("{:?}", progress.state)
        }
    });

    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Estado de Análisis", &["json"])
        .set_file_name(format!(
            "estado_analisis_{}.json",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        ))
        .save_file()
        && let Err(e) = std::fs::write(&path, serde_json::to_string_pretty(&state_data).unwrap())
    {
        eprintln!("Error guardando estado: {}", e);
    }
}

fn update_file_results(
    file_model: &Rc<VecModel<File>>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    window_weak: &Weak<MainWindow>,
) {
    filter_files_by_profile(
        file_model,
        analysis_api,
        "Todos",
        window_weak,
        &Rc::new(RefCell::new(Vec::<File>::new())),
    );
}

fn update_pagination(window_weak: &Weak<MainWindow>) {
    if let Some(window) = window_weak.upgrade() {
        let bridge = window.global::<AnalysisResultBridge>();
        let total_items = bridge.get_total_items();
        let items_per_page = bridge.get_items_per_page();

        let total_pages = if total_items > 0 {
            ((total_items as f32) / (items_per_page as f32)).ceil() as i32
        } else {
            1
        };

        bridge.set_total_pages(total_pages);

        let current_page = bridge.get_current_page();
        if current_page > total_pages {
            bridge.set_current_page(total_pages);
        }
        if current_page < 1 {
            bridge.set_current_page(1);
        }
    }
}

fn update_page_results(
    window_weak: &Weak<MainWindow>,
    file_results_model: &Rc<VecModel<File>>,
    all_filtered_files: &Rc<RefCell<Vec<File>>>,
) {
    if let Some(window) = window_weak.upgrade() {
        let bridge = window.global::<AnalysisResultBridge>();
        let current_page = bridge.get_current_page();
        let items_per_page = bridge.get_items_per_page();

        // Calcular el rango de elementos para la página actual
        let all_files = all_filtered_files.borrow();
        let total_files = all_files.len();

        let start_index = ((current_page - 1) * items_per_page) as usize;
        let end_index = (start_index + items_per_page as usize).min(total_files);

        // Obtener solo los archivos de la página actual
        let page_files: Vec<File> = if start_index < total_files {
            all_files[start_index..end_index].to_vec()
        } else {
            Vec::new()
        };

        // Actualizar el modelo con solo los archivos de la página actual
        file_results_model.set_vec(page_files);

        // Actualizar la paginación
        update_pagination(&window.as_weak());
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
