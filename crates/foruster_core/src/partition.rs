// SPDX-License-Identifier: GPL-3.0-or-later
use crate::volume::Volume;

enum PartitionType {
    Primary,
    Extended,
    Logical,
}

pub struct Partition {
    starting_offset: u64,
    size: u64,
    partition_type: PartitionType,
    volume: Volume,
}

impl Partition {
    pub fn new(
        starting_offset: u64,
        size: u64,
        partition_type: PartitionType,
        volume: Volume,
    ) -> Self {
        Self {
            starting_offset,
            size,
            partition_type,
            volume,
        }
    }

    pub fn starting_offset(&self) -> u64 {
        self.starting_offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn partition_type(&self) -> &PartitionType {
        &self.partition_type
    }

    pub fn volume(&self) -> &Volume {
        &self.volume
    }
}
