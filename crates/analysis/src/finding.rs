// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use app_core::FileEntry;

#[derive(Default)]
pub struct Finding {
    files: Arc<Mutex<Vec<PathBuf>>>,
    analyzed_files: Arc<Mutex<Vec<FileEntry>>>,
    analyzed_files_num: usize,
    total_files: usize,
}

impl Finding {
    pub fn new() -> Finding {
        Finding {
            files: Arc::new(Mutex::new(Vec::new())),
            analyzed_files: Arc::new(Mutex::new(Vec::new())),
            analyzed_files_num: 0,
            total_files: 0,
        }
    }

    pub fn files(&self) -> &Arc<Mutex<Vec<PathBuf>>> {
        &self.files
    }

    pub fn analyzed_files(&self) -> &Arc<Mutex<Vec<FileEntry>>> {
        &self.analyzed_files
    }

    pub fn analyzed_files_num(&self) -> usize {
        self.analyzed_files_num
    }

    pub fn total_files(&self) -> usize {
        self.total_files
    }
}