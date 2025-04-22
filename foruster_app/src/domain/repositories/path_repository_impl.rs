// SPDX-License-Identifier: GPL-3.0-or-later
use crate::domain;
use std::{cell::RefCell, rc::Rc};

use super::traits::{DiskRepository, PathRepository};

#[derive(Clone)]
pub struct PathRepositoryImpl {
    paths: Rc<RefCell<Vec<domain::models::PathItem>>>,
    disk_repo: Rc<dyn DiskRepository>
}

impl PathRepositoryImpl {
    pub fn new(disk_repo: Rc<dyn DiskRepository>) -> Self {
        Self {
            paths: Rc::new(RefCell::new(vec![])),
            disk_repo,
        }
    }
}

impl PathRepository for PathRepositoryImpl {
    fn get_path(&self, index: usize) -> Option<domain::PathItem> {
        self.paths.borrow().get(index).cloned()
    }

    fn add_path(&self, path: std::path::PathBuf) {
        todo!()
    }

    fn remove_path(&self, index: usize) {
        todo!()
    }

    fn update_path(&self, index: usize, path: std::path::PathBuf) {
        todo!()
    }

    fn path_count(&self) -> usize {
        self.paths.borrow().len()
    }
    
    fn load_paths(&self) {
        let mut paths = self.paths.borrow_mut();

        for disk_index in 0..self.disk_repo.disk_count() {
            if let Some(disk) = self.disk_repo.get_disk(disk_index) {
                for partition in disk.disk_data().partitions() {
                    if let Some(volume) = partition.volume() {
                        for mount_point in volume.mount_points() {
                            paths.push(domain::PathItem::new(mount_point.to_path_buf(), volume.guid_identifier()));
                        }

                        for drive_letter in volume.drive_letters() {
                            let drive_letter_path = format!("{}:\\", drive_letter);
                            paths.push(domain::PathItem::new(drive_letter_path.into(), volume.guid_identifier()));
                        }
                    }
                }
            }
        }
    }
}
