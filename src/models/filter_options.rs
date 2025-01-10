// SPDX-License-Identifier: GPL-3.0-or-later
use serde::{Deserialize, Serialize};

use crate::FileEntry;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterOptions {
    min_size: Option<usize>,
    max_size: Option<usize>,
}

impl FilterOptions {
    pub fn new() -> Self {
        FilterOptions {
            min_size: None,
            max_size: None,
        }
    }

    pub fn with_more_details(min_size: Option<usize>, max_size: Option<usize>) -> Self {
        FilterOptions { min_size, max_size }
    }

    pub fn min_size(&self) -> Option<usize> {
        self.min_size
    }

    pub fn max_size(&self) -> Option<usize> {
        self.max_size
    }

    pub fn set_min_size(&mut self, min_size: Option<usize>) {
        self.min_size = min_size;
    }

    pub fn set_max_size(&mut self, max_size: Option<usize>) {
        self.max_size = max_size;
    }

    pub fn filter(&self, file_entry: &FileEntry) -> bool {
        let size = file_entry.size();

        if let Some(min) = self.min_size {
            if size < min {
                return false;
            }
        }

        if let Some(max) = self.max_size {
            if size > max {
                return false;
            }
        }

        true
    }
}
