// SPDX-License-Identifier: GPL-3.0-or-later
use crate::core::Volume;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn get_volumes() -> Result<HashMap<String, Volume>> {
    let mut volumes: HashMap<String, Volume> = HashMap::new();
    let mountinfo = fs::read_to_string("/proc/self/mountinfo")?;

    for line in mountinfo.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            let mount_point = parts[4];
            let filesystem = parts[8];
            let device = parts[9];

            if is_virtual_filesystem(filesystem) || device.starts_with("/dev/loop") {
                continue;
            }

            let guid = get_device_uuid_from_sys(device).unwrap_or_else(|| device.to_string());

            if let Some(volume) = volumes.get_mut(device) {
                volume.add_mount_point(PathBuf::from(mount_point));
                continue;
            }

            if let Ok(disk_usage) = get_disk_usage_from_sys(device) {
                let volume = Volume::new(
                    guid,
                    Some(filesystem.to_string()),
                    disk_usage.total,
                    disk_usage.available,
                    vec![mount_point.to_string()],
                );
                volumes.insert(device.to_string(), volume);
            }
        }
    }

    Ok(volumes)
}

fn is_virtual_filesystem(filesystem: &str) -> bool {
    matches!(
        filesystem,
        "tmpfs"
            | "proc"
            | "sysfs"
            | "devtmpfs"
            | "cgroup"
            | "cgroup2"
            | "debugfs"
            | "tracefs"
            | "securityfs"
            | "pstore"
            | "bpf"
            | "autofs"
            | "mqueue"
            | "hugetlbfs"
    )
}

fn get_device_uuid_from_sys(device: &str) -> Option<String> {
    let device_name = device.strip_prefix("/dev/").unwrap_or(device);

    let uuid_dir = "/dev/disk/by-uuid/";
    if let Ok(entries) = fs::read_dir(uuid_dir) {
        for entry in entries.flatten() {
            if let Ok(target) = fs::read_link(entry.path()) {
                let target_str = target.to_string_lossy();
                if target_str.ends_with(device_name)
                    && let Some(uuid) = entry.file_name().to_str()
                {
                    return Some(format!("{{{}}}", uuid));
                }
            }
        }
    }
    None
}

#[derive(Debug)]
struct DiskUsage {
    total: u64,
    available: u64,
}

fn get_disk_usage_from_sys(device: &str) -> Result<DiskUsage> {
    let device_name = device.strip_prefix("/dev/").unwrap_or(device);

    let base_device = if device_name.starts_with("nvme") {
        // nvme0n1p1 -> nvme0n1
        if let Some(p_pos) = device_name.rfind('p') {
            &device_name[..p_pos]
        } else {
            device_name
        }
    } else {
        // sda1 -> sda
        device_name.trim_end_matches(char::is_numeric)
    };

    let size_path = format!("/sys/block/{}/size", base_device);
    let size_str = fs::read_to_string(size_path)?;
    let sectors = size_str.trim().parse::<u64>()?;

    let sector_size_path = format!("/sys/block/{}/queue/logical_block_size", base_device);
    let sector_size = fs::read_to_string(sector_size_path)
        .map(|s| s.trim().parse::<u64>().unwrap_or(512))
        .unwrap_or(512);

    let total = sectors * sector_size;

    let available = get_available_space_from_proc(device).unwrap_or(total);

    Ok(DiskUsage { total, available })
}

fn get_available_space_from_proc(device: &str) -> Option<u64> {
    let mounts = fs::read_to_string("/proc/mounts").ok()?;

    for line in mounts.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == device {
            let mount_point = parts[1];
            return read_mountstats(mount_point);
        }
    }
    None
}

fn read_mountstats(mount_point: &str) -> Option<u64> {
    let mountstats = fs::read_to_string("/proc/self/mountstats").ok()?;

    for line in mountstats.lines() {
        if line.contains(mount_point) && line.contains("statfs:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "bavail="
                    && i + 1 < parts.len()
                    && let Ok(available_blocks) = parts[i + 1].parse::<u64>()
                {
                    return Some(available_blocks * 4096);
                }
            }
        }
    }
    None
}
