// SPDX-License-Identifier: GPL-3.0-or-later
pub trait FileRepository {
    fn add_path(&self, path: &str);
    fn get_path(&self, index: usize) -> Option<String>;
    fn get_all_paths(&self) -> Vec<String>;
    fn clear_paths(&self);
}