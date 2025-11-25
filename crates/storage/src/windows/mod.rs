// SPDX-License-Identifier: GPL-3.0-or-later
mod device_event_listener;
mod driver;
mod filesystem;
mod hardware;
pub use device_event_listener::DeviceEventListener;

pub fn storage_devices() -> Result<Vec<crate::core::Disk>, windows::core::Error> {
    let volumes = filesystem::enumerate_volumes()?;

    let mut disks = hardware::enumerate_disks()?;

    driver::link_volume_to_partition(&volumes, &mut disks)?;

    Ok(disks)
}
