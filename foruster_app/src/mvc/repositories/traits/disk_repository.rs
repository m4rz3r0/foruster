// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use crate::mvc;

pub trait DiskRepository {
    fn disk_count(&self) -> usize;
    fn get_disk(&self, index: usize) -> Option<mvc::models::DiskModel>;
    fn toggle_checked(&self, index: usize) -> bool;
    fn remove_disk(&self, index: usize) -> bool;
    fn push_disk(&self, disk: mvc::models::DiskModel) -> bool;
}