// SPDX-License-Identifier: GPL-3.0-or-later
use crate::core::Volume;
use windows::Win32::Foundation::HANDLE;
use windows::{
    Win32::{
        Foundation::MAX_PATH,
        Storage::FileSystem::{
            FindFirstVolumeW, FindNextVolumeW, GetDiskFreeSpaceExW, GetVolumeInformationW,
            GetVolumePathNamesForVolumeNameW,
        },
    },
    core::{Error, PCWSTR},
};

pub struct VolumeHandle(pub HANDLE);

impl Drop for VolumeHandle {
    fn drop(&mut self) {
        unsafe {
            windows::Win32::Storage::FileSystem::FindVolumeClose(self.0)
                .expect("Failed to close volume handle");
        }
    }
}

pub fn utf16_to_string(wide: &[u16]) -> Result<String, std::string::FromUtf16Error> {
    // Find the position of the first null terminator
    if let Some(pos) = wide.iter().position(|&c| c == 0) {
        // Slice up to the first null
        let slice = &wide[..pos];
        String::from_utf16(slice)
    } else {
        // If no null terminator is found, convert the entire slice
        String::from_utf16(wide)
    }
}

pub fn utf16_to_multiple_strings(wide: &[u16], length: usize) -> Vec<String> {
    // Take valid portion of buffer
    let valid_data = &wide[..length];

    // Split at single nulls and convert each part
    valid_data
        .split(|&c| c == 0)
        .filter(|s| !s.is_empty())
        .filter_map(|s| utf16_to_string(s).ok())
        .collect()
}

pub fn string_to_pcwstr(slice: &[u16]) -> PCWSTR {
    if slice.is_empty() {
        return PCWSTR::null();
    }

    // Check if slice already ends with null terminator
    if slice[slice.len() - 1] == 0 {
        PCWSTR::from_raw(slice.as_ptr())
    } else {
        // If no null terminator, return null
        // Note: In production code, you might want to create a new null-terminated buffer
        PCWSTR::null()
    }
}

const BUFFER_SIZE: usize = MAX_PATH as usize;

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
        return Err(Error::from_thread());
    }

    let volume_handle = VolumeHandle(handle);

    loop {
        let guid_string = utf16_to_string(&volume_guid[..volume_guid.len() - 1])
            .map_err(|_| Error::from_thread())?;

        let volume_guid_pcwstr = string_to_pcwstr(&volume_guid);

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
