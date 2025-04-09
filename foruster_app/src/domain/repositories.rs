// SPDX-License-Identifier: GPL-3.0-or-later
mod disk_repository_impl;
pub use disk_repository_impl::*;
use traits::DiskRepository;

pub mod traits;

pub fn disk_repo() -> impl traits::DiskRepository + Clone {
    let mock_disk_repository = DiskRepositoryImpl::new();

    mock_disk_repository.update_disks();

    mock_disk_repository
}
