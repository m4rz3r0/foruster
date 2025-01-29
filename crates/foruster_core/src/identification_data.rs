// SPDX-License-Identifier: GPL-3.0-or-later
use crate::storage_bus_type::StorageBusType;

#[derive(Debug)]
pub struct IdentificationData {
    vendor: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    product_revision: Option<String>,
    bus_type: StorageBusType,
    removable: bool,
}

impl IdentificationData {
    pub fn new(
        vendor: Option<String>,
        model: Option<String>,
        serial_number: Option<String>,
        product_revision: Option<String>,
        bus_type: StorageBusType,
        removable: bool,
    ) -> Self {
        Self {
            model,
            serial_number,
            product_revision,
            vendor,
            bus_type,
            removable,
        }
    }

    pub fn vendor(&self) -> &Option<String> {
        &self.vendor
    }

    pub fn model(&self) -> &Option<String> {
        &self.model
    }

    pub fn serial_number(&self) -> &Option<String> {
        &self.serial_number
    }

    pub fn product_revision(&self) -> &Option<String> {
        &self.product_revision
    }

    pub fn bus_type(&self) -> &StorageBusType {
        &self.bus_type
    }

    pub fn removable(&self) -> bool {
        self.removable
    }
}
