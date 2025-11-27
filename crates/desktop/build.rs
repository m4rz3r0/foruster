// SPDX-License-Identifier: GPL-3.0-or-later
fn main() {
    let config = slint_build::CompilerConfiguration::new().with_bundled_translations("ui/i18n");
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
