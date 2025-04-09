// SPDX-License-Identifier: GPL-3.0-or-later
use std::collections::HashMap;

use windows::{
    core::{Error, HSTRING},
    Win32::{
        Storage::FileSystem::{
            CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS,
            OPEN_EXISTING,
        },
        System::{
            Ioctl::{DISK_EXTENT, VOLUME_DISK_EXTENTS},
            IO::DeviceIoControl,
        },
    },
};

use foruster_core::{Disk, Volume};

fn get_volume_extents(volume_guid: &str) -> Result<VOLUME_DISK_EXTENTS, windows::core::Error> {
    let hstring_volume_guid = HSTRING::from(volume_guid);

    // Open the volume handle
    let volume_handle = unsafe {
        CreateFileW(
            &hstring_volume_guid,
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE, // Allow shared access
            None,
            OPEN_EXISTING,
            windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        )
    }?;

    // Get initial volume disk extents to get count
    let mut initial_extents: VOLUME_DISK_EXTENTS = Default::default();
    let mut bytes_returned: u32 = 0;

    unsafe {
        DeviceIoControl(
            volume_handle,
            IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS,
            None,
            0,
            Some(&mut initial_extents as *mut _ as *mut std::ffi::c_void),
            std::mem::size_of::<VOLUME_DISK_EXTENTS>() as u32,
            Some(&mut bytes_returned),
            None,
        )
    }?;

    // Calculate required size and allocate buffer
    let extent_count = initial_extents.NumberOfDiskExtents;
    let buffer_size = std::mem::size_of::<VOLUME_DISK_EXTENTS>()
        + (extent_count as usize - 1) * std::mem::size_of::<DISK_EXTENT>();
    let mut buffer = vec![0u8; buffer_size];

    // Get all extents
    unsafe {
        DeviceIoControl(
            volume_handle,
            IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS,
            None,
            0,
            Some(buffer.as_mut_ptr() as *mut std::ffi::c_void),
            buffer_size as u32,
            Some(&mut bytes_returned),
            None,
        )
    }?;

    let extents = unsafe { &*(buffer.as_ptr() as *const VOLUME_DISK_EXTENTS) };
    // Ahora extents.Extents contiene todos los disk extents

    Ok(*extents)
}

pub fn link_volume_to_partition(volumes: &[Volume], disks: &mut [Disk]) -> Result<(), Error> {
    let mut disk_map: HashMap<u32, &mut Disk> = disks.iter_mut().map(|d| (d.number(), d)).collect();

    for volume in volumes {
        let volume_guid = volume.guid();

        if volume_guid.is_empty() {
            continue;
        }

        let guid_path = &volume_guid[..volume_guid.len() - 1];
        let volume_extents = get_volume_extents(guid_path)?;

        for extent in volume_extents.Extents.iter() {
            let disk_number = extent.DiskNumber;
            let starting_offset = extent.StartingOffset;

            if let Some(disk) = disk_map.get_mut(&disk_number) {
                let partitions = disk.partitions_mut();

                for partition in partitions {
                    if partition.starting_offset() == starting_offset as u64 {
                        partition.set_volume(Some(volume.clone()));
                    }
                }
            }
        }
    }

    Ok(())
}
