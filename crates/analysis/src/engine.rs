// SPDX-License-Identifier: GPL-3.0-or-later
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use profiling::Profile;
use crate::config::Config;
use crate::finding::Finding;
use rayon::prelude::*;
use crate::walker::Walker;

#[derive(Default)]
pub enum AnalysisState {
    #[default]
    Idle,
    Walking,
    Analyzing,
    Done,
}

#[derive(Default)]
pub struct Engine {
    state: Rc<RefCell<AnalysisState>>,
    config: Arc<Config>,
    finding: Finding
}

impl Engine {    
    pub fn initialize(&mut self, profiles: Vec<Profile>, paths: Vec<PathBuf>) {
        *Arc::make_mut(&mut self.config) = Config::new(profiles, paths);
    }
    
    pub fn state(&self) -> Rc<RefCell<AnalysisState>> {
        self.state.clone()
    }
    
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn finding(&self) -> &Finding {
        &self.finding
    }

    pub fn start(&mut self) {
        if matches!(*self.state.borrow(), AnalysisState::Idle) {
            *self.state.borrow_mut() = AnalysisState::Walking;

            let walkers = self.config.paths().par_iter().map(|path| {
                let mut walker = Walker::new(path);

                walker.start();

                walker
            }).collect::<Vec<_>>();

            println!("Walker started with {} walkers", walkers.len());
            println!("Walkers total files: {}", walkers.iter().map(|walker| walker.total_files()).sum::<u64>());
        }
    }
}