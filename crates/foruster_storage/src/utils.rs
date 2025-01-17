// SPDX-License-Identifier: GPL-3.0-or-later
use windows::core::PCWSTR;

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
    valid_data.split(|&c| c == 0)
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