// SPDX-License-Identifier: GPL-3.0-or-later
use analysis::{AnalysisState, Engine};
use profiling::Profile;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug)]
pub struct AnalysisProgress {
    pub state: AnalysisState,
    pub total_files: usize,
    pub scanned_files: usize,
    pub analyzed_files: usize,
    pub matched_files: usize,
    pub suspicious_files: usize, // New field
    pub current_path: String,
    pub elapsed_time: Duration,
    pub estimated_remaining: Option<Duration>,
}

impl Default for AnalysisProgress {
    fn default() -> Self {
        Self {
            state: AnalysisState::Idle,
            total_files: 0,
            scanned_files: 0,
            analyzed_files: 0,
            matched_files: 0,
            suspicious_files: 0, // New field
            current_path: String::new(),
            elapsed_time: Duration::ZERO,
            estimated_remaining: None,
        }
    }
}

impl AnalysisProgress {
    pub fn scan_percentage(&self) -> f32 {
        if self.total_files == 0 {
            0.0
        } else {
            (self.scanned_files as f32 / self.total_files as f32) * 100.0
        }
    }

    pub fn analysis_percentage(&self) -> f32 {
        if self.scanned_files == 0 {
            0.0
        } else {
            (self.analyzed_files as f32 / self.scanned_files as f32) * 100.0
        }
    }

    pub fn overall_percentage(&self) -> f32 {
        match self.state {
            AnalysisState::Idle => 0.0,
            AnalysisState::Walking => self.scan_percentage() * 0.3,
            AnalysisState::Analyzing => 30.0 + (self.analysis_percentage() * 0.7),
            AnalysisState::Done => 100.0,
        }
    }
}

pub enum AnalysisCommand {
    Initialize {
        profiles: Vec<Profile>,
        paths: Vec<PathBuf>,
    },
    Start,
    Stop,
}

pub struct AnalysisAPI {
    command_sender: Sender<AnalysisCommand>,
    engine: Arc<Mutex<Engine>>,
    progress: Arc<Mutex<AnalysisProgress>>,
    start_time: Arc<Mutex<Option<SystemTime>>>,
}

