// SPDX-License-Identifier: GPL-3.0-or-later
use foruster_profiles::FileCategory;
use std::{cell::RefCell, rc::Rc};

use crate::domain::ProfileItem;

use super::traits::ProfileRepository;

#[derive(Clone)]
pub struct ProfileRepositoryImpl {
    profiles: Rc<RefCell<Vec<ProfileItem>>>,
}

impl ProfileRepositoryImpl {
    pub fn new() -> Self {
        let mut default_profiles = foruster_profiles::default_profiles();
        let mut profiles = vec![];

        for profile in default_profiles.iter_mut() {
            if let Some(categories) = profile.categories() {
                let mut profile = ProfileItem::new(profile.clone());

                match categories.first().unwrap() {
                    FileCategory::Application => {
                        profile.set_background_color(slint::Color::from_argb_encoded(0xFF644661));
                        profile.set_icon_path(String::from("app"));
                    }
                    FileCategory::Audio => {
                        profile.set_background_color(slint::Color::from_argb_encoded(0xFF86883D));
                        profile.set_icon_path(String::from("audio"));
                    }
                    FileCategory::Image => {
                        profile.set_background_color(slint::Color::from_argb_encoded(0xFF5F7E81));
                        profile.set_icon_path(String::from("image"));
                    }
                    FileCategory::Model => {
                        profile.set_background_color(slint::Color::from_argb_encoded(0xFFCA672E));
                        profile.set_icon_path(String::from("model"));
                    }
                    FileCategory::Text => {
                        profile.set_background_color(slint::Color::from_argb_encoded(0xFF9D6C79));
                        profile.set_icon_path(String::from("text"));
                    }
                    FileCategory::Video => {
                        profile.set_background_color(slint::Color::from_argb_encoded(0xFF48705D));
                        profile.set_icon_path(String::from("video"));
                    }
                    FileCategory::Other => {
                        profile.set_background_color(slint::Color::from_argb_encoded(0xFFbc573b));
                        profile.set_icon_path(String::from("other"));
                    }
                }

                profiles.push(profile);
            }
        }

        Self {
            profiles: Rc::new(RefCell::new(
                profiles.into_iter().map(|p| p.into()).collect(),
            )),
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

    fn toggle_selected(&self, index: usize) -> bool {
        if let Some(profile) = self.profiles.borrow_mut().get_mut(index) {
            profile.toggle_selected();
            return true;
        }

        false
    }
}
