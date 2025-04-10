// SPDX-License-Identifier: GPL-3.0-or-later
use slint::*;
use std::rc::Rc;

use crate::{
    domain::{DiskItem, DiskListController},
    ui,
};

pub fn connect_with_controller(
    view_handle: &ui::App,
    controller: &DiskListController,
    connect_adapter_controller: impl FnOnce(ui::DiskListAdapter, DiskListController),
) {
    connect_adapter_controller(
        view_handle.global::<ui::DiskListAdapter>(),
        controller.clone(),
    );
}

pub fn connect(view_handle: &ui::App, controller: DiskListController) {
    view_handle
        .global::<ui::DiskListAdapter>()
        .set_disks(Rc::new(MapModel::new(controller.disk_model(), map_disk_to_item)).into());

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_toggle_disk_checked(move |index| {
                controller.toggle_selected(index as usize);
            });
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_count_selected_disks(move || controller.num_selected_disks() as i32);
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_update_disks(move || {
                controller.update_disks();
            })
        }
    });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_check_for_changes(move || controller.check_for_changes());
        }
    });
}

fn map_disk_to_item(disk: DiskItem) -> ui::DiskListItem {
    ui::DiskListItem {
        name: disk.disk_data().name().to_string().into(),
        size: disk.disk_data().total_size_str().to_string().into(),
        serial_number: disk
            .disk_data()
            .identification_data()
            .serial_number()
            .clone()
            .unwrap_or_default()
            .into(),
        num_partitions: disk.disk_data().partitions().len() as i32,
        num_mounted_partitions: disk
            .disk_data()
            .partitions()
            .iter()
            .filter(|p| {
                if let Some(volume) = p.volume() {
                    volume.is_mounted()
                } else {
                    false
                }
            })
            .count() as i32,
        drive_type: disk
            .disk_data()
            .identification_data()
            .bus_type()
            .to_string()
            .into(),

        selected: disk.selected(),
    }
}
