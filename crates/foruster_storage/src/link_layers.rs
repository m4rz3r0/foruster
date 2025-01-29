// SPDX-License-Identifier: GPL-3.0-or-later
use windows::{core::HSTRING, Win32::{Storage::FileSystem::{CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS, OPEN_EXISTING}, System::{Ioctl::VOLUME_DISK_EXTENTS, IO::DeviceIoControl}}};

use foruster_core::Partition;

pub fn get_volume_partitions(volume_guid: &str) -> Result<Vec<Partition>, windows::core::Error> {
    let mut partitions = Vec::new();

    println!("Volume GUID: {}", volume_guid);

    let hstring_volume_guid = HSTRING::from(volume_guid);

    // Open the volume handle
    let volume_handle = unsafe {
        CreateFileW(
            &hstring_volume_guid,
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,  // Allow shared access
            None,
            OPEN_EXISTING,
            windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        )
    }?;

    println!("Volume Handle: {:?}", volume_handle);

    // Get volume disk extents
    let mut disk_extents: VOLUME_DISK_EXTENTS = Default::default();
    let mut bytes_returned: u32 = 0;
    
    unsafe {
        DeviceIoControl(
            volume_handle,
            IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS,
            None,
            0,
            Some(&mut disk_extents as *mut _ as *mut std::ffi::c_void),
            std::mem::size_of::<VOLUME_DISK_EXTENTS>() as u32,
            Some(&mut bytes_returned),
            None,
        )
    }?;

    println!("Extents: {:?}", disk_extents);

    for extent in disk_extents.Extents.iter() {
        let starting_offset = extent.StartingOffset;
        let size = extent.ExtentLength;
        let volume_guid = volume_guid.to_string();

        // TODO: Fix this
        //partitions.push(Partition::new(starting_offset, size, volume_guid));
    }

    Ok(partitions)
}