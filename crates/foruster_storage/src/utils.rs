// SPDX-License-Identifier: GPL-3.0-or-later
use foruster_core::GPTPartitionAttribute;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HANDLE,
        System::Ioctl::{
            DRIVE_LAYOUT_INFORMATION_EX, GPT_ATTRIBUTES, GPT_ATTRIBUTE_LEGACY_BIOS_BOOTABLE,
            GPT_ATTRIBUTE_NO_BLOCK_IO_PROTOCOL, GPT_ATTRIBUTE_PLATFORM_REQUIRED,
            GPT_BASIC_DATA_ATTRIBUTE_DAX, GPT_BASIC_DATA_ATTRIBUTE_HIDDEN,
            GPT_BASIC_DATA_ATTRIBUTE_NO_DRIVE_LETTER, GPT_BASIC_DATA_ATTRIBUTE_OFFLINE,
            GPT_BASIC_DATA_ATTRIBUTE_READ_ONLY, GPT_BASIC_DATA_ATTRIBUTE_SERVICE,
            GPT_BASIC_DATA_ATTRIBUTE_SHADOW_COPY, GPT_SPACES_ATTRIBUTE_NO_METADATA,
            PARTITION_INFORMATION_EX,
        },
    },
};

pub struct SafeHandle(pub HANDLE);

impl Drop for SafeHandle {
    fn drop(&mut self) {
        unsafe { windows::Win32::Foundation::CloseHandle(self.0).expect("Failed to close handle") };
    }
}

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

pub fn safe_layout_cast(buffer: &[u8]) -> Option<&DRIVE_LAYOUT_INFORMATION_EX> {
    // Verificar tamaño mínimo y alineación
    if buffer.len() < size_of::<DRIVE_LAYOUT_INFORMATION_EX>() {
        return None;
    }

    // Verificar alineación del buffer
    let align = align_of::<DRIVE_LAYOUT_INFORMATION_EX>();
    if (buffer.as_ptr() as usize) % align != 0 {
        return None;
    }

    // Verificar PartitionCount no excede el buffer
    let layout = unsafe { &*(buffer.as_ptr() as *const DRIVE_LAYOUT_INFORMATION_EX) };
    let required_size = size_of::<DRIVE_LAYOUT_INFORMATION_EX>()
        + size_of::<PARTITION_INFORMATION_EX>() * layout.PartitionCount as usize;

    if buffer.len() < required_size {
        return None;
    }

    Some(layout)
}

pub unsafe fn extract_string(raw_data: &[u8], offset: u32) -> Option<String> {
    if offset == 0 {
        return None;
    }

    let ptr = raw_data.as_ptr().add(offset as usize);
    Some(
        std::ffi::CStr::from_ptr(ptr as *const i8)
            .to_string_lossy()
            .into_owned(),
    )
}

pub fn generate_gpt_attributes_vec(attributes: &GPT_ATTRIBUTES) -> Vec<GPTPartitionAttribute> {
    [
        (
            GPT_ATTRIBUTE_LEGACY_BIOS_BOOTABLE,
            GPTPartitionAttribute::LegacyBiosBootable,
        ),
        (
            GPT_ATTRIBUTE_NO_BLOCK_IO_PROTOCOL,
            GPTPartitionAttribute::NoBlockIoProtocol,
        ),
        (
            GPT_ATTRIBUTE_PLATFORM_REQUIRED.0,
            GPTPartitionAttribute::PlatformRequired,
        ),
        (
            GPT_BASIC_DATA_ATTRIBUTE_DAX,
            GPTPartitionAttribute::BasicDataDax,
        ),
        (
            GPT_BASIC_DATA_ATTRIBUTE_HIDDEN.0,
            GPTPartitionAttribute::BasicDataHidden,
        ),
        (
            GPT_BASIC_DATA_ATTRIBUTE_NO_DRIVE_LETTER.0,
            GPTPartitionAttribute::BasicDataNoDriveLetter,
        ),
        (
            GPT_BASIC_DATA_ATTRIBUTE_OFFLINE,
            GPTPartitionAttribute::BasicDataOffline,
        ),
        (
            GPT_BASIC_DATA_ATTRIBUTE_READ_ONLY.0,
            GPTPartitionAttribute::BasicDataReadOnly,
        ),
        (
            GPT_BASIC_DATA_ATTRIBUTE_SERVICE,
            GPTPartitionAttribute::BasicDataService,
        ),
        (
            GPT_BASIC_DATA_ATTRIBUTE_SHADOW_COPY.0,
            GPTPartitionAttribute::BasicDataShadowCopy,
        ),
        (
            GPT_SPACES_ATTRIBUTE_NO_METADATA,
            GPTPartitionAttribute::SpacesNoMetadata,
        ),
    ]
    .into_iter()
    .filter(|(flag, _)| attributes.contains(GPT_ATTRIBUTES(*flag)))
    .map(|(_, attr)| attr)
    .collect()
}
