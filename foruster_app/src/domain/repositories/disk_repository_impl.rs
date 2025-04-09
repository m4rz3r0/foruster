// SPDX-License-Identifier: GPL-3.0-or-later
use std::{cell::RefCell, rc::Rc};

use foruster_storage::storage_extractor;

use super::traits;
use crate::domain;

#[derive(Clone)]
pub struct DiskRepositoryImpl {
    disks: Rc<RefCell<Vec<domain::models::DiskItem>>>,
}

impl DiskRepositoryImpl {
    pub fn new() -> Self {
        Self {
            disks: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl traits::DiskRepository for DiskRepositoryImpl {
    fn disk_count(&self) -> usize {
        self.disks.borrow().len()
    }

    fn get_disk(&self, index: usize) -> Option<domain::models::DiskItem> {
        self.disks.borrow().get(index).cloned()
    }

    fn toggle_selected(&self, index: usize) -> bool {
        if let Some(disk) = self.disks.borrow_mut().get_mut(index) {
            disk.toggle_selected();
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

    fn push_disk(&self, disk: domain::DiskItem) -> bool {
        self.disks.borrow_mut().push(disk);
        true
    }

    fn update_disks(&self) {
        let mut old_disks = self.disks.borrow_mut();

        let new_disks = if let Ok(disks) = storage_extractor() {
            disks
        } else {
            return;
        };

        // Remove disconnected disks
        old_disks.retain(|old_disk| {
            new_disks
                .iter()
                .any(|new_disk| old_disk.disk_data().name() == new_disk.name())
        });

        // Add or update connected disks
        for disk in new_disks.iter() {
            if !old_disks
                .iter()
                .any(|old_disk| old_disk.disk_data().name() == disk.name())
            {
                old_disks.push(domain::DiskItem::new(disk.clone()));
            }
        }
    }
    
    fn checked_disk_count(&self) -> usize {
        self.disks.borrow().iter().filter(|disk| disk.selected()).count()
    }
}
