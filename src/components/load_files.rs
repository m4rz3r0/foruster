// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;
use std::{path::Path, vec};
use walkdir::WalkDir;

use crate::{classify_files, AppState, Disk, FileEntry, FileSystem, Report, Route};

#[component]
pub fn LoadFiles() -> Element {
    let nav = navigator();

    let app_state = use_context::<Signal<AppState>>();
    let report = use_context::<Signal<Report>>();

    let loading_status = use_signal(|| String::from("Cargando archivos"));
    let future =
        use_resource(move || async move { get_files(app_state, report, loading_status).await });

    if (*future.read_unchecked()).is_some() {
        spawn_forever(classify_files(app_state));
        nav.push(Route::Profiles {});
    }

    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            div {
                class: "loading loading-spinner loading-lg text-primary"
            }
            h1 {
                class: "font-semibold text-lg",
                { loading_status }
            }
        }
    }
}

async fn get_files(
    mut app_state: Signal<AppState>,
    report: Signal<Report>,
    mut loading_status: Signal<String>,
) {
    let disks: Vec<Disk> = report.peek().selected_disks.to_owned();

    let mut files = vec![];
    for (disk_index, disk) in disks.iter().enumerate() {
        let partitions = disk.partitions();
        for (partition_index, partition) in partitions.iter().enumerate() {
            let volume_paths = match partition.file_system().to_owned() {
                FileSystem::BTRFS(paths) => Some(paths),
                FileSystem::EXT4(path) => Some(vec![path]),
                FileSystem::NTFS(path) => Some(vec![path]),
                FileSystem::FAT32(path) => Some(vec![path]),
                FileSystem::EXFAT(path) => Some(vec![path]),
                FileSystem::XFS(path) => Some(vec![path]),
                FileSystem::ZFS(path) => Some(vec![path]),
                FileSystem::NotImplemented(_, path) => Some(vec![path]),
                FileSystem::Unknown => None,
            };

            if let Some(paths) = volume_paths {
                for path in paths {
                    *loading_status.write() = format!(
                        "Cargando archivos del volumen {} del disco {} ({}/{})",
                        path.to_string_lossy(),
                        disk.model(),
                        disk_index + partition_index + 1,
                        disks.len() * partitions.len()
                    );

                    let volume_files = tokio::task::spawn_blocking(move || get_file_list(&path))
                        .await
                        .unwrap();
                    files.extend(volume_files);
                }
            }
        }
    }

    app_state.write().files = files;
}

#[cfg(target_os = "windows")]
fn get_file_list(path: &Path) -> Vec<FileEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(FileEntry::from)
        .collect()
}

#[cfg(target_os = "linux")]
fn get_file_list(path: &Path) -> Vec<FileEntry> {
    use crate::{get_mounts, is_mount, Mount};

    let mounts = match get_mounts() {
        Ok(mounts) => mounts
            .values()
            .flat_map(|v| v.iter().cloned())
            .collect::<Vec<Mount>>(),
        Err(_) => return Vec::new(),
    };

    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if e.path() != path {
                !is_mount(mounts.as_slice(), e.path())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(FileEntry::from)
        .collect()
}
