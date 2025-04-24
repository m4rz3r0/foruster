// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;

use crate::domain::PathItem;

pub trait PathRepository {
    fn get_path(&self, index: usize) -> Option<PathItem>;
    fn add_path(&self, path: PathBuf);
    fn remove_path(&self, index: usize);
    fn update_path(&self, index: usize, path: PathBuf);

    fn load_paths(&self);
    fn check_redundant_paths(&self) -> Vec<usize>;
    fn path_count(&self) -> usize;
    fn redundant_count(&self) -> usize;
}