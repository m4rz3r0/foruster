// SPDX-License-Identifier: GPL-3.0-or-later
use std::{cell::RefCell, rc::Rc};

use crate::domain::ProfileItem;

use super::traits::ProfileRepository;

pub struct ProfileRepositoryImpl {
    profiles: Rc<RefCell<Vec<ProfileItem>>>,
}

impl ProfileRepositoryImpl {
    pub fn new() -> Self {
        Self {
            profiles: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl Default for ProfileRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileRepository for ProfileRepositoryImpl {
    fn get_profile(&self, index: usize) -> Option<ProfileItem> {
        self.profiles.borrow().get(index).cloned()
    }

    fn get_all_profiles(&self) -> Vec<ProfileItem> {
        self.profiles.borrow().clone()
    }

    fn clear_profiles(&self) {
        self.profiles.borrow_mut().clear();
    }

    fn selected_profile_count(&self) -> usize {
        self.profiles
            .borrow()
            .iter()
            .filter(|profile| profile.selected())
            .count()
    }
}
