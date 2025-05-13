// SPDX-License-Identifier: GPL-3.0-or-later
use crate::domain::DiskItem;

pub trait DiskRepository {
    fn update_disks(&self);
    fn check_for_device_changes(&self) -> bool;
    fn disk_count(&self) -> usize;
    fn selected_disk_count(&self) -> usize;
    fn get_disk(&self, index: usize) -> Option<DiskItem>;
    fn toggle_selected(&self, index: usize) -> bool;
    fn remove_disk(&self, index: usize) -> bool;
    fn push_disk(&self, disk: DiskItem) -> bool;
    fn volume_id_by_path(&self, path: &std::path::Path) -> Option<String>;
}
