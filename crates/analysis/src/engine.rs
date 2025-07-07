// SPDX-License-Identifier: GPL-3.0-or-later
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use profiling::Profile;
use crate::config::Config;
use crate::finding::Finding;
use rayon::prelude::*;
use crate::walker::Walker;

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

    pub async fn start_with_progress_callback(
        &mut self,
        progress_callback: Box<dyn Fn(&str, HashMap<String, String>) + Send + Sync>
    ) {
        let mut update_data = HashMap::new();
        update_data.insert("state".to_string(), "Walking".to_string());
        progress_callback("state_change", update_data);

        *self.state.lock().unwrap() = AnalysisState::Walking;

        // Tu lógica de análisis existente con callbacks de progreso
        // Ejemplo de cómo usar el callback:
        let mut update_data = HashMap::new();
        update_data.insert("total".to_string(), "1000".to_string());
        progress_callback("total_files_estimated", update_data);

        // Durante el escaneo de archivos:
        // let mut update_data = HashMap::new();
        // update_data.insert("path".to_string(), current_file_path);
        // progress_callback("file_scanned", update_data);
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