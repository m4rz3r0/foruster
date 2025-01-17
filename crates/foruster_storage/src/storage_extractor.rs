// SPDX-License-Identifier: GPL-3.0-or-later
use crate::logical_layer_extractor::enumerate_volumes;

pub fn storage_extractor() -> Result<(), windows::core::Error> {
    println!("storage_extractor");

    enumerate_volumes()?;

    Ok(())
}