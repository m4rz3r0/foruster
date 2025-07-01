// SPDX-License-Identifier: GPL-3.0-or-later
use crate::ui::{MainWindow, Profile, ProfileMenuBridge};
use api::ProfileAPI;
use slint::{Color, ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;

fn map_profile_to_ui(profile: &profiling::Profile) -> Profile {
    Profile {
        bg_color: Color::from_argb_encoded(profile.bg_color()),
        icon_source: profile.icon_source().to_string().into(),
        label: profile.name().into(),
        selected: false,
    }
}

pub fn setup(window: &MainWindow) {
    let bridge = window.global::<ProfileMenuBridge>();

    let profile_api = ProfileAPI::new();
    let profile_list_model = Rc::new(VecModel::from(
        profile_api
            .get_profiles()
            .iter()
            .map(map_profile_to_ui)
            .collect::<Vec<_>>(),
    ));

    bridge.set_profiles(ModelRc::from(profile_list_model.clone()));

    let profile_list_model_clone = profile_list_model.clone();
    bridge.on_count_selected_profiles(move || {
        profile_list_model_clone
            .iter()
            .filter(|profile| profile.selected)
            .count() as i32
    });
}
