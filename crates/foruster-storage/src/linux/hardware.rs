// SPDX-License-Identifier: GPL-3.0-or-later
use crate::core::{Disk, IdentificationData, Partition, StorageBusType};
use anyhow::Result;
use std::fs;

const SKIP_ENTRIES: [&str; 7] = [".", "..", "dm-", "loop", "ram", "zram", "fd"];

fn get_disk_field(disk_name: &str, field: &str) -> Option<String> {
    let path = format!("/sys/block/{}/{}", disk_name, field);
    match fs::read_to_string(path) {
        Ok(content) => {
            let trimmed = content.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
        Err(_) => None,
    }
}

fn detect_bus_type(disk_name: &str) -> StorageBusType {
    if let Ok(target) = std::fs::read_link(format!("/sys/block/{}/device", disk_name)) {
        let path_str = target.to_string_lossy();

        if path_str.contains("nvme") {
            return StorageBusType::Nvme;
        } else if path_str.contains("ata") || path_str.contains("scsi") {
            return StorageBusType::Sata;
        } else if path_str.contains("usb") {
            return StorageBusType::Usb;
        }
    }

    StorageBusType::Unknown
}

fn get_partition_info(disk_name: &str, partition_name: &str) -> Result<(u64, u64)> {
    let start_path = format!("/sys/block/{}/{}/start", disk_name, partition_name);
    let size_path = format!("/sys/block/{}/{}/size", disk_name, partition_name);

    let start_sectors = fs::read_to_string(start_path)?.trim().parse::<u64>()?;
    let size_sectors = fs::read_to_string(size_path)?.trim().parse::<u64>()?;

    Ok((start_sectors * 512, size_sectors * 512))
}

pub(crate) fn get_partitions_for_disk(disk_name: &str) -> Result<Vec<Partition>> {
    let mut partitions = Vec::new();
    let block_dir = format!("/sys/block/{}", disk_name);

    if let Ok(entries) = fs::read_dir(&block_dir) {
        for entry in entries {
            let entry = entry?;
            let entry_name = entry.file_name().to_string_lossy().to_string();

            if entry_name.starts_with(disk_name) && entry_name != disk_name {
                let partition_number = if disk_name.starts_with("nvme") {
                    if let Some(p_pos) = entry_name.rfind('p') {
                        entry_name[(p_pos + 1)..].parse::<u32>().unwrap_or(0)
                    } else {
                        0
                    }
                } else {
                    entry_name
                        .trim_start_matches(disk_name)
                        .parse::<u32>()
                        .unwrap_or(0)
                };

                if partition_number > 0
                    && let Ok((start_offset, size)) = get_partition_info(disk_name, &entry_name)
                {
                    let partition = Partition::new(partition_number, start_offset, size, None);
                    partitions.push(partition);
                }
            }
        }
    }

    partitions.sort_by_key(|p| p.number());
    Ok(partitions)
}

pub(crate) fn get_disks() -> Result<Vec<Disk>> {
    let mut disks = Vec::new();
    let disk_entries = std::fs::read_dir("/sys/block")?;

    for disk in disk_entries {
        let disk = disk?;
        let disk_name = disk.file_name().to_string_lossy().to_string();

        if SKIP_ENTRIES
            .iter()
            .any(|&entry| disk_name.starts_with(entry))
        {
            continue;
        }

        let model = get_disk_field(&disk_name, "device/model").unwrap_or_default();
        let vendor = get_disk_field(&disk_name, "device/vendor").unwrap_or_default();
        let serial = get_disk_field(&disk_name, "device/serial").unwrap_or_default();
        let firmware_rev = get_disk_field(&disk_name, "device/firmware_rev").unwrap_or_default();
        let size = get_disk_field(&disk_name, "size")
            .and_then(|s| s.parse::<u64>().ok())
            .map(|s| s * 512) // Convert from sectors to bytes
            .unwrap_or(0);
        let bus_type = detect_bus_type(&disk_name);

        let identification_data = IdentificationData::new(
            if vendor.is_empty() {
                None
            } else {
                Some(vendor)
            },
            if model.is_empty() { None } else { Some(model) },
            if serial.is_empty() {
                None
            } else {
                Some(serial)
            },
            if firmware_rev.is_empty() {
                None
            } else {
                Some(firmware_rev)
            },
            bus_type,
            false,
        );

        let id = disk
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let partitions = get_partitions_for_disk(&id)?;

        let disk_info = Disk::new(id, identification_data, partitions, size);

        disks.push(disk_info);
    }

    Ok(disks)
}
