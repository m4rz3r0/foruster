// SPDX-License-Identifier: GPL-3.0-or-later
mod callback;
pub use callback::*;

pub mod ui;
pub mod mvc;

use slint::ComponentHandle;

slint::include_modules!();

fn init() -> ui::App {
    let view_handle = ui::App::new().unwrap();

    let disk_list_controller = mvc::DiskListController::new(mvc::disk_repo());
    ui::disk_list_adapter::connect(&view_handle, disk_list_controller.clone());

    view_handle
}

fn main() {
    let main_window = init();

    main_window.run().unwrap();
}