impl AnalysisAPI {
    pub fn new() -> AnalysisAPI {
        let (tx, rx) = mpsc::channel();
        let engine = Arc::new(Mutex::new(Engine::default()));
        let progress = Arc::new(Mutex::new(AnalysisProgress::default()));
        let start_time = Arc::new(Mutex::new(None));

        let engine_clone = Arc::clone(&engine);
        let progress_clone = Arc::clone(&progress);
        let start_time_clone = Arc::clone(&start_time);

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();

            while let Ok(command) = rx.recv() {
                match command {
                    AnalysisCommand::Initialize { profiles, paths } => {
                        if let Ok(mut eng) = engine_clone.lock() {
                            eng.initialize(profiles, paths);
                        }
                        if let Ok(mut prog) = progress_clone.lock() {
                            *prog = AnalysisProgress::default();
                        }
                    }
                    AnalysisCommand::Start => {
                        if let Ok(mut start) = start_time_clone.lock() {
                            *start = Some(SystemTime::now());
                        }

                        let engine_for_async = Arc::clone(&engine_clone);
                        let progress_for_async = Arc::clone(&progress_clone);
                        let start_time_for_async = Arc::clone(&start_time_clone);

                        rt.block_on(async move {
                            if let Ok(mut eng) = engine_for_async.lock() {
                                Self::run_analysis_with_progress(
                                    &mut eng,
                                    progress_for_async,
                                    start_time_for_async,
                                )
                                    .await;
                            }
                        });
                    }
                    AnalysisCommand::Stop => {
                        if let Ok(mut prog) = progress_clone.lock() {
                            prog.state = AnalysisState::Idle;
                        }
                    }
                }
            }
        });

        Self {
            command_sender: tx,
            engine,
            progress,
            start_time,
        }
    }

    async fn run_analysis_with_progress(
        engine: &mut Engine,
        progress: Arc<Mutex<AnalysisProgress>>,
        start_time: Arc<Mutex<Option<SystemTime>>>,
    ) {
        if let Ok(mut start) = start_time.lock() {
            *start = Some(SystemTime::now());
        }

        let progress_callback: Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync> =
            Box::new({
                let progress = Arc::clone(&progress);
                let start_time = Arc::clone(&start_time);

                move |update_type: &str, data: HashMap<String, String>| {
                    if let Ok(mut prog) = progress.lock() {
                        if let Ok(start_opt) = start_time.lock() {
                            if let Some(start) = *start_opt {
                                prog.elapsed_time = start.elapsed().unwrap_or_default();
                            }
                        }

                        match update_type {
                            "state_change" => {
                                if let Some(state_str) = data.get("state") {
                                    prog.state = match state_str.as_str() {
                                        "Walking" => AnalysisState::Walking,
                                        "Analyzing" => AnalysisState::Analyzing,
                                        "Done" => AnalysisState::Done,
                                        _ => AnalysisState::Idle,
                                    };
                                }
                            }
                            "file_scanned" => {
                                data.get("scanned_files").and_then(|s| s.parse().ok()).map(|v| prog.scanned_files = v);
                                data.get("analyzed_files").and_then(|s| s.parse().ok()).map(|v| prog.analyzed_files = v);
                                data.get("matched_files").and_then(|s| s.parse().ok()).map(|v| prog.matched_files = v);
                                data.get("suspicious_files").and_then(|s| s.parse().ok()).map(|v| prog.suspicious_files = v); // Update suspicious
                                data.get("current_path").map(|v| prog.current_path = v.clone());
                                data.get("total_estimated").and_then(|s| s.parse().ok()).map(|v| prog.total_files = v);
                            }
                            "analysis_completed" => {
                                data.get("total_files").and_then(|s| s.parse().ok()).map(|v| prog.total_files = v);
                                data.get("analyzed_files").and_then(|s| s.parse().ok()).map(|v| prog.analyzed_files = v);
                                data.get("matched_files").and_then(|s| s.parse().ok()).map(|v| prog.matched_files = v);
                                data.get("suspicious_files").and_then(|s| s.parse().ok()).map(|v| prog.suspicious_files = v); // Update suspicious
                                prog.state = AnalysisState::Done;
                            }
                            _ => {}
                        }

                        if prog.total_files > 0 && prog.elapsed_time.as_secs() > 0 {
                            let progress_ratio = match prog.state {
                                AnalysisState::Walking if prog.total_files > 0 => prog.analyzed_files as f64 / prog.total_files as f64,
                                AnalysisState::Done => 1.0,
                                _ => 0.0,
                            };

                            if progress_ratio > 0.0 && progress_ratio < 1.0 {
                                let elapsed_secs = prog.elapsed_time.as_secs_f64();
                                let estimated_total_time = elapsed_secs / progress_ratio;
                                let remaining_time = estimated_total_time - elapsed_secs;
                                if remaining_time > 0.0 {
                                    prog.estimated_remaining = Some(Duration::from_secs_f64(remaining_time));
                                }
                            } else if progress_ratio >= 1.0 {
                                prog.estimated_remaining = Some(Duration::ZERO);
                            }
                        }
                    }
                }
            });

        if let Err(e) = engine.analyze(progress_callback).await {
            eprintln!("Error during analysis: {}", e);
        }
    }

    pub fn initialize(&self, profiles: Vec<Profile>, paths: Vec<PathBuf>) -> Result<(), String> {
        self.command_sender
            .send(AnalysisCommand::Initialize { profiles, paths })
            .map_err(|e| e.to_string())
    }

    pub fn start_analysis(&self) -> Result<(), String> {
        self.command_sender
            .send(AnalysisCommand::Start)
            .map_err(|e| e.to_string())
    }

    pub fn stop_analysis(&self) -> Result<(), String> {
        self.command_sender
            .send(AnalysisCommand::Stop)
            .map_err(|e| e.to_string())
    }

    pub fn get_progress(&self) -> Result<AnalysisProgress, String> {
        self.progress.lock().map(|prog| prog.clone()).map_err(|e| e.to_string())
    }

    pub fn is_running(&self) -> bool {
        if let Ok(progress) = self.progress.lock() {
            matches!(progress.state, AnalysisState::Walking | AnalysisState::Analyzing)
        } else {
            false
        }
    }

    pub fn get_findings(&self) -> Result<Vec<analysis::Finding>, String> {
        self.engine.lock().map_err(|e| e.to_string())?.get_findings()
    }

    pub fn get_suspicious_files(&self) -> Vec<std::path::PathBuf> {
        self.engine
            .lock()
            .map(|engine| {
                if let Ok(files) = engine.finding().suspicious_files().read() {
                    files.clone()
                } else {
                    Vec::new()
                }
            })
            .unwrap_or_default()
    }

    pub fn reset(&self) -> Result<(), String> {
        if let Ok(mut prog) = self.progress.lock() {
            *prog = AnalysisProgress::default();
        }
        if let Ok(mut start) = self.start_time.lock() {
            *start = None;
        }
        if let Ok(mut engine) = self.engine.lock() {
            engine.reset();
        }
        Ok(())
    }

    pub fn get_analysis_summary(&self) -> Result<AnalysisSummary, String> {
        let progress = self.get_progress()?;
        let findings = self.get_findings()?;

        Ok(AnalysisSummary {
            total_files_scanned: progress.scanned_files,
            total_files_analyzed: progress.analyzed_files,
            total_matches_found: progress.matched_files,
            total_suspicious_files: progress.suspicious_files,
            analysis_duration: progress.elapsed_time,
            state: progress.state,
            findings_by_profile: Self::group_findings_by_profile(&findings),
        })
    }

    fn group_findings_by_profile(findings: &[analysis::Finding]) -> HashMap<String, usize> {
        let mut profile_counts = HashMap::new();
        for finding in findings {
            *profile_counts.entry(finding.profile_name.clone()).or_insert(0) += 1;
        }
        profile_counts
    }

    pub fn get_files_by_profile(&self, profile_name: &str) -> Vec<std::path::PathBuf> {
        self.engine
            .lock()
            .map(|engine| engine.get_files_by_profile(profile_name))
            .unwrap_or_default()
    }

    pub fn get_analyzed_files(&self) -> Vec<std::path::PathBuf> {
        self.engine
            .lock()
            .map(|engine| engine.get_analyzed_files())
            .unwrap_or_default()
    }

    pub fn search_files_in_profile(&self, profile_name: &str, search_term: &str) -> Vec<std::path::PathBuf> {
        self.engine
            .lock()
            .map(|engine| engine.search_files_in_profile(profile_name, search_term))
            .unwrap_or_default()
    }

    pub fn get_profile_statistics(&self) -> HashMap<String, usize> {
        self.engine
            .lock()
            .map(|engine| engine.get_profile_statistics())
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisSummary {
    pub total_files_scanned: usize,
    pub total_files_analyzed: usize,
    pub total_matches_found: usize,
    pub total_suspicious_files: usize, // New field
    pub analysis_duration: Duration,
    pub state: AnalysisState,
    pub findings_by_profile: HashMap<String, usize>,
}

impl Default for AnalysisAPI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_progress_percentages() {
        let mut progress = AnalysisProgress::default();
        progress.total_files = 100;
        progress.scanned_files = 50;
        progress.analyzed_files = 25;

        assert_eq!(progress.scan_percentage(), 50.0);
        assert_eq!(progress.analysis_percentage(), 50.0);
    }

    #[test]
    fn test_analysis_api_creation() {
        let api = AnalysisAPI::new();
        assert!(!api.is_running());
        let progress = api.get_progress().unwrap();
        assert!(matches!(progress.state, AnalysisState::Idle));
    }

    #[test]
    fn test_progress_overall_percentage() {
        let mut progress = AnalysisProgress::default();
        assert_eq!(progress.overall_percentage(), 0.0);
        progress.state = AnalysisState::Walking;
        progress.total_files = 100;
        progress.scanned_files = 50;
        assert_eq!(progress.overall_percentage(), 15.0);
        progress.state = AnalysisState::Analyzing;
        progress.analyzed_files = 25;
        assert_eq!(progress.overall_percentage(), 65.0);
        progress.state = AnalysisState::Done;
        assert_eq!(progress.overall_percentage(), 100.0);
    }
}