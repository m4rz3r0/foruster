// SPDX-License-Identifier: GPL-3.0-or-later
use foruster_profiles::Profile;

#[derive(Debug, Clone)]
pub struct ProfileItem {
    profile_data: Profile,
    selected: bool,
}

impl ProfileItem {
    pub fn new(profile_data: Profile) -> Self {
        Self {
            profile_data,
            selected: false,
        }
    }

    pub fn profile_data(&self) -> &Profile {
        &self.profile_data
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }
}
