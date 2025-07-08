// SPDX-License-Identifier: GPL-3.0-or-later
use crate::config::Config;
use crate::finding::{Finding, FindingContainer};
use crate::walker::{WalkBatch, Walker};
use profiling::Profile;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default, Clone, Debug)]
pub enum AnalysisState {
    #[default]
    Idle,
    Walking,
    Analyzing,
    Done,
}

#[derive(Default)]
pub struct Engine {
    state: Arc<Mutex<AnalysisState>>,
    config: Arc<Config>,
    finding: FindingContainer,
    progress_callback: Option<Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>>,
}

impl Engine {
    pub fn initialize(&mut self, profiles: Vec<Profile>, paths: Vec<PathBuf>) {
        *Arc::make_mut(&mut self.config) = Config::new(profiles, paths);
    }

    pub fn state(&self) -> Arc<Mutex<AnalysisState>> {
        self.state.clone()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn finding(&self) -> &FindingContainer {
        &self.finding
    }

    // Método para establecer callback de progreso
    pub fn set_progress_callback(
        &mut self,
        callback: Option<Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>>,
    ) {
        self.progress_callback = callback;
    }

    // Método principal para ejecutar análisis
    pub async fn run_analysis(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(callback) = self.progress_callback.take() {
            let callback_arc = Arc::new(callback);
            let result = self.start_with_progress_callback(callback_arc.clone()).await;
            // Re-establish the callback if it's Arc, otherwise it's lost
            if let Ok(boxed_callback) = Arc::try_unwrap(callback_arc) {
                self.progress_callback = Some(boxed_callback);
            }
            result
        } else {
            self.start().await
        }
    }

    pub async fn start_with_progress_callback(
        &mut self,
        progress_callback: Arc<Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Cambiar estado a Walking
        self.update_state(AnalysisState::Walking)?;
        self.notify_progress(&progress_callback, "state_change", &[("state", "Walking")])?;

        // Contadores atómicos para mejor rendimiento - incluyendo matched_files
        let scanned_count = Arc::new(AtomicUsize::new(0));
        let files_count = Arc::new(AtomicUsize::new(0));
        let matched_count = Arc::new(AtomicUsize::new(0));
        let total_files_estimated = Arc::new(AtomicUsize::new(0));

        // Obtener perfiles una vez para pasarlos a los walkers
        let profiles: Vec<_> = self.config.analysis_profile().clone();

        // Índice compartido para archivos por perfil
        let files_by_profile_shared = Arc::new(Mutex::new(HashMap::<String, Vec<PathBuf>>::new()));

        // Procesar paths con mejor manejo de errores
        let mut handles = Vec::new();

        for (path_index, path) in self.config.paths().iter().enumerate() {
            let walker = Walker::new(path);
            let progress_callback_clone = Arc::clone(&progress_callback);
            let scanned_count_clone = Arc::clone(&scanned_count);
            let files_count_clone = Arc::clone(&files_count);
            let matched_count_clone = Arc::clone(&matched_count);
            let total_files_estimated_clone = Arc::clone(&total_files_estimated);
            let profiles_clone = profiles.clone();
            let files_by_profile_clone = Arc::clone(&files_by_profile_shared);

            let handle = tokio::spawn(async move {
                let mut walker = walker;
                let profiles_for_walker = profiles_clone.clone(); // Clonar antes de mover
                let result = walker
                    .start_with_batch_callback(
                        Box::new({
                            let callback = Arc::clone(&progress_callback_clone);
                            let scanned_count = Arc::clone(&scanned_count_clone);
                            let files_count = Arc::clone(&files_count_clone);
                            let matched_count = Arc::clone(&matched_count_clone);
                            let total_files_estimated = Arc::clone(&total_files_estimated_clone);
                            let files_by_profile = Arc::clone(&files_by_profile_clone);

                            move |batch: WalkBatch| {
                                // Actualizar contadores atómicamente
                                let current_scanned = scanned_count
                                    .fetch_add(batch.entries.len(), Ordering::Relaxed)
                                    + batch.entries.len();

                                let current_files = files_count
                                    .fetch_add(batch.files_count(), Ordering::Relaxed)
                                    + batch.files_count();

                                let current_matched = matched_count
                                    .fetch_add(batch.matched_files_count(), Ordering::Relaxed)
                                    + batch.matched_files_count();

                                // Construir índice por perfiles en tiempo real
                                if let Ok(mut index) = files_by_profile.lock() {
                                    for entry in &batch.entries {
                                        if entry.is_file {
                                            let path = PathBuf::from(&entry.path);

                                            // Determinar a qué perfil pertenece el archivo
                                            for profile in &profiles_clone {
                                                if profile.matches(&path) {
                                                    index
                                                        .entry(profile.name().clone())
                                                        .or_insert_with(Vec::new)
                                                        .push(path.clone());
                                                    break; // Solo asignar al primer perfil que coincida
                                                }
                                            }
                                        }
                                    }
                                }

                                // Actualizar estimación total
                                if batch.total_processed > 0 {
                                    total_files_estimated
                                        .store(batch.total_processed, Ordering::Relaxed);
                                }

                                // Callback con throttling optimizado
                                static mut LAST_CALLBACK_TIME: Option<std::time::Instant> = None;
                                let should_callback = unsafe {
                                    let now = std::time::Instant::now();
                                    if let Some(last_time) = LAST_CALLBACK_TIME {
                                        let time_diff = now.duration_since(last_time);
                                        if time_diff.as_secs() >= 1 || current_scanned % 200 == 0 {
                                            LAST_CALLBACK_TIME = Some(now);
                                            true
                                        } else {
                                            false
                                        }
                                    } else {
                                        LAST_CALLBACK_TIME = Some(now);
                                        true
                                    }
                                };

                                if should_callback || batch.entries.len() < 50 {
                                    let update_data = vec![
                                        ("scanned_files", current_scanned.to_string()),
                                        ("files_count", current_files.to_string()),
                                        ("analyzed_files", current_files.to_string()),
                                        ("matched_files", current_matched.to_string()),
                                        (
                                            "total_estimated",
                                            total_files_estimated
                                                .load(Ordering::Relaxed)
                                                .to_string(),
                                        ),
                                        (
                                            "current_path",
                                            batch
                                                .entries
                                                .last()
                                                .map(|e| e.path.as_str())
                                                .unwrap_or("")
                                                .to_string(),
                                        ),
                                    ];

                                    if let Err(e) = Self::notify_progress_static(
                                        &callback,
                                        "file_scanned",
                                        &update_data,
                                    ) {
                                        eprintln!("Error en callback de progreso: {}", e);
                                    }
                                }
                            }
                        }),
                        100,
                        Some(&profiles_for_walker),
                    )
                    .await;

                match result {
                    Ok(_) => Ok(walker),
                    Err(e) => Err(format!("Error en walker para path {}: {}", path_index, e)),
                }
            });

            handles.push(handle);
        }

        // Esperar todos los walkers
        let mut walkers = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(walker)) => walkers.push(walker),
                Ok(Err(e)) => return Err(e.into()),
                Err(e) => return Err(format!("Error en tarea async: {}", e).into()),
            }
        }

        // Procesar resultados y actualizar Finding con el índice construido
        let mut all_files = Vec::new();
        let mut total_files = 0;
        let mut analyzed_files = 0;

        for walker in &walkers {
            all_files.extend_from_slice(walker.files());
            total_files += walker.total_files();
            analyzed_files += walker.analyzed_files();
        }

        self.finding.set_files(all_files);
        self.finding.set_total_files(total_files);
        self.finding.set_analyzed_files_num(analyzed_files);

        // Transferir el índice por perfiles al Finding
        if let Ok(index) = files_by_profile_shared.lock() {
            self.finding.set_files_by_profile(index.clone());
        }

        // Cambiar estado a Done
        self.update_state(AnalysisState::Done)?;

        let final_matched = matched_count.load(Ordering::Relaxed);

        self.notify_progress(
            &progress_callback,
            "analysis_completed",
            &[
                ("total_files", &total_files.to_string()),
                ("analyzed_files", &analyzed_files.to_string()),
                ("matched_files", &final_matched.to_string()),
                ("state", "Done"),
            ],
        )?;

        Ok(())
    }

    // Método auxiliar para actualizar estado de manera segura
    fn update_state(
        &mut self,
        new_state: AnalysisState,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        *self
            .state
            .lock()
            .map_err(|e| format!("Error actualizando estado: {}", e))? = new_state;
        Ok(())
    }

    // Método auxiliar para notificaciones de progreso
    fn notify_progress(
        &self,
        callback: &Arc<Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>>,
        event_type: &str,
        data: &[(&str, &str)], // Cambiar a &str para evitar conversiones innecesarias
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut update_data = HashMap::new();
        for (key, value) in data {
            update_data.insert(key.to_string(), value.to_string());
        }
        callback(event_type, update_data);
        Ok(())
    }

    // Versión estática del método de notificación para usar en closures
    fn notify_progress_static(
        callback: &Arc<Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>>,
        event_type: &str,
        data: &[(&str, String)],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut update_data = HashMap::new();
        for (key, value) in data {
            update_data.insert(key.to_string(), value.clone());
        }
        callback(event_type, update_data);
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.update_state(AnalysisState::Walking)?;

        let mut handles = Vec::new();

        for path in self.config.paths() {
            let walker = Walker::new(path);
            let handle = tokio::spawn(async move {
                let mut walker = walker;
                walker.start().await?;
                Ok::<Walker, Box<dyn std::error::Error + Send + Sync>>(walker)
            });
            handles.push(handle);
        }

        // Recoger resultados con manejo de errores
        let mut walkers = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(walker)) => walkers.push(walker),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(format!("Error en tarea async: {}", e).into()),
            }
        }

        let mut all_files = Vec::new();
        let mut total_files = 0;

        for walker in &walkers {
            all_files.extend_from_slice(walker.files());
            total_files += walker.total_files();
        }

        self.finding.set_files(all_files);
        self.finding.set_total_files(total_files);

        self.update_state(AnalysisState::Analyzing)?;
        Ok(())
    }

    pub async fn analyze(
        &mut self,
        progress_callback: Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let callback_arc = Arc::new(progress_callback);
        self.start_with_progress_callback(callback_arc).await
    }

    pub fn get_findings(&self) -> Result<Vec<Finding>, String> {
        // Retornar archivos con información básica
        let files = if let Ok(files) = self.finding.files().lock() {
            files.clone()
        } else {
            return Err("Error accediendo a archivos".to_string());
        };

        let findings: Vec<Finding> = files
            .iter()
            .map(|path| Finding {
                file_path: path.clone(),
                profile_name: "General".to_string(), // Simplificado por ahora
                match_score: 0.85, // Score por defecto
            })
            .collect();

        Ok(findings)
    }

    pub fn reset(&mut self) {
        self.finding = FindingContainer::new();
        if let Ok(mut state) = self.state.lock() {
            *state = AnalysisState::Idle;
        }
    }

    pub fn get_analyzed_files(&self) -> Vec<PathBuf> {
        if let Ok(files) = self.finding.files().lock() {
            files.clone()
        } else {
            Vec::new()
        }
    }

    pub fn get_files_by_profile(&self, profile_name: &str) -> Vec<PathBuf> {
        if profile_name == "Todos" {
            return self.get_analyzed_files();
        }
        self.finding.get_files_for_profile(profile_name)
    }

    pub fn search_files_in_profile(&self, profile_name: &str, search_term: &str) -> Vec<PathBuf> {
        let files = if profile_name == "Todos" {
            self.get_analyzed_files()
        } else {
            self.get_files_by_profile(profile_name)
        };

        files
            .into_iter()
            .filter(|path| {
                path.to_string_lossy()
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
            })
            .collect()
    }

    pub fn get_profile_statistics(&self) -> HashMap<String, usize> {
        if let Ok(files_by_profile) = self.finding.files_by_profile().lock() {
            files_by_profile
                .iter()
                .map(|(profile, files)| (profile.clone(), files.len()))
                .collect()
        } else {
            HashMap::new()
        }
    }
}
