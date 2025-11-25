// SPDX-License-Identifier: GPL-3.0-or-later
mod driver;
mod filesystem;
mod hardware;

pub fn storage_devices() -> Result<Vec<crate::core::Disk>, anyhow::Error> {
    let mut disks = hardware::get_disks()?;
    let volumes = filesystem::get_volumes()?;

    driver::link_volumes_to_partitions(&mut disks, &volumes)?;

    Ok(disks)
}
