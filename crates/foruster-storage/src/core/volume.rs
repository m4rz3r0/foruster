// SPDX-License-Identifier: GPL-3.0-or-later
use std::{fmt, path::PathBuf};

#[derive(Debug, Clone)]
pub enum Filesystem {
    Fat32,
    Ntfs,
    Ext4,
    Xfs,
    Btrfs,
    Zfs,
    Apfs,
    Hfs,
    Ufs,
    Exfat,
    Fat16,
    Fat12,
    Fat,
    Ext3,
    Ext2,
    Refs,
    Jfs,
    Nilfs,
    F2fs,
    Unknown,
}

impl From<String> for Filesystem {
    fn from(filesystem: String) -> Self {
        match filesystem.to_ascii_lowercase().as_str() {
            "fat32" => Self::Fat32,
            "ntfs" => Self::Ntfs,
            "ext4" => Self::Ext4,
            "xfs" => Self::Xfs,
            "btrfs" => Self::Btrfs,
            "zfs" => Self::Zfs,
            "apfs" => Self::Apfs,
            "hfs" => Self::Hfs,
            "ufs" => Self::Ufs,
            "exfat" => Self::Exfat,
            "fat16" => Self::Fat16,
            "fat12" => Self::Fat12,
            "fat" => Self::Fat,
            "ext3" => Self::Ext3,
            "ext2" => Self::Ext2,
            "refs" => Self::Refs,
            "jfs" => Self::Jfs,
            "nilfs" => Self::Nilfs,
            "f2fs" => Self::F2fs,
            _ => Self::Unknown,
        }
    }
}

impl From<Option<String>> for Filesystem {
    fn from(filesystem: Option<String>) -> Self {
        match filesystem {
            Some(fs) => Self::from(fs),
            None => Self::Unknown,
        }
    }
}

impl fmt::Display for Filesystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fat32 => "FAT32",
                Self::Ntfs => "NTFS",
                Self::Ext4 => "EXT4",
                Self::Xfs => "XFS",
                Self::Btrfs => "BTRFS",
                Self::Zfs => "ZFS",
                Self::Apfs => "APFS",
                Self::Hfs => "HFS",
                Self::Ufs => "UFS",
                Self::Exfat => "EXFAT",
                Self::Fat16 => "FAT16",
                Self::Fat12 => "FAT12",
                Self::Fat => "FAT",
                Self::Ext3 => "EXT3",
                Self::Ext2 => "EXT2",
                Self::Refs => "REFS",
                Self::Jfs => "JFS",
                Self::Nilfs => "NILFS",
                Self::F2fs => "F2FS",
                Self::Unknown => "Unknown",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Volume {
    guid: String,
    filesystem: Filesystem,
    size: u64,
    free_space: u64,
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
        let mount_points = paths.into_iter().map(PathBuf::from).collect();

        Self {
            guid,
            filesystem: Filesystem::from(filesystem),
            size,
            free_space,
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
        !self.mount_points.is_empty()
    }

    pub fn add_mount_point(&mut self, path: PathBuf) {
        if !self.mount_points.contains(&path) {
            self.mount_points.push(path);
        }
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
