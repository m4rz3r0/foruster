// SPDX-License-Identifier: GPL-3.0-or-later
use slint::PlatformError;
use crate::bridges;
use crate::ui::MainWindow;

pub fn setup() -> Result<MainWindow, PlatformError> {
    let ui = MainWindow::new()?;
    
    bridges::setup(&ui);
    
    Ok(ui)
}