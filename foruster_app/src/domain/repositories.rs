// SPDX-License-Identifier: GPL-3.0-or-later
mod disk_repository_impl;
use std::rc::Rc;

pub use disk_repository_impl::*;
use traits::{DiskRepository, ProfileRepository};

mod path_repository_impl;
pub use path_repository_impl::*;
use traits::PathRepository;

mod profile_repository_impl;
pub use profile_repository_impl::*;

pub mod traits;

pub fn disk_repo() -> impl DiskRepository + Clone {
    let disk_repository_impl = DiskRepositoryImpl::new();

    disk_repository_impl.update_disks();

    disk_repository_impl
}

pub fn path_repo(disk_repo: Rc<dyn DiskRepository>) -> impl PathRepository + Clone {
    PathRepositoryImpl::new(disk_repo)
}

pub fn profile_repo() -> impl ProfileRepository + Clone {
    ProfileRepositoryImpl::new()
}
