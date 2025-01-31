// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

use crate::Volume;

#[derive(Debug)]
pub enum GPTPartitionAttribute {
    LegacyBiosBootable,
    NoBlockIoProtocol,
    PlatformRequired,
    BasicDataDax,
    BasicDataHidden,
    BasicDataNoDriveLetter,
    BasicDataOffline,
    BasicDataReadOnly,
    BasicDataService,
    BasicDataShadowCopy,
    SpacesNoMetadata,
}

#[derive(Debug)]
pub enum PartitionType {
    MBR {
        bootable: bool,
        partition_type: u8,
    },
    GPT {
        partition_guid: String,
        partition_name: String,
        attributes: Vec<GPTPartitionAttribute>,
    },
}

#[derive(Debug)]
pub struct Partition {
    number: u32,
    starting_offset: u64,
    size: u64,
    volume: Option<Volume>,
    partition_type: PartitionType,
}

impl Partition {
    pub fn new(
        number: u32,
        starting_offset: u64,
        size: u64,
        volume: Option<Volume>,
        partition_type: PartitionType,
    ) -> Self {
        Self {
            number,
            starting_offset,
            size,
            volume,
            partition_type,
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

    pub fn partition_type(&self) -> &PartitionType {
        &self.partition_type
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

        match &self.partition_type {
            PartitionType::MBR {
                bootable,
                partition_type,
            } => {
                writeln!(f, "\tMBR partition")?;
                writeln!(f, "\t\tBootable: {}", bootable)?;
                writeln!(f, "\t\tPartition type: {}", partition_type)?;
            }
            PartitionType::GPT {
                partition_guid,
                partition_name,
                attributes,
            } => {
                writeln!(f, "\tGPT partition")?;
                writeln!(f, "\t\tPartition GUID: {}", partition_guid)?;

                if !partition_name.is_empty() {
                    writeln!(f, "\t\tPartition name: {}", partition_name)?;
                }
                
                if !attributes.is_empty() {
                    writeln!(f, "\t\tAttributes:")?;
                    for attribute in attributes {
                        writeln!(f, "\t\t\t{:?}", attribute)?;
                    }
                }
            }
        }

        Ok(())
    }
}