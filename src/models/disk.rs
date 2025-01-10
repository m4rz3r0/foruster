// SPDX-License-Identifier: GPL-3.0-or-later
use serde::{Deserialize, Serialize};

use crate::Partition;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiskKind {
    HDD,
    SSD,
    SCM,
    #[default]
    Unknown,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    device_name: String,
    model: String,
    serial: String,
    kind: DiskKind,
    size: usize,
    removable: bool,
    partitions: Vec<Partition>,
}

impl Disk {
    pub fn new(
        device_name: String,
        model: String,
        serial: String,
        kind: DiskKind,
        size: usize,
        removable: bool,
        partitions: Vec<Partition>,
    ) -> Disk {
        Disk {
            device_name,
            model,
            serial,
            kind,
            size,
            removable,
            partitions,
        }
    }

    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn serial(&self) -> &str {
        &self.serial
    }

    pub fn kind(&self) -> &DiskKind {
        &self.kind
    }

    pub fn removable(&self) -> bool {
        self.removable
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn partitions(&self) -> &[Partition] {
        &self.partitions
    }
}
