// SPDX-License-Identifier: GPL-3.0-or-later
use foruster_core::Disk;

use crate::{
    link_layers::link_volume_to_partition, logical_layer_extractor::enumerate_volumes, physical_layer_extractor::enumerate_disks
};

pub fn storage_extractor() -> Result<Vec<Disk>, windows::core::Error> {
    println!("storage_extractor");

    let volumes = enumerate_volumes()?;

    let mut disks = enumerate_disks()?;

    link_volume_to_partition(&volumes, &mut disks)?;

    Ok(disks)
}
