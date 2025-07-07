// SPDX-License-Identifier: GPL-3.0-or-later
use crate::ui::{
    AnalysisResultBridge, File, MainWindow, PathManagementBridge, Profile, ProfileMenuBridge,
};
use api::{AnalysisAPI, ProfileAPI};
use slint::{ComponentHandle, Model, ModelExt, ModelRc, SharedString, VecModel, Weak};
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

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

fn get_selected_profiles(window_weak: Weak<MainWindow>) -> Vec<Profile> {
    let window = window_weak.upgrade().unwrap();

    let profile_bridge = window.global::<ProfileMenuBridge>().get_profiles();

    profile_bridge
        .iter()
        .filter(|profile| profile.selected)
        .collect()
}

pub fn initialize(
    window: &Weak<MainWindow>,
    analysis_api: &Rc<RefCell<AnalysisAPI>>,
    profile_api: &Rc<RefCell<ProfileAPI>>,
) {
    let window = window.upgrade().unwrap();
    let bridge = window.global::<AnalysisResultBridge>();

    let selected_profiles_labels: Vec<_> = get_selected_profiles(window.as_weak())
        .iter()
        .map(|profile| profile.label.clone())
        .collect();

    let selected_profiles_labels_str = selected_profiles_labels.join(", ");
    bridge.set_used_profiles(ModelRc::new(VecModel::from(
        std::iter::once(SharedString::from("Todos"))
            .chain(selected_profiles_labels.clone())
            .collect::<Vec<_>>(),
    )));
    bridge.set_used_profiles_str(SharedString::from(selected_profiles_labels_str));

    let paths = window
        .global::<PathManagementBridge>()
        .get_paths()
        .iter()
        .map(|path| (&path.path).into())
        .collect();

    let selected_profiles = selected_profiles_labels
        .iter()
        .flat_map(|label| profile_api.borrow().get_by_label(label.to_string()))
        .collect();

    analysis_api
        .borrow_mut()
        .initialize(selected_profiles, paths);

    analysis_api.borrow_mut().start();
}

pub fn setup(
    window: &MainWindow,
    analysis_api: Rc<RefCell<AnalysisAPI>>,
    profile_api: Rc<RefCell<ProfileAPI>>,
) {
    let bridge = window.global::<AnalysisResultBridge>();

    let window_weak = window.as_weak();
    let analysis_api_clone = analysis_api.clone();
    let profile_api_clone = profile_api.clone();
    bridge.on_initialize(move || initialize(&window_weak, &analysis_api_clone, &profile_api_clone));
    bridge.on_export_report(move || {});
    bridge.on_filter_by_profile(move |profile| {});
    bridge.on_search_files(move |name| {});
    bridge.on_save_analysis(move || {});
    bridge.on_go_back(move || {});
}

/*
// En tu bridge
pub fn setup_progress_monitoring(
    window: &MainWindow,
    analysis_api: Rc<RefCell<AnalysisAPI>>,
) {
    let window_weak = window.as_weak();
    let analysis_api_clone = analysis_api.clone();
    
    // Timer para actualizar progreso cada 100ms
    slint::Timer::default().start(
        slint::TimerMode::Repeated, 
        std::time::Duration::from_millis(100), 
        move || {
            if let Some(window) = window_weak.upgrade() {
                let progress = analysis_api_clone.borrow().get_progress();
                let bridge = window.global::<AnalysisResultBridge>();
                
                // Actualizar propiedades del bridge
                bridge.set_analysis_percentage(progress.overall_percentage() as i32);
                bridge.set_scanned_files(progress.scanned_files.to_string().into());
                bridge.set_current_file(progress.current_path.into());
                // ... más actualizaciones
            }
        }
    );
}
 */
