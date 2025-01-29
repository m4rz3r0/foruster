// SPDX-License-Identifier: GPL-3.0-or-later
use std::mem::size_of;
use windows::{
    core::{HRESULT, HSTRING},
    Win32::{
        Foundation::{CloseHandle, GetLastError, ERROR_FILE_NOT_FOUND, HANDLE},
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
            STORAGE_BUS_TYPE,
        },
        System::{
            Ioctl::{
                PropertyStandardQuery, StorageDeviceProperty, DISK_GEOMETRY,
                DRIVE_LAYOUT_INFORMATION_EX, IOCTL_DISK_GET_DRIVE_GEOMETRY,
                IOCTL_DISK_GET_DRIVE_LAYOUT_EX, IOCTL_STORAGE_QUERY_PROPERTY,
                PARTITION_INFORMATION_EX, PARTITION_STYLE, PARTITION_STYLE_GPT,
                PARTITION_STYLE_MBR, STORAGE_DEVICE_DESCRIPTOR, STORAGE_PROPERTY_QUERY,
            },
            IO::DeviceIoControl,
        },
    },
};

use crate::utils::safe_layout_cast;
use foruster_core::Disk;

fn get_disk_identification_data(handle: HANDLE) -> Result<(), windows::core::Error> {
    let mut query = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceProperty,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0; 1],
    };

    let mut raw_descriptor = vec![0u8; 1024];
    let mut bytes_returned: u32 = 0;
    unsafe {
        DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some(&mut query as *mut _ as *mut _),
            size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            Some(raw_descriptor.as_mut_ptr() as *mut _),
            raw_descriptor.len() as u32,
            Some(&mut bytes_returned),
            None,
        )?;
    }

    println!("Bytes Returned: {}", bytes_returned);

    let descriptor = unsafe { &*(raw_descriptor.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR) };

    //println!("Raw Descriptor: {:?}", raw_descriptor);

    let serial_number = if descriptor.SerialNumberOffset != 0 {
        unsafe {
            let ptr = raw_descriptor
                .as_ptr()
                .add(descriptor.SerialNumberOffset as usize);
            Some(
                std::ffi::CStr::from_ptr(ptr as *const i8)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    } else {
        None
    };
    println!("Serial Number: {:?}", serial_number);

    let vendor_id = if descriptor.VendorIdOffset != 0 {
        unsafe {
            let ptr = raw_descriptor
                .as_ptr()
                .add(descriptor.VendorIdOffset as usize);
            Some(
                std::ffi::CStr::from_ptr(ptr as *const i8)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    } else {
        None
    };
    println!("Vendor ID: {:?}", vendor_id);

    let product_id = if descriptor.ProductIdOffset != 0 {
        unsafe {
            let ptr = raw_descriptor
                .as_ptr()
                .add(descriptor.ProductIdOffset as usize);
            Some(
                std::ffi::CStr::from_ptr(ptr as *const i8)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    } else {
        None
    };
    println!("Product ID: {:?}", product_id);

    let product_revision = if descriptor.ProductRevisionOffset != 0 {
        unsafe {
            let ptr = raw_descriptor
                .as_ptr()
                .add(descriptor.ProductRevisionOffset as usize);
            Some(
                std::ffi::CStr::from_ptr(ptr as *const i8)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    } else {
        None
    };
    println!("Product Revision: {:?}", product_revision);

    let bus_type = match descriptor.BusType {
        STORAGE_BUS_TYPE(0x01) => "SCSI",
        STORAGE_BUS_TYPE(0x02) => "ATAPI",
        STORAGE_BUS_TYPE(0x03) => "ATA",
        STORAGE_BUS_TYPE(0x04) => "1394",
        STORAGE_BUS_TYPE(0x05) => "SSA",
        STORAGE_BUS_TYPE(0x06) => "Fibre",
        STORAGE_BUS_TYPE(0x07) => "USB",
        STORAGE_BUS_TYPE(0x08) => "RAID",
        STORAGE_BUS_TYPE(0x09) => "iSCSI",
        STORAGE_BUS_TYPE(0x0A) => "SAS",
        STORAGE_BUS_TYPE(0x0B) => "SATA",
        STORAGE_BUS_TYPE(0x0C) => "SD",
        STORAGE_BUS_TYPE(0x0D) => "MMC",
        STORAGE_BUS_TYPE(0x0E) => "VIRTUAL",
        STORAGE_BUS_TYPE(0x0F) => "FileBackedVirtual",
        STORAGE_BUS_TYPE(0x10) => "Spaces",
        STORAGE_BUS_TYPE(0x11) => "NVMe",
        STORAGE_BUS_TYPE(0x12) => "SCM",
        STORAGE_BUS_TYPE(0x7F) => "BusTypeMaxReserved",
        _ => "UNKNOWN",
    };
    println!("Bus Type: {}", bus_type);

    println!("Descriptor: {:?}", descriptor);
    println!("Bytes Returned: {}", bytes_returned);

    Ok(())
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

        let disk_handle = disk_handle_result?;

        let mut disk_geometry: DISK_GEOMETRY = Default::default();
        let mut bytes_returned: u32 = 0;

        unsafe {
            DeviceIoControl(
                disk_handle,
                IOCTL_DISK_GET_DRIVE_GEOMETRY,
                None,
                0,
                Some(&mut disk_geometry as *mut _ as *mut std::ffi::c_void),
                size_of::<DISK_GEOMETRY>() as u32,
                Some(&mut bytes_returned),
                None,
            )?;
        };

        println!("Disk Geometry: {:?}", disk_geometry);

        // Get drive layout
        let mut layout = vec![
            0u8;
            size_of::<DRIVE_LAYOUT_INFORMATION_EX>()
                + size_of::<PARTITION_INFORMATION_EX>() * 127
        ];
        let mut bytes_returned = 0;

        unsafe {
            DeviceIoControl(
                disk_handle,
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

        println!(
            "\nDisk {} ({} partitions)",
            disk_number, layout.PartitionCount
        );
        println!(
            "Partition Style: {}",
            match PARTITION_STYLE(layout.PartitionStyle as i32) {
                PARTITION_STYLE_MBR => "MBR",
                PARTITION_STYLE_GPT => "GPT",
                _ => "RAW/Unknown",
            }
        );

        // Get partitions
        let partitions = unsafe {
            std::slice::from_raw_parts(
                layout.PartitionEntry.as_ptr(),
                layout.PartitionCount as usize,
            )
        };

        for partition in partitions {
            println!("\nPartition {}", partition.PartitionNumber);
            println!("  Starting Offset: {} bytes", partition.StartingOffset);
            println!("  Size: {} bytes", partition.PartitionLength);

            match partition.PartitionStyle {
                PARTITION_STYLE_MBR => {
                    let mbr = unsafe { partition.Anonymous.Mbr };
                    println!("  Type: MBR");
                    println!("  Partition Type: 0x{:x}", mbr.PartitionType);
                    println!("  Bootable: {}", mbr.BootIndicator);
                }
                PARTITION_STYLE_GPT => {
                    let gpt = unsafe { partition.Anonymous.Gpt };
                    println!("  Type: GPT");
                    println!("  Partition Type: {:?}", gpt.PartitionType);
                    println!("  Partition ID: {:?}", gpt.PartitionId);
                    let name = String::from_utf16_lossy(&gpt.Name);
                    println!("  Name: {}", name.trim_end_matches('\0'));
                }
                _ => println!("  Unknown Partition Type"),
            }
        }

        // Get disk identification data
        get_disk_identification_data(disk_handle)?;

        unsafe { CloseHandle(disk_handle) }?;
        disk_number += 1;
    }

    Ok(disks)
}
