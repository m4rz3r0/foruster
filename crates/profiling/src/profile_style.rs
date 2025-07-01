// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum IconSource {
    App,
    Audio,
    Image,
    Model,
    Text,
    Video,
}

impl Display for IconSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IconSource::App => {
                    String::from("assets/profile_icons/app.svg")
                }
                IconSource::Audio => {
                    String::from("assets/profile_icons/audio.svg")
                }
                IconSource::Image => {
                    String::from("assets/profile_icons/image.svg")
                }
                IconSource::Model => {
                    String::from("assets/profile_icons/model.svg")
                }
                IconSource::Text => {
                    String::from("assets/profile_icons/text.svg")
                }
                IconSource::Video => {
                    String::from("assets/profile_icons/video.svg")
                }
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct ProfileStyle {
    bg_color: u32,
    icon_source: IconSource,
}

impl ProfileStyle {
    pub fn new(bg_color: u32, icon_source: IconSource) -> Self {
        Self { bg_color, icon_source }
    }
    
    pub fn bg_color(&self) -> u32 {
        self.bg_color
    }
    
    pub fn icon_source(&self) -> &IconSource {
        &self.icon_source
    }
}
