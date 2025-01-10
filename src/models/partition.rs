// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileSystem {
    BTRFS(Vec<PathBuf>),
    EXT4(PathBuf),
    NTFS(PathBuf),
    FAT32(PathBuf),
    EXFAT(PathBuf),
    XFS(PathBuf),
    ZFS(PathBuf),
    NotImplemented(String, PathBuf),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Partition {
    id: usize,
    name: String,
    file_system: FileSystem,
    total_space: usize,
    available_space: usize,
}

impl Partition {
    pub fn new(
        id: usize,
        name: String,
        file_system: FileSystem,
        total_space: usize,
        available_space: usize,
    ) -> Self {
        Partition {
            id,
            name,
            file_system,
            total_space,
            available_space,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_system(&self) -> &FileSystem {
        &self.file_system
    }

    pub fn total_space(&self) -> usize {
        self.total_space
    }

    pub fn available_space(&self) -> usize {
        self.available_space
    }
}
