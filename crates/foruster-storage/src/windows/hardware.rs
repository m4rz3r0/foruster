// SPDX-License-Identifier: GPL-3.0-or-later
use crate::core::{Disk, IdentificationData, Partition, StorageBusType};
use std::mem::size_of;
use windows::Win32::Foundation::HANDLE;
use windows::{
    Win32::{
        Foundation::{ERROR_FILE_NOT_FOUND, GetLastError},
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
        },
        System::{
            IO::DeviceIoControl,
            Ioctl::{
                DISK_GEOMETRY_EX, DRIVE_LAYOUT_INFORMATION_EX, IOCTL_DISK_GET_DRIVE_GEOMETRY,
                IOCTL_DISK_GET_DRIVE_LAYOUT_EX, IOCTL_STORAGE_QUERY_PROPERTY,
                PARTITION_INFORMATION_EX, PARTITION_STYLE, PARTITION_STYLE_GPT,
                PARTITION_STYLE_MBR, PropertyStandardQuery, STORAGE_DEVICE_DESCRIPTOR,
                STORAGE_PROPERTY_QUERY, StorageDeviceProperty,
            },
        },
    },
    core::{HRESULT, HSTRING},
};

pub struct SafeHandle(pub HANDLE);

impl Drop for SafeHandle {
    fn drop(&mut self) {
        unsafe { windows::Win32::Foundation::CloseHandle(self.0).expect("Failed to close handle") };
    }
}

fn safe_layout_cast(buffer: &[u8]) -> Option<&DRIVE_LAYOUT_INFORMATION_EX> {
    // Verify minimum size
    if buffer.len() < size_of::<DRIVE_LAYOUT_INFORMATION_EX>() {
        return None;
    }

    // Verify buffer align
    let align = align_of::<DRIVE_LAYOUT_INFORMATION_EX>();
    if (buffer.as_ptr() as usize) % align != 0 {
        return None;
    }

    // Verify PartitionCount doesn't exceed buffer
    let layout = unsafe { &*(buffer.as_ptr() as *const DRIVE_LAYOUT_INFORMATION_EX) };
    let required_size = size_of::<DRIVE_LAYOUT_INFORMATION_EX>()
        + size_of::<PARTITION_INFORMATION_EX>() * layout.PartitionCount as usize;

    if buffer.len() < required_size {
        return None;
    }

    Some(layout)
}

fn extract_string(raw_data: &[u8], offset: u32) -> Option<String> {
    if offset == 0 {
        return None;
    }

    unsafe {
        let ptr = raw_data.as_ptr().add(offset as usize);
        Some(
            std::ffi::CStr::from_ptr(ptr as *const i8)
                .to_string_lossy()
                .into_owned(),
        )
    }
}

fn get_disk_identification_data(
    handle: &SafeHandle,
) -> Result<IdentificationData, windows::core::Error> {
    let query = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceProperty,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0; 1],
    };

    let mut raw_descriptor = vec![0u8; 1024];
    let mut bytes_returned: u32 = 0;

    unsafe {
        DeviceIoControl(
            handle.0,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some(&query as *const _ as *const _),
            size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            Some(raw_descriptor.as_mut_ptr() as *mut _),
            raw_descriptor.len() as u32,
            Some(&mut bytes_returned),
            None,
        )?;

        let descriptor = &*(raw_descriptor.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR);

        Ok(IdentificationData::new(
            extract_string(&raw_descriptor, descriptor.VendorIdOffset),
            extract_string(&raw_descriptor, descriptor.ProductIdOffset),
            extract_string(&raw_descriptor, descriptor.SerialNumberOffset),
            extract_string(&raw_descriptor, descriptor.ProductRevisionOffset),
            StorageBusType::from(descriptor.BusType.0 as u8),
            descriptor.RemovableMedia,
        ))
    }
}

fn process_mbr_partitions(partitions: &[&PARTITION_INFORMATION_EX]) -> Vec<Partition> {
    partitions
        .iter()
        .filter(|p| p.PartitionNumber != 0)
        .map(|partition| {
            let number = partition.PartitionNumber;
            let starting_offset = partition.StartingOffset as u64;
            let size = partition.PartitionLength as u64;

            Partition::new(number, starting_offset, size, None)
        })
        .collect()
}

fn process_gpt_partitions(partitions: &[&PARTITION_INFORMATION_EX]) -> Vec<Partition> {
    partitions
        .iter()
        .filter(|p| p.PartitionNumber != 0)
        .map(|partition| {
            let number = partition.PartitionNumber;
            let starting_offset = partition.StartingOffset as u64;
            let size = partition.PartitionLength as u64;

            Partition::new(number, starting_offset, size, None)
        })
        .collect()
}

pub fn enumerate_disks() -> Result<Vec<Disk>, windows::core::Error> {
    let mut disks = vec![];
    let mut disk_number = 0;

    loop {
        let disk_path = format!("\\\\.\\PhysicalDrive{}", disk_number);
        let hstring_disk_path = HSTRING::from(disk_path);

        let disk_handle_result = unsafe {
            CreateFileW(
                &hstring_disk_path,
                0,
                FILE_SHARE_READ | FILE_SHARE_WRITE, // Allow shared access
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        };

        if disk_handle_result.is_err() {
            let error = unsafe { GetLastError() };
            if error == ERROR_FILE_NOT_FOUND {
                break;
            } else {
                return Err(HRESULT::from(error).into());
            }
        }

        let disk_handle = SafeHandle(disk_handle_result?);

        // Get drive geometry
        let mut geometry = DISK_GEOMETRY_EX::default();

        unsafe {
            DeviceIoControl(
                disk_handle.0,
                IOCTL_DISK_GET_DRIVE_GEOMETRY,
                None,
                0,
                Some(&mut geometry as *mut _ as *mut _),
                size_of::<DISK_GEOMETRY_EX>() as u32,
                None,
                None,
            )?;
        };

        let geometry = geometry.Geometry;
        let total_size = geometry.Cylinders
            * geometry.TracksPerCylinder as i64
            * geometry.SectorsPerTrack as i64
            * geometry.BytesPerSector as i64;

        // Get drive layout
        let mut layout = vec![
            0u8;
            size_of::<DRIVE_LAYOUT_INFORMATION_EX>()
                + size_of::<PARTITION_INFORMATION_EX>() * 127
        ];
        let mut bytes_returned = 0;

        unsafe {
            DeviceIoControl(
                disk_handle.0,
                IOCTL_DISK_GET_DRIVE_LAYOUT_EX,
                None,
                0,
                Some(layout.as_mut_ptr() as *mut _),
                layout.len() as u32,
                Some(&mut bytes_returned),
                None,
            )?;
        };

        let layout = safe_layout_cast(&layout).expect("Invalid layout");

        let layout_partitions = unsafe {
            std::slice::from_raw_parts(
                layout.PartitionEntry.as_ptr(),
                layout.PartitionCount as usize,
            )
            .iter()
            .filter(|p| p.PartitionNumber != 0)
            .collect::<Vec<_>>() // Filter out empty partitions
        };

        let partitions: Vec<Partition> = match PARTITION_STYLE(layout.PartitionStyle as i32) {
            PARTITION_STYLE_MBR => process_mbr_partitions(layout_partitions.as_slice()),
            PARTITION_STYLE_GPT => process_gpt_partitions(layout_partitions.as_slice()),
            _ => vec![],
        };

        let identification_data = get_disk_identification_data(&disk_handle)?;

        disks.push(Disk::new(
            disk_number,
            identification_data,
            partitions,
            total_size as u64,
        ));

        disk_number += 1;
    }

    Ok(disks)
}
