// SPDX-License-Identifier: GPL-3.0-or-later
use crate::ui::{Disk, DiskListBridge, MainWindow, Path, PathManagementBridge};
use api::StorageAPI;
use rfd::FileDialog;
use slint::{ComponentHandle, Model, ModelExt, ModelRc, SharedString, VecModel, Weak};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn map_path_to_ui(path: PathBuf, volume_id: String) -> Path {
    Path {
        path: path.to_string_lossy().to_string().into(),
        redundant: false,
        redundancy_message: SharedString::new(),
        volume_id: volume_id.into(),
    }
}

fn get_start_paths(
    window_weak: &Weak<MainWindow>,
    storage_api: Rc<RefCell<StorageAPI>>,
) -> Vec<Path> {
    let window = window_weak.unwrap();
    let disk_list_bridge = window.global::<DiskListBridge>();

    let selected_disks: Vec<Disk> = disk_list_bridge
        .get_disks()
        .filter(|disk| disk.selected)
        .iter()
        .collect();

    storage_api
        .borrow()
        .get_all()
        .into_iter()
        .filter(|disk| {
            selected_disks
                .iter()
                .any(|sel_disk| sel_disk.name == disk.name())
        })
        .map(|disk| {
            disk.partitions()
                .iter()
                .flat_map(|partition| partition.volume())
                .flat_map(|volume| {
                    let mut vec = volume
                        .drive_letters()
                        .iter()
                        .map(|drive_letter| format!("{}:\\", drive_letter).into())
                        .collect::<Vec<PathBuf>>();
                    vec.extend_from_slice(volume.mount_points());

                    if vec.is_empty() {
                        None
                    } else {
                        Some((vec, volume.guid()))
                    }
                })
                .map(|(paths, volume_guid)| {
                    paths
                        .into_iter()
                        .map(|path| map_path_to_ui(path, volume_guid.to_string()))
                })
                .flatten()
                .collect::<Vec<Path>>()
        })
        .flatten()
        .collect()
}

pub fn setup(window: &MainWindow, storage_api: Rc<RefCell<StorageAPI>>) {
    let bridge = window.global::<PathManagementBridge>();

    let paths_model = Rc::new(VecModel::from(Vec::<Path>::new()));

    bridge.set_paths(ModelRc::from(paths_model.clone()));

    let window_weak = window.as_weak();
    let paths_model_clone = paths_model.clone();
    let storage_api_clone = storage_api.clone();
    bridge.on_get_start_paths(move || {
        paths_model_clone.set_vec(get_start_paths(&window_weak, storage_api_clone.clone()))
    });

    let paths_model_clone = paths_model.clone();
    let storage_api_clone = storage_api.clone();
    bridge.on_add_path(move |path| {
        paths_model_clone.push(Path {
            path: path.clone(),
            redundancy_message: Default::default(),
            redundant: false,
            volume_id: storage_api_clone
                .borrow()
                .get_volume_id(path.to_string().into())
                .unwrap_or_default()
                .into(),
        })
    });

    let paths_model_clone = paths_model.clone();
    bridge.on_remove_path(move |index| {
        paths_model_clone.remove(index as usize);
    });

    let paths_model_clone = paths_model.clone();
    let storage_api_clone = storage_api.clone();
    bridge.on_update_path(move |index, new_path| {
        paths_model_clone.set_row_data(
            index as usize,
            map_path_to_ui(
                new_path.to_string().into(),
                storage_api_clone
                    .borrow()
                    .get_volume_id(new_path.to_string().into())
                    .unwrap_or_default(),
            ),
        );
    });

    bridge.on_browse_path(|| {
        FileDialog::new()
            .pick_folder()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            .into()
    })
}
