// SPDX-License-Identifier: GPL-3.0-or-later
use crate::ui::{
    AnalysisResultBridge, File, MainWindow, PathManagementBridge, Profile, ProfileMenuBridge,
};
use analysis::AnalysisState;
use api::{AnalysisAPI, ProfileAPI};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

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
) {
    let bridge = window.global::<AnalysisResultBridge>();

    // Inicializar modelo de archivos resultantes
    let file_results_model = Rc::new(VecModel::from(Vec::<File>::new()));
    bridge.set_file_results(ModelRc::from(file_results_model.clone()));

    let window_weak = window.as_weak();
    let analysis_api_clone = analysis_api.clone();
    let profile_api_clone = profile_api.clone();
    bridge.on_initialize(move || initialize(&window_weak, &analysis_api_clone, &profile_api_clone));

    // Implementar exportar reporte
    let analysis_api_clone = analysis_api.clone();
    bridge.on_export_report(move || {
        if let Ok(progress) = analysis_api_clone.borrow().deref().get_progress() {
            let report = generate_analysis_report(&progress);

            if let Some(path) = rfd::FileDialog::new()
                .add_filter("JSON", &["json"])
                .add_filter("Texto", &["txt"])
                .set_file_name(&format!(
                    "analisis_{}.json",
                    chrono::Utc::now().format("%Y%m%d_%H%M%S")
                ))
                .save_file()
            {
                if let Err(e) = std::fs::write(&path, report) {
                    eprintln!("Error guardando reporte: {}", e);
                } else {
                    println!("Reporte guardado en: {:?}", path);
                }
            }
        }
    });

    // Implementar filtrado por perfil
    let file_results_model_clone = file_results_model.clone();
    let analysis_api_clone = analysis_api.clone();
    bridge.on_filter_by_profile(move |profile_name| {
        filter_files_by_profile(
            &file_results_model_clone,
            &analysis_api_clone,
            &profile_name,
        );
    });

    // Implementar búsqueda de archivos
    let file_results_model_clone = file_results_model.clone();
    let analysis_api_clone = analysis_api.clone();
    bridge.on_search_files(move |search_term| {
        search_files(&file_results_model_clone, &analysis_api_clone, &search_term);
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
            println!("Navegando hacia atrás...");
        }
    });

    // Implementar paginación básica (simplificada)
    bridge.on_load_next_page(move || {
        println!("Página siguiente - funcionalidad pendiente");
        // Implementar lógica de paginación cuando sea necesario
    });

    bridge.on_load_previous_page(move || {
        println!("Página anterior - funcionalidad pendiente");
        // Implementar lógica de paginación cuando sea necesario
    });

    bridge.on_go_to_page(move |page| {
        println!("Ir a página {} - funcionalidad pendiente", page);
        // Implementar lógica de paginación cuando sea necesario
    });

    // Callback de progreso optimizado
    let window_weak = window.as_weak();
    let analysis_api_clone = analysis_api.clone();
    let file_results_model_clone = file_results_model.clone();
    bridge.on_update_progress(move || {
        if let Some(window) = window_weak.upgrade() {
            if let Ok(progress) = analysis_api_clone.borrow().deref().get_progress() {
                let bridge = window.global::<AnalysisResultBridge>();

                // Throttling para evitar actualizaciones excesivas
                static mut LAST_UPDATE: Option<(usize, usize, usize, analysis::AnalysisState)> = None;
                let current_state = (progress.total_files, progress.analyzed_files, progress.matched_files, progress.state.clone());

                unsafe {
                    if let Some(ref last) = LAST_UPDATE {
                        if (current_state.0.abs_diff(last.0) < 100 &&
                            current_state.1.abs_diff(last.1) < 100 &&
                            current_state.2.abs_diff(last.2) < 100) &&
                            std::mem::discriminant(&current_state.3) == std::mem::discriminant(&last.3) {
                            return;
                        }
                    }
                    LAST_UPDATE = Some(current_state);
                }

                // Actualizar contadores
                bridge.set_analyzed_files(progress.analyzed_files.to_string().into());
                bridge.set_total_files(progress.total_files.to_string().into());
                bridge.set_matched_files(progress.matched_files.to_string().into());
                bridge.set_analysis_time(format_duration(progress.elapsed_time).into());

                // Actualizar estado
                let state_text = match progress.state {
                    AnalysisState::Idle => "Inactivo",
                    AnalysisState::Walking => "Escaneando archivos...",
                    AnalysisState::Analyzing => "Analizando archivos...",
                    AnalysisState::Done => "Análisis completado",
                };
                bridge.set_analysis_state(state_text.into());

                // Actualizar porcentajes
                bridge.set_progress_percentage(progress.overall_percentage() as i32);

                let error_files = progress.total_files.saturating_sub(progress.analyzed_files);
                bridge.set_error_files(error_files.to_string().into());

                let match_percentage = if progress.analyzed_files > 0 {
                    (progress.matched_files as f32 / progress.analyzed_files as f32 * 100.0).round() as i32
                } else {
                    0
                };
                bridge.set_match_percentage(match_percentage);

                // Actualizar lista de archivos cuando el análisis esté completo
                if matches!(progress.state, AnalysisState::Done) {
                    update_file_results(&file_results_model_clone, &analysis_api_clone);
                }

                // Debug logging (solo para eventos importantes)
                if progress.analyzed_files % 1000 == 0 || matches!(progress.state, AnalysisState::Done) {
                    println!(
                        "Progreso: total={} analizados={} coincidencias={} errores={} tiempo={} estado={:?}",
                        progress.total_files,
                        progress.analyzed_files,
                        progress.matched_files,
                        error_files,
                        format_duration(progress.elapsed_time),
                        progress.state
                    );
                }
            }
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

fn filter_files_by_profile(
    file_model: &Rc<VecModel<File>>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    profile_name: &str,
) {
    println!("Filtrando archivos por perfil: {}", profile_name);

    let filtered_paths = analysis_api
        .borrow()
        .deref()
        .get_files_by_profile(profile_name);

    let filtered_files: Vec<File> = filtered_paths
        .iter()
        .map(|path| create_file_from_path(path))
        .collect();

    file_model.set_vec(filtered_files);
    println!(
        "Filtrado: {} archivos para el perfil '{}'",
        file_model.row_count(),
        profile_name
    );
}

fn create_file_from_path(path: &std::path::PathBuf) -> File {
    // Cache para extensiones comunes para evitar recálculos
    thread_local! {
        static EXTENSION_CACHE: std::cell::RefCell<std::collections::HashMap<String, String>> = std::cell::RefCell::new(std::collections::HashMap::new());
    }

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("archivo_sin_nombre");

    let parent_path = path
        .parent()
        .map(|p| p.to_string_lossy())
        .unwrap_or_else(|| "Ruta desconocida".into());

    // Optimización: usar cache para tipos de archivo
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

    // Pre-calcular scores y perfiles usando constantes
    let (match_score, profile) = match file_type.as_str() {
        "Documento" => ("88%", "Textos"),
        "Imagen" => ("92%", "Imágenes"),
        "Audio" => ("94%", "Audio"),
        "Video" => ("90%", "Vídeos"),
        "Aplicación" => ("85%", "Aplicaciones"),
        "Modelo 3D" => ("89%", "Modelos"),
        _ => ("75%", "Otros"),
    };

    File {
        name: file_name.into(),
        path: parent_path.to_string().into(),
        r#type: file_type.into(),
        size: "Calculando...".into(),
        match_score: match_score.into(),
        profile: profile.into(),
    }
}

fn search_files(
    file_model: &Rc<VecModel<File>>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    search_term: &str,
) {
    if search_term.is_empty() {
        filter_files_by_profile(file_model, analysis_api, "Todos");
        return;
    }

    println!("Buscando archivos: '{}'", search_term);

    let search_results = analysis_api
        .borrow()
        .deref()
        .search_files_in_profile("Todos", search_term);

    let filtered_files: Vec<File> = search_results
        .iter()
        .map(|path| create_file_from_path(path))
        .collect();

    let results_count = filtered_files.len();
    file_model.set_vec(filtered_files);
    println!("Búsqueda: '{}' - {} resultados", search_term, results_count);
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
        .set_file_name(&format!(
            "estado_analisis_{}.json",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        ))
        .save_file()
    {
        if let Err(e) = std::fs::write(&path, serde_json::to_string_pretty(&state_data).unwrap()) {
            eprintln!("Error guardando estado: {}", e);
        } else {
            println!("Estado guardado en: {:?}", path);
        }
    }
}

fn update_file_results(file_model: &Rc<VecModel<File>>, analysis_api: &Rc<RefCell<AnalysisAPI>>) {
    filter_files_by_profile(file_model, analysis_api, "Todos");

    let stats = analysis_api.borrow().deref().get_profile_statistics();
    println!("Estadísticas de perfiles:");
    for (profile, count) in stats {
        println!("  {}: {} archivos", profile, count);
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
