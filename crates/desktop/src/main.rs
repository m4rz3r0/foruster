// SPDX-License-Identifier: GPL-3.0-or-later
// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use foruster_desktop::launcher;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = launcher::setup()?;

    ui.run()?;

    Ok(())
}
