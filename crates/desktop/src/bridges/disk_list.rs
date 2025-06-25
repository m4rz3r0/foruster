// SPDX-License-Identifier: GPL-3.0-or-later
use std::cell::RefCell;
use std::rc::Rc;
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use crate::ui::{Disk, DiskListBridge, MainWindow};

fn map_to_disk_ui(disk: &app_core::Disk) -> Disk {
    Disk {
        name: disk.name().into(),
        size: disk.total_size_str().into(),
        serial: match disk.identification_data().serial_number() {
            Some(serial) => serial.to_string().into(),
            None => slint::SharedString::new(),
        },
        partitions: disk.partitions().len() as i32,
        mounted_partitions: disk.partitions().iter().filter(|partition| match partition.volume() {
            Some(volume) => volume.is_mounted(),
            None => false
        }).count() as i32,
        interface: disk.identification_data().bus_type().to_string().into(),
        selected: false,
    }
}

fn refresh_disks(disks_model: Rc<VecModel<Disk>>) {
    let new_disks = api::StorageAPI::get_all();

    // Remove disconnected disks
    let mut indices_to_remove = Vec::new();
    for i in 0..disks_model.row_count() {
        let old_disk = disks_model.row_data(i).unwrap();
        if !new_disks.iter().any(|new_disk| old_disk.name == new_disk.name()) {
            indices_to_remove.push(i);
        }
    }

    // Remove in reverse order
    for i in indices_to_remove.iter().rev() {
        disks_model.remove(*i);
    }

    // Add or update connected disks
    for disk in &new_disks {
        if !disks_model.iter().any(|old_disk| old_disk.name == disk.name()) {
            disks_model.push(map_to_disk_ui(disk));
        }
    }
}

pub fn setup(window: &MainWindow) {
    let bridge = window.global::<DiskListBridge>();

    let disk_list_model = Rc::new(VecModel::from(api::StorageAPI::get_all().iter().map(map_to_disk_ui).collect::<Vec<_>>()));
    let event_listener = Rc::new(RefCell::new(api::StorageAPI::get_device_event_listener()));

    bridge.set_disks(ModelRc::from(disk_list_model.clone()));

    bridge.on_check_for_changes(move || event_listener.borrow_mut().poll_event().is_some());
    
    let disk_list_model_clone = disk_list_model.clone();
    bridge.on_refresh_disks(move || refresh_disks(disk_list_model_clone.clone()));

    let disk_list_model_clone = disk_list_model.clone();
    bridge.on_num_selected_disks(move || disk_list_model_clone.clone().iter().filter(|disk| disk.selected).count() as i32);
}
