// SPDX-License-Identifier: GPL-3.0-or-later
use std::{fmt, path::PathBuf};

use crate::filesystem::Filesystem;

#[derive(Debug, Clone)]
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
    pub fn guid(&self) -> &str {
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

    pub fn guid_identifier(&self) -> String {
        let start = self.guid.find('{').unwrap_or(0) + 1;
        let end = self.guid.rfind('}').unwrap_or(self.guid.len());

        self.guid[start..end].to_string()
    }

    pub fn size_in_gb(&self) -> f64 {
        self.size as f64 / 1_073_741_824.0
    }

    pub fn free_space_in_gb(&self) -> f64 {
        self.free_space as f64 / 1_073_741_824.0
    }

    pub fn is_mounted(&self) -> bool {
        !self.drive_letters.is_empty() || !self.mount_points.is_empty()
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} - {:.2} GB / {:.2} GB",
            self.guid,
            self.filesystem,
            self.free_space_in_gb(),
            self.size_in_gb()
        )?;

        if self.is_mounted() {
            write!(f, " - Mounted")?;
        } else {
            write!(f, " - Not mounted")?;
        }

        if !self.drive_letters.is_empty() {
            write!(
                f,
                " - Drive letter(s): {}",
                self.drive_letters.iter().collect::<String>()
            )?;
        }

        if !self.mount_points.is_empty() {
            write!(
                f,
                " - Mount point(s): {}",
                self.mount_points
                    .iter()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        Ok(())
    }
}
