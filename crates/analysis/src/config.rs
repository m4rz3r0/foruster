// SPDX-License-Identifier: GPL-3.0-or-later
use profiling::Profile;
use std::path::PathBuf;

#[derive(Default, Clone)]
pub struct Config {
    analysis_profile: Vec<Profile>,
    paths: Vec<PathBuf>,
}

impl Config {
    pub fn new(profiles: Vec<Profile>, paths: Vec<PathBuf>) -> Config {
        Self {
            analysis_profile: profiles,
            paths,
        }
    }

    pub fn analysis_profile(&self) -> &Vec<Profile> {
        &self.analysis_profile
    }

    pub fn paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }
}
