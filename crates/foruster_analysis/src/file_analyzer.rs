// SPDX-License-Identifier: GPL-3.0-or-later
use std::hash::DefaultHasher;
use foruster_core::FileEntry;
use file_format::{FileFormat, Kind};

fn analyze_file(file: FileEntry) -> Option<String> {
    let fmt = FileFormat::from_file(file.path());
    
    println!("{:?}", fmt);
    
    None
}