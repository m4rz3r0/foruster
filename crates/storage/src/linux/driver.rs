// SPDX-License-Identifier: GPL-3.0-or-later
use crate::core::{Disk, Volume};
use anyhow::Result;
use std::collections::HashMap;

pub fn link_volumes_to_partitions(
    disks: &mut [Disk],
    volumes: &HashMap<String, Volume>,
) -> Result<()> {
    for disk in disks.iter_mut() {
        for volume in volumes {
            for partition in disk.partitions_mut() {
                let vol_dev_num = if volume.0.starts_with("/dev/nvme") {
                    if let Some(p_pos) = volume.0.rfind('p') {
                        &volume.0[p_pos + 1..]
                    } else {
                        volume.0
                    }
                } else {
                    volume.0.trim_start_matches(char::is_alphabetic)
                }
                .parse::<u32>()
                .unwrap_or(0);

                if vol_dev_num == 0 {
                    continue;
                }

                if vol_dev_num == partition.number() {
                    partition.set_volume(Some(volume.1.clone()));
                }
            }
        }
    }

    Ok(())
}
