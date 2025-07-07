// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
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
    state: Arc<Mutex<AnalysisState>>,
    config: Arc<Config>,
    finding: Finding
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

    pub fn finding(&self) -> &Finding {
        &self.finding
    }

    pub async fn start(&mut self) {
        *self.state.lock().unwrap() = AnalysisState::Walking;

        let handles: Vec<_> = self.config.paths().par_iter().map(|path| {
            let mut walker = Walker::new(path);
            tokio::spawn(async move {
                walker.start().await;
                walker // Devolver el walker del async block
            })
        }).collect();

        // Esperar a que todos los walkers terminen y recoger los resultados
        let walkers = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();

        let files = walkers.iter().map(|walker| walker.files().iter().cloned().collect()).collect::<Vec<_>>();
        let total_files = walkers.iter().map(|walker| walker.total_files()).sum::<usize>();

        self.finding.set_files(files);
        self.finding.set_total_files(total_files);

        println!("Files: {:?}", self.finding.files());
        println!("Total files: {}", self.finding.total_files());

        *self.state.lock().unwrap() = AnalysisState::Analyzing;
    }
}