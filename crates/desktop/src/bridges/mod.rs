// SPDX-License-Identifier: GPL-3.0-or-later
mod disk_list;
mod path_management;

use crate::ui::MainWindow;
use api::StorageAPI;
use std::cell::RefCell;
use std::rc::Rc;

pub fn setup(window: &MainWindow) {
    let storage_api = Rc::new(RefCell::new(StorageAPI::new()));
    storage_api.borrow_mut().refresh_disks();

    disk_list::setup(window, storage_api.clone());
    path_management::setup(window, storage_api.clone());
}
