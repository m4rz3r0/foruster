// SPDX-License-Identifier: GPL-3.0-or-later
use crate::domain::PathItem;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use super::traits::{DiskRepository, PathRepository};

#[derive(Clone)]
pub struct PathRepositoryImpl {
    paths: Rc<RefCell<Vec<PathItem>>>,
    disk_repo: Rc<dyn DiskRepository>,
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
    fn get_path(&self, index: usize) -> Option<PathItem> {
        self.paths.borrow().get(index).cloned()
    }

    fn add_path(&self, path: std::path::PathBuf) {
        if path.as_os_str().is_empty() {
            return;
        }

        let mut paths = self.paths.borrow_mut();
        let volume_id = self
            .disk_repo
            .volume_id_by_path(&path)
            .unwrap_or("No Volume".to_string());

        paths.push(PathItem::new(path, volume_id));
    }

    fn remove_path(&self, index: usize) {
        let mut paths = self.paths.borrow_mut();
        if index < paths.len() {
            paths.remove(index);
        }
    }

    fn update_path(&self, index: usize, path: std::path::PathBuf) {
        if path.as_os_str().is_empty() {
            return;
        }

        let mut paths = self.paths.borrow_mut();
        let volume_id = self
            .disk_repo
            .volume_id_by_path(&path)
            .unwrap_or("No Volume".to_string());

        match paths.get_mut(index) {
            Some(path_item) => {
                path_item.set_path(path);
                path_item.set_volume_id(volume_id);
            }
            None => {
                paths.push(PathItem::new(path, volume_id));
            }
        }
    }

    fn path_count(&self) -> usize {
        self.paths.borrow().len()
    }

    fn redundant_count(&self) -> usize {
        self.paths
            .borrow()
            .iter()
            .filter(|path| path.is_redundant())
            .count()
    }

    fn load_paths(&self) {
        let mut paths = self.paths.borrow_mut();

        for disk_index in 0..self.disk_repo.disk_count() {
            if let Some(disk) = self.disk_repo.get_disk(disk_index) {
                if disk.selected() {
                    let disk_paths = disk
                        .disk_data()
                        .partitions()
                        .iter()
                        .flat_map(|partition| partition.volume())
                        .flat_map(|volume| {
                            let mut vec = volume
                                .drive_letters()
                                .iter()
                                .map(|drive_letter| format!("{}:\\", drive_letter).into())
                                .collect::<Vec<PathBuf>>();
                            vec.extend_from_slice(volume.mount_points());
                            vec
                        })
                        .map(|mount_point| PathItem::new(mount_point, disk.disk_data().name()))
                        .collect::<Vec<_>>();

                    paths.extend(disk_paths);
                }
            }
        }
    }

    fn check_redundant_paths(&self) -> Vec<usize> {
        let mut paths = self.paths.borrow_mut();
        let mut redundant_changes = Vec::new();

        for i in 0..paths.len() {
            let mut redundant = false;
            let previous_redundant = paths[i].is_redundant();
            for j in (0..paths.len()).filter(|&j| j != i) {
                if paths[i].path().starts_with(paths[j].path()) {
                    let redundant_message = format!(
                        "La ruta es un subdirectorio de {}",
                        paths[j].path().display()
                    );
                    paths[i].set_redundant(true);
                    paths[i].set_redundancy_message(redundant_message);

                    redundant = true;
                }
            }
            if !redundant {
                paths[i].set_redundant(false);
                paths[i].set_redundancy_message(String::new());
            }

            if paths[i].is_redundant() != previous_redundant {
                redundant_changes.push(i);
            }
        }

        redundant_changes
    }
}
