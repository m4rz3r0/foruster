// SPDX-License-Identifier: GPL-3.0-or-later
mod disk_list;

use crate::ui::MainWindow;

pub fn setup(window: &MainWindow) {
    disk_list::setup(window);
}