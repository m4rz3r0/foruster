// SPDX-License-Identifier: GPL-3.0-or-later
use foruster_core::Disk;

#[derive(Debug, Clone)]
pub struct DiskModel {
    disk_data: Disk,

    checked: bool,
}

impl DiskModel {
    pub fn new(disk_data: Disk) -> Self {
        Self {
            disk_data,
            checked: false,
        }
    }

    pub fn disk_data(&self) -> &Disk {
        &self.disk_data
    }

    pub fn checked(&self) -> bool {
        self.checked
    }

    pub fn toggle_checked(&mut self) {
        self.checked = !self.checked;
    }
}