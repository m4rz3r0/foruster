// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

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
        match filesystem.to_uppercase().as_str() {
            "FAT32" => Self::Fat32,
            "NTFS" => Self::Ntfs,
            "EXT4" => Self::Ext4,
            "XFS" => Self::Xfs,
            "BTRFS" => Self::Btrfs,
            "ZFS" => Self::Zfs,
            "APFS" => Self::Apfs,
            "HFS" => Self::Hfs,
            "UFS" => Self::Ufs,
            "EXFAT" => Self::Exfat,
            "FAT16" => Self::Fat16,
            "FAT12" => Self::Fat12,
            "FAT" => Self::Fat,
            "EXT3" => Self::Ext3,
            "EXT2" => Self::Ext2,
            "REFS" => Self::Refs,
            "JFS" => Self::Jfs,
            "NILFS" => Self::Nilfs,
            "F2FS" => Self::F2fs,
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
