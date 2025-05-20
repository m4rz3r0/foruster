// SPDX-License-Identifier: GPL-3.0-or-later
use crate::domain::{self, DiskItem};
use foruster_storage::{device_event_listener::DeviceEventListener, storage_extractor};
use std::{cell::RefCell, path::Path, rc::Rc};

use super::traits::DiskRepository;

#[derive(Clone)]
pub struct DiskRepositoryImpl {
    disks: Rc<RefCell<Vec<DiskItem>>>,
    event_listener: Rc<RefCell<DeviceEventListener>>,
}

impl DiskRepositoryImpl {
    pub fn new() -> Self {
        Self {
            disks: Rc::new(RefCell::new(vec![])),
            event_listener: Rc::new(RefCell::new(DeviceEventListener::new())),
        }
    }
}

impl Default for DiskRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl DiskRepository for DiskRepositoryImpl {
    fn disk_count(&self) -> usize {
        self.disks.borrow().len()
    }

    fn get_disk(&self, index: usize) -> Option<DiskItem> {
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

    fn push_disk(&self, disk: DiskItem) -> bool {
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
        for disk in new_disks.into_iter() {
            if !old_disks
                .iter()
                .any(|old_disk| old_disk.disk_data().name() == disk.name())
            {
                old_disks.push(domain::DiskItem::new(disk));
            }
        }
    }

    fn selected_disk_count(&self) -> usize {
        self.disks
            .borrow()
            .iter()
            .filter(|disk| disk.selected())
            .count()
    }

    fn check_for_device_changes(&self) -> bool {
        let has_changes = self.event_listener.borrow_mut().poll_event().is_some();

        if has_changes {
            self.update_disks();
        }

        has_changes
    }

    fn volume_id_by_path(&self, path: &Path) -> Option<String> {
        let volumes = self
            .disks
            .borrow()
            .iter()
            .flat_map(|disk| disk.disk_data().partitions())
            .filter_map(|partition| partition.volume().clone())
            .collect::<Vec<_>>();

        for volume in volumes {
            for mount_point in volume.mount_points() {
                if path.starts_with(mount_point) {
                    return Some(volume.guid_identifier());
                }
            }

            for drive_letter in volume.drive_letters() {
                if path.starts_with(drive_letter.to_string()) {
                    return Some(volume.guid_identifier());
                }
            }
        }

        None
    }
}
