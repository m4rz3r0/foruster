// SPDX-License-Identifier: GPL-3.0-or-later
mod link_layers;
mod logical_layer_extractor;
mod physical_layer_extractor;
mod storage_extractor;

mod utils;

pub use storage_extractor::storage_extractor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor() {
        storage_extractor::storage_extractor().unwrap();
    }
}
