// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;

use crate::filesystem::Filesystem;

#[derive(Debug)]
pub struct Volume {
    guid: String,
    filesystem: Filesystem,
    size: u64,
    free_space: u64,
    drive_letters: Vec<char>,
    mount_points: Vec<PathBuf>,
}

impl Volume {
    pub fn new(
        guid: String,
        filesystem: Option<String>,
        size: u64,
        free_space: u64,
        paths: Vec<String>,
    ) -> Self {
        let (drive_letters, mount_points): (Vec<_>, Vec<_>) =
            paths.iter().partition(|p| p.ends_with(":\\"));

        let drive_letters = drive_letters
            .into_iter()
            .filter_map(|p| p.chars().next())
            .collect();

        let mount_points = mount_points.into_iter().map(PathBuf::from).collect();

        Self {
            guid,
            filesystem: Filesystem::from(filesystem),
            size,
            free_space,
            drive_letters,
            mount_points,
        }
    }

    #[inline]
    pub fn id(&self) -> &str {
        &self.guid
    }

    #[inline]
    pub fn filesystem(&self) -> &Filesystem {
        &self.filesystem
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.size
    }

    #[inline]
    pub fn free_space(&self) -> u64 {
        self.free_space
    }

    #[inline]
    pub fn drive_letters(&self) -> &[char] {
        &self.drive_letters
    }

    #[inline]
    pub fn mount_points(&self) -> &[PathBuf] {
        &self.mount_points
    }
}
