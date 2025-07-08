// SPDX-License-Identifier: GPL-3.0-or-later
mod analysis_result;
mod disk_list;
mod path_management;
mod profile_list;

use crate::ui::MainWindow;
use api::{AnalysisAPI, ProfileAPI, StorageAPI};
use std::cell::RefCell;
use std::rc::Rc;

pub fn setup(window: &MainWindow) {
    let storage_api = Rc::new(RefCell::new(StorageAPI::new()));
    let profile_api = Rc::new(RefCell::new(ProfileAPI::new()));
    let analysis_api = Rc::new(RefCell::new(AnalysisAPI::new()));

    storage_api.borrow_mut().refresh_disks();

    disk_list::setup(window, storage_api.clone());
    path_management::setup(window, storage_api.clone());
    profile_list::setup(window);
    analysis_result::setup(window, analysis_api, profile_api);
}
