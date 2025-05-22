// SPDX-License-Identifier: GPL-3.0-or-later
use slint::*;
use std::rc::Rc;

use crate::{
    domain::{ProfileItem, ProfilesController},
    ui,
};

pub fn connect_with_controller(
    view_handle: &ui::App,
    controller: &ProfilesController,
    connect_adapter_controller: impl FnOnce(ui::ProfileListAdapter, ProfilesController),
) {
    connect_adapter_controller(
        view_handle.global::<ui::ProfileListAdapter>(),
        controller.clone(),
    );
}

pub fn connect(view_handle: &ui::App, controller: ProfilesController) {
    view_handle.global::<ui::ProfileListAdapter>().set_profiles(
        Rc::new(MapModel::new(
            controller.profiles_model(),
            map_profile_to_item,
        ))
        .into(),
    );

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_toggle_profile_checked(move |index| {
                controller.toggle_selected(index as usize);
            });
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_count_selected_profiles(move || controller.selected_profile_count() as i32);
        }
    });
}

fn map_profile_to_item(profile: ProfileItem) -> ui::ProfileListItem {
    ui::ProfileListItem {
        label: profile.profile_data().name().into(),
        bg_color: profile.background_color(),
        icon_source: profile.icon_path().into(),

        selected: profile.selected(),
    }
}
