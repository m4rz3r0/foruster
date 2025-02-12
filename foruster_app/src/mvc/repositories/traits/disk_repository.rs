// SPDX-License-Identifier: GPL-3.0-or-later
use crate::mvc;

pub trait DiskRepository {
    fn update_disks(&self);
    fn disk_count(&self) -> usize;
    fn checked_disk_count(&self) -> usize;
    fn get_disk(&self, index: usize) -> Option<mvc::models::DiskModel>;
    fn toggle_checked(&self, index: usize) -> bool;
    fn remove_disk(&self, index: usize) -> bool;
    fn push_disk(&self, disk: mvc::models::DiskModel) -> bool;
}