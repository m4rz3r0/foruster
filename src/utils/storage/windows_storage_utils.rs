// SPDX-License-Identifier: GPL-3.0-or-later
use std::collections::HashMap;

use wmi::{COMLibrary, Variant, WMIConnection};

use crate::{Disk, DiskError, DiskKind, FileSystem, Partition};

pub fn get_disks() -> Result<Vec<Disk>, DiskError> {
    let com_con = unsafe { COMLibrary::assume_initialized() };
    let wmi_storage_con =
        WMIConnection::with_namespace_path("ROOT\\Microsoft\\Windows\\Storage", com_con)?;
    let wmi_con = WMIConnection::new(com_con)?;

    let mut disks_wmi: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT * FROM Win32_DiskDrive")?;

    let mut disks = vec![];
    let mut partition_count = 0;
    for disk_wmi in disks_wmi.iter_mut() {
        if *disk_wmi.get("InterfaceType").unwrap() == Variant::String(String::from("USB")) {
            if let Some(Variant::String(serial_number)) = disk_wmi.get("SerialNumber") {
                let device_id: Vec<HashMap<String, Variant>> = wmi_con.raw_query(&format!("SELECT DeviceID FROM Win32_PnPSignedDriver WHERE Description LIKE '%Mass%' AND DeviceID LIKE '%{}%'", serial_number))?;
                if let Some(device) = device_id.first() {
                    if let Some(Variant::String(device_id)) = device.get("DeviceID") {
                        let serial = device_id.split('\\').last().unwrap().to_string();
                        disk_wmi
                            .entry(String::from("SerialNumber"))
                            .and_modify(|v| *v = Variant::String(serial));
                    }
                }
            }
        }
        if let Some(Variant::String(device_id)) = disk_wmi.get("DeviceID") {
            let id = device_id.chars().last().unwrap();
            let fru_id_result: Vec<HashMap<String, Variant>> = wmi_storage_con.raw_query(
                format!("SELECT * FROM MSFT_PhysicalDisk WHERE DeviceId = '{}'", id),
            )?;

            let fru_id_result = match fru_id_result.first() {
                Some(fru_id) => fru_id,
                None => {
                    continue;
                }
            };

            let serial;
            if let Some(Variant::String(fru_id_serial)) = fru_id_result.get("FruId") {
                serial = fru_id_serial.clone();
            } else {
                serial = if let Some(Variant::String(serial_number)) = disk_wmi.get("SerialNumber")
                {
                    serial_number.clone()
                } else {
                    String::new()
                };
            }

            disk_wmi
                .entry(String::from("SerialNumber"))
                .and_modify(|v| *v = Variant::String(serial));

            if let Some(Variant::String(model)) = fru_id_result.get("Model") {
                disk_wmi
                    .entry(String::from("Model"))
                    .and_modify(|v| *v = Variant::String(model.clone()));
            }

            if let Some(Variant::UI2(media_type)) = fru_id_result.get("MediaType") {
                disk_wmi.insert(String::from("Kind"), Variant::UI2(media_type.to_owned()));
            }
        }

        if let Some(Variant::Array(capabilities_descriptions)) =
            disk_wmi.get("CapabilityDescriptions")
        {
            disk_wmi.insert(
                String::from("Removable"),
                Variant::Bool(
                    capabilities_descriptions
                        .contains(&Variant::String(String::from("Supports Removable Media"))),
                ),
            );
        }

        // Get partitions
        let device_id = if let Some(Variant::String(device_id)) = disk_wmi.get("DeviceID") {
            device_id
        } else {
            continue;
        };
        let partitions = get_partitions(&wmi_con, device_id, &mut partition_count)?;

        let device_name = if let Some(Variant::String(device_name)) = disk_wmi.get("Caption") {
            device_name.to_owned()
        } else {
            continue;
        };
        let model = if let Some(Variant::String(model)) = disk_wmi.get("Model") {
            model.to_owned()
        } else {
            continue;
        };
        let serial = if let Some(Variant::String(serial)) = disk_wmi.get("SerialNumber") {
            serial.to_owned()
        } else {
            continue;
        };
        let kind = if let Some(Variant::UI2(kind)) = disk_wmi.get("Kind") {
            match kind {
                3 => DiskKind::HDD,
                4 => DiskKind::SSD,
                5 => DiskKind::SCM,
                _ => DiskKind::Unknown,
            }
        } else {
            continue;
        };
        let size = if let Some(Variant::UI8(size)) = disk_wmi.get("Size") {
            size.to_owned() as usize
        } else {
            continue;
        };
        let removable = if let Some(Variant::Bool(removable)) = disk_wmi.get("Removable") {
            removable.to_owned()
        } else {
            continue;
        };

        let disk = Disk::new(
            device_name,
            model,
            serial,
            kind,
            size,
            removable,
            partitions,
        );
        disks.push(disk);
    }

    Ok(disks)
}

fn get_partitions(
    wmi_con: &WMIConnection,
    device_id: &str,
    partition_count: &mut usize,
) -> Result<Vec<Partition>, DiskError> {
    let disk_to_partitions_query = format!(
        "ASSOCIATORS OF {{Win32_DiskDrive.DeviceID='{}'}} WHERE AssocClass=Win32_DiskDriveToDiskPartition",
        device_id
    );

    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query(disk_to_partitions_query)?;

    let mut partitions = vec![];
    for result in results {
        let partitions_to_logical_disk_query = format!(
            "ASSOCIATORS OF {{Win32_DiskPartition.DeviceID='{}'}} WHERE AssocClass=Win32_LogicalDiskToPartition",
            {if let Some(Variant::String(device_id)) = result.get("DeviceID") { device_id } else { continue; }}
        );

        let logical_disk_search_results: Vec<HashMap<String, Variant>> =
            wmi_con.raw_query(&partitions_to_logical_disk_query)?;

        let logical_disk = match logical_disk_search_results.first() {
            Some(logical_disk) => logical_disk,
            None => continue,
        };

        let partition_name = if let Some(Variant::String(partition_name)) = logical_disk.get("Name")
        {
            partition_name.to_owned()
        } else {
            continue;
        };

        let file_system = if let Some(Variant::String(file_system)) = logical_disk.get("FileSystem")
        {
            file_system
        } else {
            continue;
        };
        let mount_path = if let Some(Variant::String(mount_path)) = logical_disk.get("DeviceID") {
            mount_path.to_owned() + r"\"
        } else {
            continue;
        };
        let partition_file_system = match file_system.as_str() {
            "NTFS" => FileSystem::NTFS(mount_path.into()),
            "FAT32" => FileSystem::FAT32(mount_path.into()),
            "exFAT" => FileSystem::EXFAT(mount_path.into()),
            _ => {
                continue;
            }
        };

        let partition_total_space =
            if let Some(Variant::UI8(partition_total_space)) = logical_disk.get("Size") {
                partition_total_space.to_owned()
            } else {
                continue;
            };
        let partition_available_space =
            if let Some(Variant::UI8(partition_available_space)) = logical_disk.get("FreeSpace") {
                partition_available_space.to_owned()
            } else {
                continue;
            };

        let partition = Partition::new(
            *partition_count,
            partition_name,
            partition_file_system,
            partition_total_space as usize,
            partition_available_space as usize,
        );
        partitions.push(partition);

        *partition_count += 1;
    }

    Ok(partitions)
}
