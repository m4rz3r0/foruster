// SPDX-License-Identifier: GPL-3.0-or-later
use profiling::{Profile, default_profiles};

pub struct ProfileAPI {
    profiles: Vec<Profile>,
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
}
