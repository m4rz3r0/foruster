// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

#[repr(u8)]
#[derive(Debug)]
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
    Unknown,
}

impl From<u8> for StorageBusType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => StorageBusType::Scsi,
            0x02 => StorageBusType::Atapi,
            0x03 => StorageBusType::Ata,
            0x04 => StorageBusType::FireWire1394,
            0x05 => StorageBusType::Ssa,
            0x06 => StorageBusType::Fibre,
            0x07 => StorageBusType::Usb,
            0x08 => StorageBusType::Raid,
            0x09 => StorageBusType::Iscsi,
            0x0A => StorageBusType::Sas,
            0x0B => StorageBusType::Sata,
            0x0C => StorageBusType::Sd,
            0x0D => StorageBusType::Mmc,
            0x0E => StorageBusType::Virtual,
            0x0F => StorageBusType::FileBackedVirtual,
            0x10 => StorageBusType::Spaces,
            0x11 => StorageBusType::Nvme,
            0x12 => StorageBusType::Scm,
            0x7F => StorageBusType::BusTypeMaxReserved,
            _ => StorageBusType::Unknown,
        }
    }
}

impl fmt::Display for StorageBusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
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
        };
        write!(f, "{}", str)
    }
}