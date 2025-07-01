// SPDX-License-Identifier: GPL-3.0-or-later
use std::rc::Rc;
use slint::{ComponentHandle, ModelRc, VecModel};
use crate::ui::{AnalysisResultBridge, File, MainWindow};

fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}

pub fn setup(window: &MainWindow) {
    let bridge = window.global::<AnalysisResultBridge>();

    bridge.on_exportReport(move || {});

    bridge.on_filterByProfile(move |profile| {});

    bridge.on_searchFiles(move |name| {});

    bridge.on_saveAnalysis(move || {});

    bridge.on_goBack(move || {});
}