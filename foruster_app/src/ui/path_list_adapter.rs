// SPDX-License-Identifier: GPL-3.0-or-later
use slint::*;
use std::{path::PathBuf, rc::Rc};

use crate::{
    domain::{PathItem, PathListController},
    ui,
};

pub fn connect_with_controller(
    view_handle: &ui::App,
    controller: &PathListController,
    connect_adapter_controller: impl FnOnce(ui::PathListAdapter, PathListController),
) {
    connect_adapter_controller(
        view_handle.global::<ui::PathListAdapter>(),
        controller.clone(),
    );
}

pub fn connect(view_handle: &ui::App, controller: PathListController) {
    view_handle
        .global::<ui::PathListAdapter>()
        .set_paths(Rc::new(MapModel::new(controller.path_model(), map_path_to_item)).into());

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_add_path(move |path| {
                controller.add_path(PathBuf::from(path.as_str()));
            });
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_remove_path(move |index| {
                controller.remove_path(index as usize);
            });
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_update_path(move |index, path| {
                controller.update_path(index as usize, PathBuf::from(path.as_str()));
            });
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_redundant_count(move || controller.redundant_count() as i32);
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_browse_for_path(move || {
                controller
                    .browse_path()
                    .to_string_lossy()
                    .to_string()
                    .into()
            });
        }
    });
}

fn map_path_to_item(path: PathItem) -> ui::PathListItem {
    ui::PathListItem {
        path: path.path().to_string_lossy().to_string().into(),
        redundant: path.is_redundant(),
        redundancy_message: path.redundancy_message().to_string().into(),
    }
}
