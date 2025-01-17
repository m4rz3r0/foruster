// SPDX-License-Identifier: GPL-3.0-or-later
pub struct IdentificationData {
    model: String,
    serial_number: String,
    firmware_revision: String,
    user_capacity: u64,
    manufacturer: String,
}

impl IdentificationData {
    pub fn new(
        model: String,
        serial_number: String,
        firmware_revision: String,
        user_capacity: u64,
        manufacturer: String,
    ) -> Self {
        Self {
            model,
            serial_number,
            firmware_revision,
            user_capacity,
            manufacturer,
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    pub fn firmware_revision(&self) -> &str {
        &self.firmware_revision
    }

    pub fn user_capacity(&self) -> u64 {
        self.user_capacity
    }

    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }
}
