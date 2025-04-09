// SPDX-License-Identifier: GPL-3.0-or-later
use crate::domain;

pub trait DiskRepository {
    fn update_disks(&self);
    fn disk_count(&self) -> usize;
    fn selected_disk_count(&self) -> usize;
    fn get_disk(&self, index: usize) -> Option<domain::models::DiskItem>;
    fn toggle_selected(&self, index: usize) -> bool;
    fn remove_disk(&self, index: usize) -> bool;
    fn push_disk(&self, disk: domain::models::DiskItem) -> bool;
}
