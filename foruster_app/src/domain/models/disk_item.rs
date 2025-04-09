// SPDX-License-Identifier: GPL-3.0-or-later
use foruster_core::Disk;

#[derive(Debug, Clone)]
pub struct DiskItem {
    disk_data: Disk,

    selected: bool,
}

impl DiskItem {
    pub fn new(disk_data: Disk) -> Self {
        Self {
            disk_data,
            selected: false,
        }
    }

    pub fn disk_data(&self) -> &Disk {
        &self.disk_data
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }
}
