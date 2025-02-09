// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use super::traits;
use crate::mvc;

#[derive(Clone)]
pub struct MockDiskRepository {
    disks: Rc<RefCell<Vec<mvc::models::DiskModel>>>,
}

impl MockDiskRepository {
    pub fn new(disks: Vec<mvc::models::DiskModel>) -> Self {
        Self { disks: Rc::new(RefCell::new(disks)) }
    }
}

impl traits::DiskRepository for MockDiskRepository {
    fn disk_count(&self) -> usize {
        self.disks.borrow().len()
    }

    fn get_disk(&self, index: usize) -> Option<mvc::models::DiskModel> {
        self.disks.borrow().get(index).cloned()
    }

    fn toggle_checked(&self, index: usize) -> bool {
        if let Some(disk) = self.disks.borrow_mut().get_mut(index) {
            disk.toggle_checked();
            return true;
        }

        false
    }

    fn remove_disk(&self, index: usize) -> bool {
        if index < self.disks.borrow().len() {
            self.disks.borrow_mut().remove(index);
            return true;
        }

        false
    }

    fn push_disk(&self, disk: mvc::models::DiskModel) -> bool {
        self.disks.borrow_mut().push(disk);
        true
    }
}