// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

use crate::{identification_data::IdentificationData, partition::Partition};

#[derive(Debug)]
pub struct Disk {
    number: u32,
    identification_data: IdentificationData,
    partitions: Vec<Partition>,
}

impl Disk {
    pub fn new(
        number: u32,
        identification_data: IdentificationData,
        partitions: Vec<Partition>,
    ) -> Self {
        Self {
            number,
            identification_data,
            partitions,
        }
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn identification_data(&self) -> &IdentificationData {
        &self.identification_data
    }

    pub fn partitions(&self) -> &[Partition] {
        &self.partitions
    }

    pub fn partitions_mut(&mut self) -> &mut Vec<Partition> {
        &mut self.partitions
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Disk number: {}", self.number)?;
        writeln!(f, "{}", self.identification_data)?;

        writeln!(f, "Partitions:")?;
        for partition in self.partitions.iter() {
            writeln!(f, "{}", partition)?;
        }

        Ok(())
    }
}