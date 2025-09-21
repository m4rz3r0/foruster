// SPDX-License-Identifier: GPL-3.0-or-later

    let search_results = analysis_api
        .borrow()
        .deref()
        .search_files_in_profile("Todos", search_term);

    let filtered_files: Vec<File> = search_results
        .iter()
        .map(|path| create_file_from_path(path))
        .collect();

    file_model.set_vec(filtered_files);
}

pub fn initialize(
    window: &Weak<MainWindow>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    profile_api: &Rc<RefCell<ProfileAPI>>
) {
    // Prevenir múltiples análisis simultáneos
    if ANALYSIS_RUNNING.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
        return;
    }

    let window = window.upgrade().unwrap();
    let bridge = window.global::<AnalysisResultBridge>();

    let selected_labels: Vec<_> = get_selected_profiles(window.as_weak())
        .iter()
        .map(|p| p.label.clone())
        .collect();

    bridge.set_used_profiles(ModelRc::new(VecModel::from(
        std::iter::once(SharedString::from("Todos"))
            .chain(selected_labels.clone())
            .collect::<Vec<_>>()
    )));

    let paths = window
        .global::<PathManagementBridge>()
        .get_paths()
        .iter()
        .map(|path| (&path.path).into())
        .collect();

    let profiles = selected_labels
        .iter()
        .flat_map(|l| profile_api.borrow().get_by_label(l.to_string()))
        .collect();

    if let Err(e) = analysis_api.borrow_mut().deref().initialize(profiles, paths) {
        eprintln!("Error inicializando análisis: {}", e);
        ANALYSIS_RUNNING.store(false, Ordering::Release);
        return;
    }

    if let Err(e) = analysis_api.borrow_mut().deref().start_analysis() {
        eprintln!("Error iniciando análisis: {}", e);
        ANALYSIS_RUNNING.store(false, Ordering::Release);
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

    // Canal para recibir la lista completa de archivos construida en background
    let (files_sender, files_receiver): (Sender<Vec<File>>, Receiver<Vec<File>>) = mpsc::channel();

    // Temporizador que aplica actualizaciones cuando llegan (evita bloqueo de UI)
    {
        let file_results_model_clone = file_results_model.clone();
        let timer = Timer::default();
        timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
            // Limitar el número de archivos procesados por iteración para evitar congelamiento
            let mut processed_count = 0;
            const MAX_FILES_PER_BATCH: usize = 50;

            while let Ok(batch) = files_receiver.try_recv() {
                for f in batch.into_iter().take(MAX_FILES_PER_BATCH - processed_count) {
                    file_results_model_clone.push(f);
                    processed_count += 1;

                    if processed_count >= MAX_FILES_PER_BATCH {
                        break;
                    }
                }

                if processed_count >= MAX_FILES_PER_BATCH {
                    break;
                }
            }
        });

        // Guardar el timer para evitar que se destruya
        Box::leak(Box::new(timer));
    }

    let window_weak = window.as_weak();
    let analysis_api_clone = analysis_api.clone();
    let profile_api_clone = profile_api.clone();
    bridge.on_initialize(move || initialize(&window_weak, &analysis_api_clone, &profile_api_clone));

    // Implementar exportar reporte con manejo seguro de errores
    let analysis_api_clone = analysis_api.clone();
    bridge.on_export_report(move || {
        match analysis_api_clone.try_borrow() {
            Ok(api) => {
                if let Ok(progress) = api.deref().get_progress() {
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
            },
            Err(e) => {
                eprintln!("Error accediendo a la API para exportar reporte: {}", e);
            }
        }
    });

    // Implementar filtrado por perfil con manejo seguro
    let file_results_model_clone = file_results_model.clone();
    let analysis_api_clone = analysis_api.clone();
    bridge.on_filter_by_profile(move |profile_name| {
        filter_files_by_profile(
            &file_results_model_clone,
            &analysis_api_clone,
            &profile_name,
        );
    });

    // Implementar búsqueda de archivos con manejo seguro
    let file_results_model_clone = file_results_model.clone();
    let analysis_api_clone = analysis_api.clone();
    bridge.on_search_files(move |search_term| {
        search_files(&file_results_model_clone, &analysis_api_clone, &search_term);
    });

    // Implementar guardar análisis con manejo seguro
    let analysis_api_clone = analysis_api.clone();
    bridge.on_save_analysis(move || {
        match analysis_api_clone.try_borrow() {
            Ok(api) => {
                if let Ok(progress) = api.deref().get_progress() {
                    save_analysis_state(&progress);
                }
            },
            Err(e) => {
                eprintln!("Error accediendo a la API para guardar análisis: {}", e);
            }
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
    });

    bridge.on_load_previous_page(move || {
        println!("Página anterior - funcionalidad pendiente");
    });

    bridge.on_go_to_page(move |page| {
        println!("Ir a página {} - funcionalidad pendiente", page);
    });

    // Callback de progreso optimizado con throttling mejorado y manejo seguro
    let window_weak = window.as_weak();
    let analysis_api_clone = analysis_api.clone();
    let file_results_model_clone = file_results_model.clone();
    let files_sender_clone = files_sender.clone();
    let last_update_time = Arc::new(std::sync::Mutex::new(Instant::now()));
    let files_built = Arc::new(AtomicBool::new(false));

    bridge.on_update_progress(move || {
        if let Some(window) = window_weak.upgrade() {
            // Usar try_borrow para evitar panics durante actualizaciones concurrentes
            let progress = match analysis_api_clone.try_borrow() {
                Ok(api) => {
                    match api.deref().get_progress() {
                        Ok(progress) => progress,
                        Err(e) => {
                            eprintln!("Error obteniendo progreso: {}", e);
                            return;
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error accediendo a la API durante actualización de progreso: {}", e);
                    return;
                }
            };

            let bridge = window.global::<AnalysisResultBridge>();

            // Throttling mejorado basado en tiempo
            let should_update = {
                let mut last_time = last_update_time.lock().unwrap();
                let now = Instant::now();
                if now.duration_since(*last_time) >= Duration::from_millis(200) {
                    *last_time = now;
                    true
                } else {
                    false
                }
            };

            if !should_update && !matches!(progress.state, AnalysisState::Done) {
                return;
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

            // Cuando finaliza el análisis, construir la lista completa en background (solo una vez)
            if matches!(progress.state, AnalysisState::Done) {
                // Marcar que el análisis ha terminado
                ANALYSIS_RUNNING.store(false, Ordering::Release);

                // Construir archivos solo si no se han construido ya
                if files_built.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok()
                    && file_results_model_clone.row_count() == 0 {

                    let sender = files_sender_clone.clone();
                    let api_clone = analysis_api_clone.clone();

                    thread::spawn(move || {
                        // Obtener los paths de forma segura
                        let paths = match api_clone.try_borrow() {
                            Ok(api) => api.get_analyzed_files(),
                            Err(e) => {
                                eprintln!("Error accediendo a archivos analizados: {}", e);
                                return;
                            }
                        };

                        let files: Vec<File> = paths
                            .into_iter()
                            .take(1000) // Limitar el número de archivos para evitar saturación
                            .map(|p| create_file_from_path(&p))
                            .collect();
                        let _ = sender.send(files);
                    });
                }
            }

            // Debug logging reducido
            if progress.analyzed_files % 5000 == 0 || matches!(progress.state, AnalysisState::Done) {
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
    });

    // Inicializar con archivos existentes al finalizar setup
    let file_model_init = file_results_model.clone();
    let analysis_api_init = analysis_api.clone();

    // Cargar archivos iniciales si ya hay análisis disponible
    if let Ok(api) = analysis_api_init.try_borrow() {
        if let Ok(progress) = api.deref().get_progress() {
            if matches!(progress.state, AnalysisState::Done) {
                filter_files_by_profile(&file_model_init, &analysis_api_init, "Todos");
                let stats = api.deref().get_profile_statistics();
                println!("Estadísticas de perfiles:");
                for (profile, count) in stats {
                    println!("  {}: {} archivos", profile, count);
                }
            }
        }
    }
}
