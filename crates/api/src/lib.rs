// SPDX-License-Identifier: GPL-3.0-or-later
use app_core::Disk;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use storage::device_event_listener::DeviceEventListener;

pub struct StorageAPI {
    event_listener: Rc<RefCell<DeviceEventListener>>,
    cached_disks: Vec<Disk>,
}

impl StorageAPI {
    pub fn new() -> StorageAPI {
        Self {
            event_listener: Rc::new(RefCell::new(DeviceEventListener::new())),
            cached_disks: Vec::new(),
        }
    }

    pub fn refresh_disks(&mut self) {
        self.cached_disks = storage::storage_extractor().unwrap_or_default();
    }

    pub fn get_all(&self) -> &Vec<Disk> {
        &self.cached_disks
    }

    pub fn get_device_event_listener(&self) -> Rc<RefCell<DeviceEventListener>> {
        self.event_listener.clone()
    }

    pub fn get_volume_id(&self, path: PathBuf) -> Option<String> {
        let volumes = self
            .cached_disks
            .iter()
            .map(|disk| {
                disk.partitions()
                    .iter()
                    .flat_map(|partition| partition.volume())
            })
            .flatten()
            .collect::<Vec<_>>();

        for volume in volumes {
            let drive_letters = volume.drive_letters();
            let mount_points = volume.mount_points();

            if drive_letters
                .iter()
                .any(|drive_letter| path.starts_with(drive_letter.to_string() + ":"))
                || mount_points
                    .iter()
                    .any(|mount_point| path.starts_with(mount_point))
            {
                return Some(volume.guid().to_string());
            }
        }

        None
    }
}
