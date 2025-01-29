// SPDX-License-Identifier: GPL-3.0-or-later
use crate::{identification_data::IdentificationData, partition::Partition};

#[derive(Debug)]
pub struct Disk {
    identification_data: IdentificationData,
    partitions: Vec<Partition>,
}

impl Disk {
    pub fn new(
        identification_data: IdentificationData,
        partitions: Vec<Partition>,
    ) -> Self {
        Self {
            identification_data,
            partitions,
        }
    }

    pub fn identification_data(&self) -> &IdentificationData {
        &self.identification_data
    }

    pub fn partitions(&self) -> &[Partition] {
        &self.partitions
    }
}
