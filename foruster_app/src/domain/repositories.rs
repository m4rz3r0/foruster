// SPDX-License-Identifier: GPL-3.0-or-later
mod disk_repository_impl;
pub use disk_repository_impl::*;
use traits::DiskRepository;

pub mod traits;

pub fn disk_repo() -> impl traits::DiskRepository + Clone {
    let disk_repository_impl = DiskRepositoryImpl::new();

    disk_repository_impl.update_disks();

    disk_repository_impl
}
