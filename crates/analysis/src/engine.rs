// SPDX-License-Identifier: GPL-3.0-or-later
use crate::config::Config;
use crate::finding::{Finding, FindingContainer};
use crate::walker::{WalkBatch, Walker};
use profiling::Profile;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

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
    state: Arc<RwLock<AnalysisState>>,
    config: Arc<Config>,
    finding: FindingContainer,
    progress_callback: Option<Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>>,
}

impl Engine {
    pub fn initialize(&mut self, profiles: Vec<Profile>, paths: Vec<PathBuf>) {
        *Arc::make_mut(&mut self.config) = Config::new(profiles, paths);
    }

    pub fn state(&self) -> Arc<RwLock<AnalysisState>> {
        self.state.clone()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn finding(&self) -> &FindingContainer {
        &self.finding
    }

    pub fn set_progress_callback(
        &mut self,
        callback: Option<Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>>,
    ) {
        self.progress_callback = callback;
    }

    pub async fn run_analysis(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(callback) = self.progress_callback.take() {
            let callback_arc = Arc::new(callback);
            let result = self
                .start_with_progress_callback(callback_arc.clone())
                .await;
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
        self.update_state(AnalysisState::Walking)?;
        self.notify_progress(&progress_callback, "state_change", &[("state", "Walking")])?;

        let scanned_count = Arc::new(AtomicUsize::new(0));
        let files_count = Arc::new(AtomicUsize::new(0));
        let matched_count = Arc::new(AtomicUsize::new(0));
        let suspicious_count = Arc::new(AtomicUsize::new(0));
        let total_files_estimated = Arc::new(AtomicUsize::new(0));

        let profiles: Vec<_> = self.config.analysis_profile().clone();

        let files_by_profile_shared = Arc::new(RwLock::new(
            HashMap::<String, Vec<PathBuf>>::with_capacity(profiles.len()),
        ));
        let suspicious_files_shared = self.finding.suspicious_files().clone();

        let mut handles = Vec::with_capacity(self.config.paths().len());

        for path in self.config.paths().iter() {
            let walker = Walker::new(path);
            let progress_callback_clone = Arc::clone(&progress_callback);
            let scanned_count_clone = Arc::clone(&scanned_count);
            let files_count_clone = Arc::clone(&files_count);
            let matched_count_clone = Arc::clone(&matched_count);
            let suspicious_count_clone = Arc::clone(&suspicious_count);
            let total_files_estimated_clone = Arc::clone(&total_files_estimated);
            let profiles_clone = profiles.clone();
            let files_by_profile_clone = Arc::clone(&files_by_profile_shared);
            let suspicious_files_clone_for_thread = Arc::clone(&suspicious_files_shared);

            let handle = tokio::spawn(async move {
                let mut walker = walker;
                let profiles_for_walker = profiles_clone.clone();

                let result = walker
                    .start_with_batch_callback(
                        Box::new({
                            let callback = Arc::clone(&progress_callback_clone);
                            let scanned_count = Arc::clone(&scanned_count_clone);
                            let files_count = Arc::clone(&files_count_clone);
                            let matched_count = Arc::clone(&matched_count_clone);
                            let suspicious_count = Arc::clone(&suspicious_count_clone);
                            let total_files_estimated = Arc::clone(&total_files_estimated_clone);
                            let files_by_profile = Arc::clone(&files_by_profile_clone);
                            let suspicious_files = Arc::clone(&suspicious_files_clone_for_thread);

                            move |batch: WalkBatch| {
                                let batch_len = batch.entries.len();
                                let batch_files = batch.files_count();

                                // ** START OF REVISED LOGIC **
                                let mut batch_matched_count = 0;
                                let mut new_suspicious_files_in_batch = Vec::new();

                                // Process the batch to correlate matches and suspicious flags
                                if let Ok(mut index) = files_by_profile.try_write() {
                                    for entry in &batch.entries {
                                        if entry.is_file && !entry.matched_profiles.is_empty() {
                                            // This is a "coincidence" or match
                                            batch_matched_count += 1;
                                            let path = PathBuf::from(&entry.path);

                                            // Add to profile index as before
                                            for profile_name in &entry.matched_profiles {
                                                index.entry(profile_name.clone())
                                                    .or_default()
                                                    .push(path.clone());
                                            }

                                            // **CRITICAL CHANGE**: Only if it's a match, check if it's also suspicious.
                                            if entry.is_suspicious {
                                                new_suspicious_files_in_batch.push(path);
                                            }
                                        }
                                    }
                                }

                                let batch_suspicious_count = new_suspicious_files_in_batch.len();
                                // ** END OF REVISED LOGIC **

                                // Update atomic counters
                                let current_scanned = scanned_count.fetch_add(batch_len, Ordering::Relaxed) + batch_len;
                                let current_files = files_count.fetch_add(batch_files, Ordering::Relaxed) + batch_files;
                                let current_matched = matched_count.fetch_add(batch_matched_count, Ordering::Relaxed) + batch_matched_count;
                                let current_suspicious = suspicious_count.fetch_add(batch_suspicious_count, Ordering::Relaxed) + batch_suspicious_count;

                                // Add the newly found suspicious files to the main list
                                if batch_suspicious_count > 0 {
                                    if let Ok(mut suspicious_list) = suspicious_files.try_write() {
                                        suspicious_list.extend(new_suspicious_files_in_batch);
                                    }
                                }

                                if batch.total_processed > total_files_estimated.load(Ordering::Relaxed) {
                                    total_files_estimated.store(batch.total_processed, Ordering::Relaxed);
                                }

                                thread_local! {
                                    static LAST_CALLBACK_TIME: std::cell::Cell<Option<std::time::Instant>> = const { std::cell::Cell::new(None) };
                                }
                                let should_callback = LAST_CALLBACK_TIME.with(|last_time| {
                                    let now = std::time::Instant::now();
                                    match last_time.get() {
                                        Some(last) if now.duration_since(last).as_millis() < 250 => false,
                                        _ => {
                                            last_time.set(Some(now));
                                            true
                                        }
                                    }
                                });

                                if should_callback || batch_len < 50 {
                                    let update_data = [
                                        ("scanned_files", current_scanned.to_string()),
                                        ("analyzed_files", current_files.to_string()),
                                        ("matched_files", current_matched.to_string()),
                                        ("suspicious_files", current_suspicious.to_string()),
                                        ("total_estimated", total_files_estimated.load(Ordering::Relaxed).to_string()),
                                        ("current_path", batch.entries.last().map(|e| e.path.clone()).unwrap_or_default()),
                                    ];

                                    if let Err(e) = Self::notify_progress_static(&callback, "file_scanned", &update_data) {
                                        eprintln!("Error in progress callback: {}", e);
                                    }
                                }
                            }
                        }),
                        500,
                        Some(&profiles_for_walker),
                    )
                    .await;

                result.map(|_| walker).map_err(|e| e.to_string())
            });

            handles.push(handle);
        }

        let mut walkers = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(walker)) => walkers.push(walker),
                Ok(Err(e)) => return Err(e.into()),
                Err(e) => return Err(format!("Async task error: {}", e).into()),
            }
        }

