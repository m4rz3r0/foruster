// SPDX-License-Identifier: GPL-3.0-or-later
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
#[cfg_attr(windows, path = "windows/mod.rs")]
pub mod platform;

pub mod core;

#[cfg(test)]
mod tests {
    use super::*;
    use platform::storage_devices;

    #[test]
    fn it_works() {
        let devices = storage_devices().unwrap();

        for device in devices {
            println!("DEVICE:\n{}\n", device);
        }
    }
}
