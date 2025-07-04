// SPDX-License-Identifier: GPL-3.0-or-later
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use analysis::{AnalysisState, Engine};
use profiling::Profile;

pub struct AnalysisAPI {
    engine: Engine,
}

impl AnalysisAPI {
    pub fn new() -> AnalysisAPI {
        Self {
            engine: Engine::default(),
        }
    }
    
    pub fn initialize(&mut self, profiles: Vec<Profile>, paths: Vec<PathBuf>) {
        self.engine.initialize(profiles, paths);
    }
    
    pub fn analysis_state(&self) -> Rc<RefCell<AnalysisState>> {
        self.engine.state()
    }
    
    pub fn start(&mut self) {
        self.engine.start();
    }
}