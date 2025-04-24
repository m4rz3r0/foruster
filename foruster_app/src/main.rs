// SPDX-License-Identifier: GPL-3.0-or-later
pub mod domain;
pub mod ui;

use domain::traits::PathRepository;
use slint::ComponentHandle;
use std::rc::Rc;

slint::include_modules!();

fn init() -> ui::App {
    let view_handle = ui::App::new().unwrap();

    let disk_repo = Rc::new(domain::disk_repo());
    let path_repo = Rc::new(domain::path_repo(disk_repo.clone()));

    let path_repo_clone = path_repo.clone();
    view_handle.on_load_paths(move || {
        path_repo_clone.load_paths();
    });

    let disk_list_controller = domain::DiskListController::new(disk_repo.clone());
    ui::disk_list_adapter::connect(&view_handle, disk_list_controller.clone());

    let path_list_controller = domain::PathListController::new(path_repo.clone());
    ui::path_list_adapter::connect(&view_handle, path_list_controller.clone());

    view_handle
}

fn main() {
    let main_window = init();

    main_window.run().unwrap();
}
