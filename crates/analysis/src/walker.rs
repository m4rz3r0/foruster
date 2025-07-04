// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Walker {
    path: PathBuf,
    files: Vec<PathBuf>,
    total_files: u64,
}

impl Walker {
    pub fn new<P: Into<PathBuf>>(path: P) -> Walker {
        let path = path.into();
        
        Self {
            path,
            files: Vec::new(),
            total_files: 0,
        }
    }
    
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    
    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }
    
    pub fn total_files(&self) -> u64 {
        self.total_files
    }

    pub fn start(&mut self) {
        WalkDir::new(&self.path).into_iter().for_each(|entry| {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    self.files.push(entry.into_path());
                }
            }
            
            self.total_files += 1;
        })
    }
}