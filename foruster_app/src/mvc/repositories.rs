// SPDX-License-Identifier: GPL-3.0-or-later
mod mock_disk_repository;
use foruster_core::{Disk, IdentificationData};
use foruster_storage::storage_extractor;
pub use mock_disk_repository::*;

use crate::mvc::models::DiskModel;

pub mod traits;

pub fn disk_repo() -> impl traits::DiskRepository + Clone {
    MockDiskRepository::new(
        storage_extractor()
            .unwrap()
            .into_iter()
            .map(|d| DiskModel::new(d))
            .collect(),
    )
}
