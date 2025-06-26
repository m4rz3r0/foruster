// SPDX-License-Identifier: GPL-3.0-or-later
use crate::bridges;
use crate::ui::MainWindow;
use slint::PlatformError;

pub fn setup() -> Result<MainWindow, PlatformError> {
    let ui = MainWindow::new()?;

    bridges::setup(&ui);

    Ok(ui)
}
