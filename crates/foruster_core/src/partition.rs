// SPDX-License-Identifier: GPL-3.0-or-later
#[derive(Debug)]
pub struct Partition {
    starting_offset: i64,
    size: i64,
    volume_guid: String,
}

impl Partition {
    pub fn new(
        starting_offset: i64,
        size: i64,
        volume_guid: String,
    ) -> Self {
        Self {
            starting_offset,
            size,
            volume_guid,
        }
    }

    pub fn starting_offset(&self) -> i64 {
        self.starting_offset
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn volume_guid(&self) -> &str {
        &self.volume_guid
    }
}
