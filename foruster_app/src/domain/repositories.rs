// SPDX-License-Identifier: GPL-3.0-or-later
mod disk_repository_impl;
use std::rc::Rc;

pub use disk_repository_impl::*;
use traits::DiskRepository;

mod path_repository_impl;
pub use path_repository_impl::*;
use traits::PathRepository;

pub mod traits;

pub fn disk_repo() -> impl traits::DiskRepository + Clone {
    let disk_repository_impl = DiskRepositoryImpl::new();

    disk_repository_impl.update_disks();

    disk_repository_impl
}

pub fn path_repo(disk_repo: Rc<dyn DiskRepository>) -> impl traits::PathRepository + Clone {
    let path_repository_impl = PathRepositoryImpl::new(disk_repo);

    path_repository_impl
}