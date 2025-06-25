// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

use crate::storage_bus_type::StorageBusType;

#[derive(Debug, Clone)]
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
        let vendor = vendor.map(|v| v.trim().to_string());
        let model = model.map(|m| m.trim().to_string());

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

impl fmt::Display for IdentificationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(vendor) = &self.vendor {
            writeln!(f, "Vendor: {}", vendor)?;
        }

        if let Some(model) = &self.model {
            writeln!(f, "Model: {}", model)?;
        }

        if let Some(serial_number) = &self.serial_number {
            writeln!(f, "Serial number: {}", serial_number)?;
        }

        if let Some(product_revision) = &self.product_revision {
            writeln!(f, "Product revision: {}", product_revision)?;
        }

        writeln!(f, "Bus type: {}", self.bus_type)?;
        writeln!(f, "Removable: {}", self.removable)?;

        Ok(())
    }
}
