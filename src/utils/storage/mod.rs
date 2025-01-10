// SPDX-License-Identifier: GPL-3.0-or-later
#[cfg(target_os = "linux")]
mod linux_storage_utils;
mod storage_utils;
#[cfg(target_os = "windows")]
mod windows_storage_utils;

#[cfg(target_os = "linux")]
pub use linux_storage_utils::{get_disks, get_mounts, is_mount, Mount};
pub use storage_utils::{bytes_to_mb, format_size, mb_to_bytes, read_magic_bytes};
#[cfg(target_os = "windows")]
pub use windows_storage_utils::get_disks;
