// SPDX-License-Identifier: GPL-3.0-or-later
mod mock_disk_repository;
pub use mock_disk_repository::*;
use traits::DiskRepository;

pub mod traits;

pub fn disk_repo() -> impl traits::DiskRepository + Clone {
    let mock_disk_repository = MockDiskRepository::new();

    mock_disk_repository.update_disks();

    mock_disk_repository
}
