// SPDX-License-Identifier: GPL-3.0-or-later
use app_core::Disk;

pub struct StorageAPI;

impl StorageAPI {
    pub fn get_all() -> Vec<Disk> {
        storage::storage_extractor().unwrap_or_else(|_| Vec::new())
    }
    
    pub fn get_device_event_listener() -> storage::device_event_listener::DeviceEventListener {
        storage::device_event_listener::DeviceEventListener::new()
    }
}