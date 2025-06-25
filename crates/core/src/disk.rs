// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

use crate::{format_size, identification_data::IdentificationData, partition::Partition};

#[derive(Debug, Clone)]
pub struct Disk {
    number: u32,
    identification_data: IdentificationData,
    partitions: Vec<Partition>,
    total_size: u64,
}

impl Disk {
    pub fn new(
        number: u32,
        identification_data: IdentificationData,
        partitions: Vec<Partition>,
        total_size: u64,
    ) -> Self {
        Self {
            number,
            identification_data,
            partitions,
            total_size,
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

    pub fn total_size(&self) -> u64 {
        self.total_size
    }

    pub fn partitions_mut(&mut self) -> &mut Vec<Partition> {
        &mut self.partitions
    }

    pub fn name(&self) -> String {
        match (
            self.identification_data.vendor(),
            self.identification_data.model(),
        ) {
            (Some(vendor), Some(model)) => format!("{} {}", vendor, model),
            (Some(vendor), None) => vendor.clone(),
            (None, Some(model)) => model.clone(),
            (None, None) => "Unknown".to_string(),
        }
    }

    pub fn total_size_str(&self) -> String {
        format_size(self.total_size as usize)
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
