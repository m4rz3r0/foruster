// SPDX-License-Identifier: GPL-3.0-or-later
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[cfg(windows)]
use foruster_storage::platform::DeviceEventListener;
use foruster_storage::platform::storage_devices;

pub use foruster_storage::core::Disk;

pub struct StorageAPI {
    #[cfg(windows)]
    event_listener: Rc<RefCell<DeviceEventListener>>,
    cached_disks: Vec<Disk>,
}

impl StorageAPI {
    pub fn new() -> StorageAPI {
        Self {
            #[cfg(windows)]
            event_listener: Rc::new(RefCell::new(DeviceEventListener::new())),
            cached_disks: Vec::new(),
        }
    }

    pub fn refresh_disks(&mut self) {
        self.cached_disks = storage_devices().unwrap_or_default();
    }

    pub fn get_disks(&self) -> &Vec<Disk> {
        &self.cached_disks
    }

    #[cfg(windows)]
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
            let mount_points = volume.mount_points();

            if mount_points
                .iter()
                .any(|mount_point| path.starts_with(mount_point))
            {
                return Some(volume.guid().to_string());
            }
        }

        None
    }
}
