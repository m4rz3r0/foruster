// SPDX-License-Identifier: GPL-3.0-or-later
use std::fmt;

use glob::PatternError;

#[cfg(target_os = "windows")]
use wmi::WMIError;

#[derive(Debug, Clone)]
pub struct DiskError {
    message: String,
}

impl DiskError {
    pub fn new(message: String) -> Self {
        DiskError { message }
    }
}

impl fmt::Display for DiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<PatternError> for DiskError {
    fn from(value: PatternError) -> Self {
        DiskError {
            message: value.to_string(),
        }
    }
}

impl From<std::io::Error> for DiskError {
    fn from(value: std::io::Error) -> Self {
        DiskError {
            message: value.to_string(),
        }
    }
}

#[cfg(target_os = "windows")]
impl From<WMIError> for DiskError {
    fn from(value: WMIError) -> Self {
        DiskError {
            message: value.to_string(),
        }
    }
}
