// SPDX-License-Identifier: GPL-3.0-or-later
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = App::new()?;
    let ui_handle = ui.as_weak();
    ui.run()
}
