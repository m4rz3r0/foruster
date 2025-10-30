// SPDX-License-Identifier: GPL-3.0-or-later
use app_core::FileEntry;
use file_format::Kind;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub enum SuspicionReason {
    DeceptiveExtension { hidden_ext: String },
    ContentMismatch { expected: Kind, actual: Kind },
}

impl fmt::Display for SuspicionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuspicionReason::DeceptiveExtension { hidden_ext } => {
                write!(
                    f,
                    "Extensión engañosa, parece ocultar un archivo '.{}'",
                    hidden_ext
                )
            }
            SuspicionReason::ContentMismatch { expected, actual } => {
                write!(
                    f,
                    "El contenido parece ser '{:?}' pero la extensión indica '{:?}'",
                    actual, expected
                )
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Finding {
    pub file_path: PathBuf,
    pub profile_name: String,
}

#[derive(Default)]
pub struct FindingContainer {
    files: Arc<RwLock<Vec<PathBuf>>>,
    analyzed_files: Arc<RwLock<Vec<FileEntry>>>,
    matched_files: Arc<RwLock<Vec<FileEntry>>>,
    files_by_profile: Arc<RwLock<HashMap<String, Vec<PathBuf>>>>,
    suspicious_files: Arc<RwLock<Vec<(PathBuf, SuspicionReason)>>>,
    analyzed_files_num: usize,
    total_files: usize,
}

impl FindingContainer {
    pub fn new() -> FindingContainer {
        FindingContainer {
            files: Arc::new(RwLock::new(Vec::new())),
            analyzed_files: Arc::new(RwLock::new(Vec::new())),
            matched_files: Arc::new(RwLock::new(Vec::new())),
            files_by_profile: Arc::new(RwLock::new(HashMap::new())),
            suspicious_files: Arc::new(RwLock::new(Vec::new())),
            analyzed_files_num: 0,
            total_files: 0,
        }
    }

    pub fn files(&self) -> &Arc<RwLock<Vec<PathBuf>>> {
        &self.files
    }

    pub fn suspicious_files(&self) -> &Arc<RwLock<Vec<(PathBuf, SuspicionReason)>>> {
        &self.suspicious_files
    }

    pub fn analyzed_files(&self) -> &Arc<RwLock<Vec<FileEntry>>> {
        &self.analyzed_files
    }

    pub fn matched_files(&self) -> &Arc<RwLock<Vec<FileEntry>>> {
        &self.matched_files
    }

    pub fn analyzed_files_num(&self) -> usize {
        self.analyzed_files_num
    }

    pub fn total_files(&self) -> usize {
        self.total_files
    }

    pub fn set_files(&mut self, files: Vec<PathBuf>) {
        let mut locked_files = self.files.write().unwrap();
        *locked_files = files;
    }

    pub fn set_analyzed_files(&mut self, files: Vec<FileEntry>) {
        let mut locked_analyzed_files = self.analyzed_files.write().unwrap();
        *locked_analyzed_files = files;
    }

    pub fn set_matched_files(&mut self, files: Vec<FileEntry>) {
        let mut locked_matched_files = self.matched_files.write().unwrap();
        *locked_matched_files = files;
    }

    pub fn set_analyzed_files_num(&mut self, num: usize) {
        self.analyzed_files_num = num;
    }

    pub fn set_total_files(&mut self, num: usize) {
        self.total_files = num;
    }

    pub fn files_by_profile(&self) -> &Arc<RwLock<HashMap<String, Vec<PathBuf>>>> {
        &self.files_by_profile
    }

    pub fn set_files_by_profile(&mut self, files_by_profile: HashMap<String, Vec<PathBuf>>) {
        let mut locked_files_by_profile = self.files_by_profile.write().unwrap();
        *locked_files_by_profile = files_by_profile;
    }

    pub fn add_file_to_profile(&self, profile_name: String, file_path: PathBuf) {
        if let Ok(mut files_by_profile) = self.files_by_profile.write() {
            files_by_profile
                .entry(profile_name)
                .or_insert_with(Vec::new)
                .push(file_path);
        }
    }

    pub fn get_files_for_profile(&self, profile_name: &str) -> Vec<PathBuf> {
        if let Ok(files_by_profile) = self.files_by_profile.read() {
            files_by_profile
                .get(profile_name)
                .cloned()
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    pub fn get_all_profile_names(&self) -> Vec<String> {
        if let Ok(files_by_profile) = self.files_by_profile.read() {
            files_by_profile.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn clear_profile_index(&self) {
        if let Ok(mut files_by_profile) = self.files_by_profile.write() {
            files_by_profile.clear();
        }
    }
}
