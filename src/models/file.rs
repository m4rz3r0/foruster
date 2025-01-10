// SPDX-License-Identifier: GPL-3.0-or-later
use chrono::{DateTime, Local};
use serde::Serialize;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf}
};
use walkdir::DirEntry;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FileEntry {
    path: PathBuf,
    size: usize,
    modified: DateTime<Local>,
    magic_bytes: Vec<u8>,
    hash: Option<String>,
    suspicious: bool
}

impl From<DirEntry> for FileEntry {
    fn from(entry: DirEntry) -> Self {
        let date_time = match entry.metadata().map(|m| m.modified()) {
            Ok(Ok(system_time)) => DateTime::from(system_time),
            Ok(Err(e)) => {
                tracing::error!("No se puedo obtener la fecha de última modificación: {e}");
                Local::now()
            },
            Err(e) => {
                tracing::error!("No se puedo obtener la fecha de última modificación: {e}");
                Local::now()
            }
        };

        Self {
            path: entry.clone().into_path(),
            size: entry.metadata().map(|m| m.len()).unwrap_or(0) as usize,
            modified: date_time,
            magic_bytes: Vec::new(),
            hash: None,
            suspicious: false,
        }
    }
}

impl FileEntry {
    pub fn name(&self) -> &OsStr {
        self.path
            .file_name()
            .unwrap_or_else(|| self.path.as_os_str())
    }

    pub fn extension(&self) -> Option<&OsStr> {
        self.path.extension()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn into_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn modified(&self) -> DateTime<Local> {
        self.modified
    }

    pub fn magic_bytes(&self) -> &[u8] {
        &self.magic_bytes
    }

    pub fn set_magic_bytes(&mut self, bytes: &[u8]) {
        self.magic_bytes = bytes.to_vec();
    }

    pub fn hash(&self) -> Option<&String> {
        self.hash.as_ref()
    }

    pub fn suspicious(&self) -> bool {
        self.suspicious
    }

    pub fn set_suspicious(&mut self) {
        self.suspicious = true;
    }

    pub fn calculate_hash(&mut self) {
        self.hash = sha256::try_digest(self.path())
            .ok()
            .map(|hash| hash.to_string());
    }
}
