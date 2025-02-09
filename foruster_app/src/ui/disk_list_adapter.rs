// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{DiskListController, DiskModel},
    ui,
};

// a helper function to make adapter and controller connection a little bit easier
pub fn connect_with_controller(
    view_handle: &ui::App,
    controller: &DiskListController,
    connect_adapter_controller: impl FnOnce(ui::DiskListAdapter, DiskListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::DiskListAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::App, controller: DiskListController) {
    // sets a mapped list of the disk items to the ui
    view_handle
        .global::<ui::DiskListAdapter>()
        .set_disks(Rc::new(MapModel::new(controller.disk_model(), map_disk_to_item)).into());

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_toggle_disk_checked(move |index| {
                controller.toggle_checked(index as usize);
            })
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_remove_disk(move |index| {
                controller.remove_disk(index as usize);
            })
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter: ui::DiskListAdapter, controller| {
            adapter.on_show_create_disk(move || {
                //controller.show_create_disk();
            })
        }
    });
}

// maps a DiskModel (data) to a SelectionItem (ui)
fn map_disk_to_item(disk: DiskModel) -> ui::SelectionListViewItem {
    ui::SelectionListViewItem {
        name: disk.disk_data().name().to_string().into(),
        size: disk.disk_data().total_size_str().to_string().into(),
        serial_number: disk.disk_data().identification_data().serial_number().clone().unwrap_or_default().into(),
        r#type: disk.disk_data().identification_data().bus_type().to_string().into(),
        checked: disk.checked(),
    }
}