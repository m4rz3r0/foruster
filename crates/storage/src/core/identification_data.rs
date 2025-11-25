// SPDX-License-Identifier: GPL-3.0-or-later
use num_enum::{FromPrimitive, IntoPrimitive};
use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive)]
pub enum StorageBusType {
    Scsi = 0x01,
    Atapi = 0x02,
    Ata = 0x03,
    FireWire1394 = 0x04,
    Ssa = 0x05,
    Fibre = 0x06,
    Usb = 0x07,
    Raid = 0x08,
    Iscsi = 0x09,
    Sas = 0x0A,
    Sata = 0x0B,
    Sd = 0x0C,
    Mmc = 0x0D,
    Virtual = 0x0E,
    FileBackedVirtual = 0x0F,
    Spaces = 0x10,
    Nvme = 0x11,
    Scm = 0x12,
    BusTypeMaxReserved = 0x7F,
    #[num_enum(default)]
    Unknown,
}

impl fmt::Display for StorageBusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StorageBusType::Scsi => "SCSI",
                StorageBusType::Atapi => "ATAPI",
                StorageBusType::Ata => "ATA",
                StorageBusType::FireWire1394 => "1394",
                StorageBusType::Ssa => "SSA",
                StorageBusType::Fibre => "Fibre",
                StorageBusType::Usb => "USB",
                StorageBusType::Raid => "RAID",
                StorageBusType::Iscsi => "iSCSI",
                StorageBusType::Sas => "SAS",
                StorageBusType::Sata => "SATA",
                StorageBusType::Sd => "SD",
                StorageBusType::Mmc => "MMC",
                StorageBusType::Virtual => "VIRTUAL",
                StorageBusType::FileBackedVirtual => "FileBackedVirtual",
                StorageBusType::Spaces => "Spaces",
                StorageBusType::Nvme => "NVMe",
                StorageBusType::Scm => "SCM",
                StorageBusType::BusTypeMaxReserved => "BusTypeMaxReserved",
                StorageBusType::Unknown => "UNKNOWN",
            }
        )
    }
}

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
