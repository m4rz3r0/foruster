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
        .get_disks()
        .iter()
        .filter(|disk| {
            selected_disks
                .iter()
                .any(|sel_disk| sel_disk.name == disk.name())
        })
        .flat_map(|disk| {
            disk.partitions()
                .iter()
                .flat_map(|partition| partition.volume())
                .flat_map(|volume| {
                    let vec = volume.mount_points();

                    if vec.is_empty() {
                        None
                    } else {
                        Some((vec, volume.guid()))
                    }
                })
                .flat_map(|(paths, volume_guid)| {
                    paths
                        .iter()
                        .map(|path| map_path_to_ui(path.clone(), volume_guid.to_string()))
                })
                .collect::<Vec<Path>>()
        })
        .collect()
}

fn check_paths(paths_model: &Rc<VecModel<Path>>) {
    for i in 0..paths_model.iter().len() {
        let mut redundancy = false;
        for j in 0..paths_model.iter().len() {
            if i == j {
                continue;
            }

            let path_i = paths_model.row_data(i).unwrap_or_default();
            let path_j = paths_model.row_data(j).unwrap_or_default();
            let path_i_str = path_i.clone().path;
            let path_j_str = path_j.clone().path;

            if path_i_str.starts_with(&path_j_str.to_string()) {
                paths_model.set_row_data(
                    i,
                    Path {
                        redundancy_message: format!(
                            "La ruta {path_i_str} es un subdirectorio de {path_j_str}"
                        )
                        .into(),
                        redundant: true,
                        ..path_i
                    },
                );
                redundancy = true;
                break;
            }
        }

        if !redundancy {
            let path_i = paths_model.row_data(i).unwrap_or_default();

            paths_model.set_row_data(
                i,
                Path {
                    redundancy_message: SharedString::new(),
                    redundant: false,
                    ..path_i
                },
            );
        }
    }
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
        });
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

    let paths_model_clone = paths_model.clone();
    bridge.on_check_paths(move || check_paths(&paths_model_clone));

    let paths_model_clone = paths_model.clone();
    bridge.on_redundant_count(move || {
        paths_model_clone
            .iter()
            .filter(|path| path.redundant)
            .count() as i32
    });

    bridge.on_browse_path(|| {
        FileDialog::new()
            .pick_folder()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            .into()
    });
}
