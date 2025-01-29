// SPDX-License-Identifier: GPL-3.0-or-later
use windows::{
    core::{Error, PCWSTR},
    Win32::{
        Foundation::{HANDLE, MAX_PATH},
        Storage::FileSystem::{
            FindFirstVolumeW, FindNextVolumeW, FindVolumeClose, GetDiskFreeSpaceExW,
            GetVolumeInformationW, GetVolumePathNamesForVolumeNameW,
        },
    },
};

use crate::utils::{string_to_pcwstr, utf16_to_multiple_strings, utf16_to_string};
use foruster_core::Volume;

const BUFFER_SIZE: usize = MAX_PATH as usize;

struct VolumeHandle(HANDLE);

impl Drop for VolumeHandle {
    fn drop(&mut self) {
        unsafe {
            FindVolumeClose(self.0).ok();
        }
    }
}

#[derive(Debug)]
struct VolumeInfo {
    filesystem: Option<String>,
    total_bytes: u64,
    free_bytes: u64,
    paths: Vec<String>,
}

fn get_volume_info(volume_guid_pcwstr: PCWSTR) -> Result<VolumeInfo, Error> {
    let mut filesystem_name = vec![0; BUFFER_SIZE];
    let mut total_bytes = 0;
    let mut free_bytes = 0;
    let mut path_names = vec![0; BUFFER_SIZE];
    let mut return_length = 0;

    // Get filesystem info
    let filesystem = unsafe {
        GetVolumeInformationW(
            volume_guid_pcwstr,
            None,
            None,
            None,
            None,
            Some(&mut filesystem_name),
        )
        .ok()
        .and_then(|_| utf16_to_string(&filesystem_name).ok())
    };

    // Get space info
    unsafe {
        GetDiskFreeSpaceExW(
            volume_guid_pcwstr,
            None,
            Some(&mut total_bytes),
            Some(&mut free_bytes),
        )
    }?;

    // Get path names
    unsafe {
        GetVolumePathNamesForVolumeNameW(
            volume_guid_pcwstr,
            Some(&mut path_names),
            &mut return_length,
        )
    }?;

    let paths = utf16_to_multiple_strings(&path_names, return_length as usize);

    Ok(VolumeInfo {
        filesystem,
        total_bytes,
        free_bytes,
        paths,
    })
}

pub fn enumerate_volumes() -> Result<Vec<Volume>, Error> {
    let mut volumes = Vec::new();
    let mut volume_guid = vec![0; BUFFER_SIZE];

    let handle = unsafe { FindFirstVolumeW(&mut volume_guid)? };
    if handle.is_invalid() {
        return Err(Error::from_win32());
    }

    let volume_handle = VolumeHandle(handle);

    loop {
        let guid_string = utf16_to_string(&volume_guid[..volume_guid.len() - 1])
            .map_err(|_| Error::from_win32())?;

        let volume_guid_pcwstr = string_to_pcwstr(&volume_guid);

        println!("Volume GUID: {:?}", volume_guid_pcwstr);

        match get_volume_info(volume_guid_pcwstr) {
            Ok(info) => {
                volumes.push(Volume::new(
                    guid_string,
                    info.filesystem,
                    info.total_bytes,
                    info.free_bytes,
                    info.paths,
                ));
            }
            Err(e) => {
                // Log error but continue with next volume
                eprintln!("Failed to get volume info: {}", e);
            }
        }

        unsafe {
            if FindNextVolumeW(volume_handle.0, &mut volume_guid).is_err() {
                break;
            }
        }
    }

    Ok(volumes)
}
