// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PathItem {
    path: PathBuf,
    redundant: bool,
    redundancy_message: String,

    volume_id: String,
}

impl PathItem {
    pub fn new(path: PathBuf, volume_id: String) -> Self {
        Self {
            path,
            redundant: false,
            redundancy_message: String::new(),
            volume_id,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn volume_id(&self) -> &str {
        &self.volume_id
    }

    pub fn is_redundant(&self) -> bool {
        self.redundant
    }

    pub fn redundancy_message(&self) -> &str {
        &self.redundancy_message
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    pub fn set_volume_id(&mut self, volume_id: String) {
        self.volume_id = volume_id;
    }


    pub fn set_redundant(&mut self, redundant: bool) {
        self.redundant = redundant;
    }

    pub fn set_redundancy_message(&mut self, message: String) {
        self.redundancy_message = message;
    }
}