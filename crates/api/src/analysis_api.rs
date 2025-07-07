// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use analysis::{AnalysisState, Engine};
use profiling::Profile;

pub struct AnalysisAPI {
    engine: Arc<Mutex<Engine>>,
}

impl AnalysisAPI {
    pub fn new() -> AnalysisAPI {
        Self {
            engine: Arc::new(Mutex::new(Engine::default())),
        }
    }
    
    pub fn initialize(&mut self, profiles: Vec<Profile>, paths: Vec<PathBuf>) {
        let mut engine = self.engine.lock().unwrap();
        engine.initialize(profiles, paths);
    }
    
    pub fn analysis_state(&self) -> Arc<Mutex<AnalysisState>> {
        let engine = self.engine.lock().unwrap();
        engine.state()
    }
    
    pub fn start(&mut self) {
        /*tokio::spawn(async move {
            let mut engine = self.engine.lock().unwrap();
            engine.start();
        });*/
    }
}