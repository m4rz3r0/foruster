// SPDX-License-Identifier: GPL-3.0-or-later
mod storage_extractor;
mod physical_layer_extractor;
mod logical_layer_extractor;

mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor() {
        storage_extractor::storage_extractor().unwrap();
    }
}