// SPDX-License-Identifier: GPL-3.0-or-later
use super::utils::format_size;
use super::{IdentificationData, Partition};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Disk {
    #[cfg(windows)]
    id: u32,
    #[cfg(target_os = "linux")]
    id: String,
    identification_data: IdentificationData,
    partitions: Vec<Partition>,
    total_size: u64,
}

impl Disk {
    #[cfg(windows)]
    pub fn new(
        id: u32,
        identification_data: IdentificationData,
        partitions: Vec<Partition>,
        total_size: u64,
    ) -> Self {
        Self {
            id,
            identification_data,
            partitions,
            total_size,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn new(
        id: String,
        identification_data: IdentificationData,
        partitions: Vec<Partition>,
        total_size: u64,
    ) -> Self {
        Self {
            id,
            identification_data,
            partitions,
            total_size,
        }
    }

    #[cfg(windows)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[cfg(target_os = "linux")]
    pub fn id(&self) -> &String {
        &self.id
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

    pub fn set_partitions(&mut self, partitions: Vec<Partition>) {
        self.partitions = partitions;
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Disk identifier: {}", self.id)?;
        writeln!(f, "{}", self.identification_data)?;

        writeln!(f, "Partitions:")?;
        for partition in self.partitions.iter() {
            writeln!(f, "{}", partition)?;
        }

        writeln!(f, "Total size: {}", self.total_size_str())?;

        Ok(())
    }
}
