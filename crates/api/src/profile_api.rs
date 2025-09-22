// SPDX-License-Identifier: GPL-3.0-or-later
use profiling::{Profile, default_profiles};

pub struct ProfileAPI {
    profiles: Vec<Profile>,
}

impl Default for ProfileAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileAPI {
    pub fn new() -> ProfileAPI {
        ProfileAPI {
            profiles: default_profiles(),
        }
    }

    pub fn load_default_profiles(&mut self) {
        self.profiles = default_profiles()
    }

    pub fn get_profiles(&self) -> Vec<Profile> {
        self.profiles.clone()
    }

    pub fn get_by_label(&self, label: String) -> Option<Profile> {
        self.profiles
            .iter()
            .find(|profile| *profile.name() == label)
            .cloned()
    }
}
