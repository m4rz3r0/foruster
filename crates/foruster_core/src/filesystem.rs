// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

#[derive(Debug, Clone)]
pub enum Filesystem {
    FAT32,
    NTFS,
    EXT4,
    XFS,
    BTRFS,
    ZFS,
    APFS,
    HFS,
    UFS,
    EXFAT,
    FAT16,
    FAT12,
    FAT,
    EXT3,
    EXT2,
    REFS,
    JFS,
    NILFS,
    F2FS,
    UNKNOWN,
}

impl From<String> for Filesystem {
    fn from(filesystem: String) -> Self {
        match filesystem.as_str() {
            "FAT32" => Self::FAT32,
            "NTFS" => Self::NTFS,
            "EXT4" => Self::EXT4,
            "XFS" => Self::XFS,
            "BTRFS" => Self::BTRFS,
            "ZFS" => Self::ZFS,
            "APFS" => Self::APFS,
            "HFS" => Self::HFS,
            "UFS" => Self::UFS,
            "EXFAT" => Self::EXFAT,
            "FAT16" => Self::FAT16,
            "FAT12" => Self::FAT12,
            "FAT" => Self::FAT,
            "EXT3" => Self::EXT3,
            "EXT2" => Self::EXT2,
            "REFS" => Self::REFS,
            "JFS" => Self::JFS,
            "NILFS" => Self::NILFS,
            "F2FS" => Self::F2FS,
            _ => Self::UNKNOWN,
        }
    }
}

impl From<Option<String>> for Filesystem {
    fn from(filesystem: Option<String>) -> Self {
        match filesystem {
            Some(fs) => Self::from(fs),
            None => Self::UNKNOWN,
        }
    }
}

impl fmt::Display for Filesystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FAT32 => "FAT32",
                Self::NTFS => "NTFS",
                Self::EXT4 => "EXT4",
                Self::XFS => "XFS",
                Self::BTRFS => "BTRFS",
                Self::ZFS => "ZFS",
                Self::APFS => "APFS",
                Self::HFS => "HFS",
                Self::UFS => "UFS",
                Self::EXFAT => "EXFAT",
                Self::FAT16 => "FAT16",
                Self::FAT12 => "FAT12",
                Self::FAT => "FAT",
                Self::EXT3 => "EXT3",
                Self::EXT2 => "EXT2",
                Self::REFS => "REFS",
                Self::JFS => "JFS",
                Self::NILFS => "NILFS",
                Self::F2FS => "F2FS",
                Self::UNKNOWN => "Unknown",
            }
        )
    }
}
