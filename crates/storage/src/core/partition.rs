// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

use super::volume::Volume;

#[derive(Debug, Clone)]
pub struct Partition {
    number: u32,
    starting_offset: u64,
    size: u64,
    volume: Option<Volume>,
}

impl Partition {
    pub fn new(number: u32, starting_offset: u64, size: u64, volume: Option<Volume>) -> Self {
        Self {
            number,
            starting_offset,
            size,
            volume,
        }
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn starting_offset(&self) -> u64 {
        self.starting_offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn volume(&self) -> &Option<Volume> {
        &self.volume
    }

    pub fn set_volume(&mut self, volume: Option<Volume>) {
        self.volume = volume;
    }
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\tPartition number: {}", self.number)?;
        writeln!(f, "\tStarting offset: {}", self.starting_offset)?;
        writeln!(f, "\tSize: {}", self.size)?;

        if let Some(volume) = &self.volume {
            writeln!(f, "\tVolume: {}", volume)?;
        }

        Ok(())
    }
}
