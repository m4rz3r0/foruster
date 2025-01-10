// SPDX-License-Identifier: GPL-3.0-or-later
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

use regex::Regex;

use crate::Disk;
use crate::DiskError;
use crate::FileSystem;

use crate::{DiskKind, Partition};

pub fn get_disks() -> Result<Vec<Disk>, DiskError> {
    let mounts = get_mounts()?;
    let re = Regex::new(r"\d+$").unwrap();

    let mut disks_info = Vec::new();
    for path in (glob::glob("/sys/block/*").map_err(DiskError::from)?).flatten() {
        let device_path = path.join("device");
        if device_path.exists() {
            let model_path = device_path.join("model");
            let model = std::fs::read_to_string(&model_path)
                .unwrap_or_else(|_| "Unknown".to_string())
                .trim()
                .to_string();

            let dev_name = path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("Unknown"))
                .to_string_lossy()
                .to_string();
            let dev_path = format!("/dev/{}", dev_name);
            let serial = get_disk_serial(&dev_path)?;

            let device_name = path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("Unknown"))
                .to_string_lossy()
                .to_string();

            let removable = std::fs::read_to_string(path.join("removable"))
                .unwrap_or_else(|_| "0".to_string())
                .trim()
                == "1";

            let size = std::fs::read_to_string(path.join("size"))
                .unwrap_or_else(|_| "0".to_string())
                .trim()
                .parse::<usize>()
                .unwrap_or(0)
                * 512;

            let kind = match std::fs::read_to_string(path.join("queue/rotational"))
                .unwrap_or_else(|_| "0".to_string())
                .trim()
                .parse()
            {
                Ok(0) => DiskKind::SSD,
                Ok(1) => DiskKind::HDD,
                _ => DiskKind::Unknown,
            };

            let mut partitions = vec![];
            for partition_path in
                (glob::glob(&format!("{}/{}*", path.to_string_lossy(), device_name))
                    .map_err(DiskError::from)?)
                .flatten()
            {
                let partition_name = partition_path
                    .file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new("Unknown"))
                    .to_string_lossy()
                    .to_string();

                let partition_id;
                if let Some(captures) = re.captures(&partition_name) {
                    if let Some(capture) = captures.get(0) {
                        if let Ok(id) = capture.as_str().parse() {
                            partition_id = id;
                        } else {
                            partition_id = 0;
                        }
                    } else {
                        partition_id = 0;
                    }
                } else {
                    partition_id = 0;
                }

                let partition_path = format!("/dev/{}", partition_name);
                let partition_filesystem = if let Some(mounts) = mounts.get(&partition_path) {
                    if let Some(mount) = mounts.first() {
                        let path = mount.path().to_owned();
                        match mount.fs_type().as_str() {
                            "btrfs" => FileSystem::BTRFS(
                                mounts.iter().map(|m| m.path().to_owned()).collect(),
                            ),
                            "ext4" => FileSystem::EXT4(path),
                            "ntfs" | "ntfs3" => FileSystem::NTFS(path),
                            "vfat" => FileSystem::FAT32(path),
                            "exfat" => FileSystem::EXFAT(path),
                            "xfs" => FileSystem::XFS(path),
                            "zfs" => FileSystem::ZFS(path),
                            _ => FileSystem::NotImplemented(mount.fs_type().to_string(), path),
                        }
                    } else {
                        FileSystem::Unknown
                    }
                } else {
                    FileSystem::Unknown
                };

                let partition_size = 0;
                let partition_aviable_space = 0;

                partitions.push(Partition::new(
                    partition_id,
                    partition_name,
                    partition_filesystem,
                    partition_size,
                    partition_aviable_space,
                ));
            }

            let disk = Disk::new(
                device_name,
                model,
                serial,
                kind,
                size,
                removable,
                partitions,
            );

            disks_info.push(disk);
        }
    }

    Ok(disks_info)
}

fn get_disk_serial(disk: &str) -> Result<String, DiskError> {
    let output = Command::new("udevadm")
        .arg("info")
        .arg("--query=all")
        .arg(format!("--name={}", disk))
        .output()
        .map_err(|e| DiskError::new(e.to_string()))?;

    if !output.status.success() {
        return Err(DiskError::new(format!(
            "Command failed with status: {}",
            output.status
        )));
    }

    // Convert the output to a string
    let stdout = from_utf8(&output.stdout)
        .map_err(|e| e.to_string())
        .map_err(|e| DiskError::new(e.to_string()))?;

    // Search for the serial number in the output
    for line in stdout.lines() {
        if line.contains("ID_SERIAL_SHORT=") {
            return Ok(line.split('=').nth(1).unwrap_or("").to_ascii_uppercase());
        }
    }

    Err(DiskError::new("Serial number not found".to_string()))
}

#[derive(Debug, Clone)]
pub struct Mount(PathBuf, String);

impl Mount {
    pub fn path(&self) -> &PathBuf {
        &self.0
    }

    pub fn fs_type(&self) -> &String {
        &self.1
    }
}

pub fn get_mounts() -> Result<HashMap<String, Vec<Mount>>, DiskError> {
    let mounts_file = std::fs::read_to_string("/proc/mounts")?;

    let mounts = mounts_file
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                match parts[0] {
                    "fusectl" | "sysfs" | "proc" | "devtmpfs" | "devpts" | "tmpfs"
                    | "securityfs" | "pstore" | "cgroup2" | "mqueue" | "hugetlbfs" | "debugfs"
                    | "configfs" | "systemd-1" | "tracefs" | "bpf" | "binfmt_misc" | "autofs"
                    | "rpc_pipefs" | "nfsd" | "sunrpc" | "devfs" | "selinuxfs" | "efivarfs"
                    | "portal" | "gvfsd-fuse" => None,
                    _ => {
                        let device = parts[0].to_string();
                        let mount_point = PathBuf::from(parts[1]);
                        let fs_type = parts[2].to_string();
                        Some((device, Mount(mount_point, fs_type)))
                    }
                }
            } else {
                None
            }
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, Vec<_>>, (key, value)| {
                acc.entry(key).or_default().push(value);
                acc
            },
        );

    Ok(mounts)
}

pub fn is_mount(mounts: &[Mount], path: &Path) -> bool {
    for mount in mounts {
        if path == mount.path() {
            return true;
        }
    }

    false
}
