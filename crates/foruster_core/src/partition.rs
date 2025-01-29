// SPDX-License-Identifier: GPL-3.0-or-later
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
    volume_guid: Option<String>,
    partition_type: PartitionType,
}

impl Partition {
    pub fn new(
        number: u32,
        starting_offset: u64,
        size: u64,
        volume_guid: Option<String>,
        partition_type: PartitionType,
    ) -> Self {
        Self {
            number,
            starting_offset,
            size,
            volume_guid,
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

    pub fn volume_guid(&self) -> &Option<String> {
        &self.volume_guid
    }
}
