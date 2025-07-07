// SPDX-License-Identifier: GPL-3.0-or-later
use analysis::{AnalysisState, Engine};
use profiling::Profile;
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
            AnalysisState::Walking => self.scan_percentage() * 0.3, // 30% para escaneo
            AnalysisState::Analyzing => 30.0 + (self.analysis_percentage() * 0.7), // 70% para análisis
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
    GetProgress,
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

        // Crear hilo dedicado para operaciones de análisis
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();

            while let Ok(command) = rx.recv() {
                match command {
                    AnalysisCommand::Initialize { profiles, paths } => {
                        if let Ok(mut eng) = engine_clone.lock() {
                            eng.initialize(profiles, paths);
                        }

                        // Resetear progreso
                        if let Ok(mut prog) = progress_clone.lock() {
                            *prog = AnalysisProgress::default();
                        }
                    }
                    AnalysisCommand::Start => {
                        // Marcar tiempo de inicio
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
                        // Implementar lógica de parada si es necesario
                        if let Ok(mut prog) = progress_clone.lock() {
                            prog.state = AnalysisState::Idle;
                        }
                    }
                    AnalysisCommand::GetProgress => {
                        // Este comando se maneja directamente desde el getter
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
        // Actualizar estado a Walking
        if let Ok(mut prog) = progress.lock() {
            prog.state = AnalysisState::Walking;
        }

        // Ejecutar análisis con actualizaciones de progreso
        engine
            .start_with_progress_callback(Box::new(move |update_type, data| {
                if let Ok(mut prog) = progress.lock() {
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
                            if let Some(path) = data.get("path") {
                                prog.current_path = path.clone();
                                prog.scanned_files += 1;
                            }
                        }
                        "file_analyzed" => {
                            prog.analyzed_files += 1;
                        }
                        "file_matched" => {
                            prog.matched_files += 1;
                        }
                        "total_files_estimated" => {
                            if let Some(total_str) = data.get("total") {
                                if let Ok(total) = total_str.parse::<usize>() {
                                    prog.total_files = total;
                                }
                            }
                        }
                        _ => {}
                    }

                    // Actualizar tiempo transcurrido y estimación
                    if let Ok(start_opt) = start_time.lock() {
                        if let Some(start) = *start_opt {
                            prog.elapsed_time = start.elapsed().unwrap_or_default();

                            // Calcular tiempo estimado restante
                            if prog.scanned_files > 0 && prog.total_files > prog.scanned_files {
                                let rate =
                                    prog.scanned_files as f64 / prog.elapsed_time.as_secs_f64();
                                let remaining_files = prog.total_files - prog.scanned_files;
                                let estimated_seconds = remaining_files as f64 / rate;
                                prog.estimated_remaining =
                                    Some(Duration::from_secs_f64(estimated_seconds));
                            }
                        }
                    }
                }
            }))
            .await;
    }

    pub fn initialize(&self, profiles: Vec<Profile>, paths: Vec<PathBuf>) {
        let _ = self
            .command_sender
            .send(AnalysisCommand::Initialize { profiles, paths });
    }

    pub fn start(&self) {
        let _ = self.command_sender.send(AnalysisCommand::Start);
    }

    pub fn stop(&self) {
        let _ = self.command_sender.send(AnalysisCommand::Stop);
    }

    // Método no bloqueante para obtener el progreso actual
    pub fn get_progress(&self) -> AnalysisProgress {
        self.progress
            .lock()
            .map(|prog| prog.clone())
            .unwrap_or_default()
    }

    // Método para obtener solo el porcentaje (más eficiente)
    pub fn get_progress_percentage(&self) -> f32 {
        self.progress
            .lock()
            .map(|prog| prog.overall_percentage())
            .unwrap_or(0.0)
    }

    // Método para verificar si está en progreso
    pub fn is_running(&self) -> bool {
        self.progress
            .lock()
            .map(|prog| !matches!(prog.state, AnalysisState::Idle | AnalysisState::Done))
            .unwrap_or(false)
    }

    pub fn analysis_state(&self) -> Arc<Mutex<AnalysisState>> {
        if let Ok(engine) = self.engine.lock() {
            engine.state()
        } else {
            Arc::new(Mutex::new(AnalysisState::default()))
        }
    }
}
