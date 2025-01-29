// SPDX-License-Identifier: GPL-3.0-or-later
use crate::{
    logical_layer_extractor::enumerate_volumes, physical_layer_extractor::enumerate_disks,
};

pub fn storage_extractor() -> Result<(), windows::core::Error> {
    println!("storage_extractor");

    let volumes = enumerate_volumes()?;

    println!("Volumes: {:?}", volumes);

    /*let mut partitions = vec![];
    for volume in volumes {
        let volume_id = volume.id();
        let volume_partitions = get_volume_partitions(&volume_id[..volume_id.len() - 1])?;

        partitions.push(volume_partitions);
    }

    println!("Partitions: {:?}", partitions);*/

    let disks = enumerate_disks()?;

    println!("Disks: {:?}", disks);

    Ok(())
}