        let mut all_files = Vec::new();
        let mut total_files_scanned = 0;
        let mut analyzed_files = 0;
        for walker in &walkers {
            all_files.extend_from_slice(walker.files());
            total_files_scanned += walker.total_files();
            analyzed_files += walker.analyzed_files();
        }

        self.finding.set_files(all_files);
        self.finding.set_total_files(total_files_scanned);
        self.finding.set_analyzed_files_num(analyzed_files);

        if let Ok(index) = Arc::try_unwrap(files_by_profile_shared) {
            self.finding.set_files_by_profile(index.into_inner().unwrap());
        }

        self.update_state(AnalysisState::Done)?;
        let final_matched = matched_count.load(Ordering::Relaxed);
        let final_suspicious = suspicious_count.load(Ordering::Relaxed);

        self.notify_progress(
            &progress_callback,
            "analysis_completed",
            &[
                ("total_files", &total_files_scanned.to_string()),
                ("analyzed_files", &analyzed_files.to_string()),
                ("matched_files", &final_matched.to_string()),
                ("suspicious_files", &final_suspicious.to_string()),
                ("state", "Done"),
            ],
        )?;

        Ok(())
    }

    // ... (The rest of the functions in engine.rs remain unchanged)

    fn update_state(
        &mut self,
        new_state: AnalysisState,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        *self
            .state
            .write()
            .map_err(|e| format!("Error actualizando estado: {}", e))? = new_state;
        Ok(())
    }

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

    // Optimización: método más eficiente para notificaciones
    fn notify_progress_static(
        callback: &Arc<Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>>,
        event_type: &str,
        data: &[(&str, String)],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Pre-allocar con capacidad conocida
        let mut update_data = HashMap::with_capacity(data.len());
        for (key, value) in data {
            update_data.insert((*key).to_string(), value.clone());
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
        let files = if let Ok(files) = self.finding.files().read() {
            files.clone()
        } else {
            return Err("Error accediendo a archivos".to_string());
        };

        let findings: Vec<Finding> = files
            .iter()
            .map(|path| Finding {
                file_path: path.clone(),
                profile_name: "General".to_string(), // Simplificado por ahora
            })
            .collect();

        Ok(findings)
    }

    pub fn reset(&mut self) {
        self.finding = FindingContainer::new();
        if let Ok(mut state) = self.state.write() {
            *state = AnalysisState::Idle;
        }
    }

    pub fn get_analyzed_files(&self) -> Vec<PathBuf> {
        if let Ok(files) = self.finding.files().read() {
            files.clone()
        } else {
            Vec::new()
        }
    }

    pub fn get_files_by_profile(&self, profile_name: &str) -> Vec<PathBuf> {
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
        if let Ok(files_by_profile) = self.finding.files_by_profile().read() {
            files_by_profile
                .iter()
                .map(|(profile, files)| (profile.clone(), files.len()))
                .collect()
        } else {
            HashMap::new()
        }
    }
}