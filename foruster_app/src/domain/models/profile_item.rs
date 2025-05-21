// SPDX-License-Identifier: GPL-3.0-or-later
use foruster_profiles::Profile;
use slint::{Color};

#[derive(Debug, Clone)]
pub struct ProfileItem {
    profile_data: Profile,
    background_color: Color,
    icon_path: String,
    selected: bool,
}

impl From<Profile> for ProfileItem {
    fn from(profile_data: Profile) -> Self {
        Self {
            profile_data,
            background_color: Color::from_argb_f32(0.5, 0.65, 0.32, 1.),
            icon_path: "icon.png".to_string(),
            selected: false,
        }
    }
}

impl ProfileItem {
    pub fn new(profile_data: Profile) -> Self {
        Self {
            profile_data,
            background_color: Color::from_argb_f32(0.5, 0.65, 0.32, 1.),
            icon_path: "icon.png".to_string(),
            selected: false,
        }
    }

    pub fn profile_data(&self) -> &Profile {
        &self.profile_data
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn icon_path(&self) -> &String {
        &self.icon_path
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_icon_path(&mut self, path: String) {
        self.icon_path = path;
    }
    
    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }
}
