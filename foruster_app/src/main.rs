// SPDX-License-Identifier: GPL-3.0-or-later
// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let disks = foruster_storage::storage_extractor()?;

    let disks_str = disks.iter().map(|disk| disk.to_string()).collect::<Vec<String>>().join("\n");
    // Replace tabs with spaces to avoid text wrapping in Slint
    let disks_str = disks_str.replace("\t", "    ");

    ui.set_dynamicText(disks_str.into());

    ui.run()?;

    Ok(())
}
