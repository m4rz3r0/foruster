// SPDX-License-Identifier: GPL-3.0-or-later
use async_walkdir::WalkDir;
use std::path::PathBuf;
use futures_lite::stream::StreamExt;

pub struct Walker {
    path: PathBuf,
    files: Vec<PathBuf>,
    total_files: usize,
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

    pub fn total_files(&self) -> usize {
        self.total_files
    }

    pub async fn start(&mut self) {
        let mut entries =
            WalkDir::new(&self.path);

        while let Some(entry) = entries.next().await {
            if let Ok(entry) = entry {
                if entry.file_type().await.is_ok_and(|file_type| file_type.is_file() ) {
                    self.files.push(entry.path().into());
                }
            }
            self.total_files += 1;
        }
    }
}
