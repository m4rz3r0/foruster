// SPDX-License-Identifier: GPL-3.0-or-later
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    CM_NOTIFY_ACTION, CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL,
    CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL, CM_NOTIFY_EVENT_DATA, CM_NOTIFY_FILTER,
    CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES, CM_Register_Notification, HCMNOTIFICATION,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DeviceEvent {
    Added,
    Removed,
    Changed,
}

pub struct CallbackContext {
    pub sender: Sender<DeviceEvent>,
}

pub struct DeviceEventListener {
    event_receiver: Receiver<DeviceEvent>,
    _handle: HCMNOTIFICATION,
    _context: *mut CallbackContext,
    // HashMap to track recently processed events with timestamps
    recent_events: HashMap<DeviceEvent, Instant>,
    // Debounce duration to filter duplicate events
    debounce_duration: Duration,
}

unsafe extern "system" fn notification_callback(
    _notification_handle: HCMNOTIFICATION,
    context: *const c_void,
    action: CM_NOTIFY_ACTION,
    _event_data: *const CM_NOTIFY_EVENT_DATA,
    _flags: u32,
) -> u32 {
    let ctx = &*(context as *const CallbackContext);
    let event = match action {
        CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL => Some(DeviceEvent::Added),
        CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL => Some(DeviceEvent::Removed),
        _ => None,
    };

    if let Some(evt) = event {
        if ctx.sender.send(evt).is_err() { 1 } else { 0 }
    } else {
        1
    }
}

impl Default for DeviceEventListener {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceEventListener {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<DeviceEvent>();

        let ctx = Box::new(CallbackContext { sender });
        let context = Box::into_raw(ctx) as *const c_void;

        let filter = CM_NOTIFY_FILTER {
            cbSize: std::mem::size_of::<CM_NOTIFY_FILTER>() as u32,
            Flags: CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES,
            ..Default::default()
        };

        let mut handle = HCMNOTIFICATION::default();
        unsafe {
            let result = CM_Register_Notification(
                &filter,
                Some(context),
                Some(notification_callback),
                &mut handle,
            );
            if result.0 != 0 {
                panic!("Fallo al registrar notificaciones: {:?}", result);
            }
        }

        Self {
            event_receiver: receiver,
            _handle: handle,
            _context: context as *mut CallbackContext,
            recent_events: HashMap::new(),
            debounce_duration: Duration::from_secs(1),
        }
    }

    pub fn poll_event(&mut self) -> Option<DeviceEvent> {
        while let Ok(event) = self.event_receiver.try_recv() {
            let now = Instant::now();
            if let Some(&last_time) = self.recent_events.get(&event) {
                if now.duration_since(last_time) < self.debounce_duration {
                    continue;
                }
            }
            self.recent_events.insert(event, now);
            return Some(event);
        }
        None
    }
}

impl Drop for DeviceEventListener {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self._context);
            if !self._handle.is_invalid() {
                let _ = CM_Register_Notification(
                    &CM_NOTIFY_FILTER::default(),
                    None,
                    None,
                    &mut self._handle,
                );
            }
        }
    }
}
