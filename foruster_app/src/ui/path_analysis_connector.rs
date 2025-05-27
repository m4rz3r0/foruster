// SPDX-License-Identifier: GPL-3.0-or-later
use slint::*;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::domain::analysis_models::{AnalysisUpdate, PathAnalysisResult};
use crate::domain::path_analyzer;
use crate::ui;

use super::PathAnalysisState;

pub fn connect(app_handle: &ui::App) { // app_handle es &super::App (o crate::ui::App)
    let ui_handle_weak = app_handle.as_weak();

    let (path_analysis_tx, path_analysis_rx): (
        Sender<AnalysisUpdate<PathAnalysisResult>>,
        Receiver<AnalysisUpdate<PathAnalysisResult>>,
    ) = mpsc::channel();

    // Acceder al global. Asegúrate que PathManagementViewCallbacks esté exportado en tus .slint files
    // y que app.slint (o el archivo compilado por build.rs) lo haga accesible.
    /*app_handle.global::<ui::PathManagementViewCallbacks>().on_start_path_analysis(
        move |path_to_analyze: slint::SharedString| {
            let tx = path_analysis_tx.clone();
            let current_ui = ui_handle_weak.clone();

            current_ui
                .upgrade_in_event_loop(move |ui_ref| {
                    // Acceder al global. Asegúrate que PathAnalysisState esté exportado.
                    // Usar PathAnalysisState directamente ya que está importado con `use super::PathAnalysisState;`
                    let state = ui_ref.global::<PathAnalysisState>();
                    state.set_running(true);
                    state.set_indeterminate(true);
                    state.set_message("Iniciando análisis de rutas...".into());
                })
                .expect("Failed to upgrade UI handle for starting analysis");

            path_analyzer::start_path_analysis(path_to_analyze.to_string(), tx);
        },
    );*/

    let timer = slint::Timer::default();
    let ui_handle_timer_weak = app_handle.as_weak();
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(100),
        move || {
            if let Ok(update) = path_analysis_rx.try_recv() {
                let current_ui = ui_handle_timer_weak.clone();
                current_ui
                    .upgrade_in_event_loop(move |ui_ref| {
                        let state = ui_ref.global::<PathAnalysisState>();
                        match update {
                            AnalysisUpdate::Progress {
                                percentage,
                                message,
                            } => {
                                state.set_indeterminate(false);
                                state.set_progress(percentage);
                                state.set_message(message.into());
                            }
                            AnalysisUpdate::Completed(result) => {
                                state.set_running(false);
                                state.set_message("Análisis de rutas completado.".into());
                                println!("Resultado del análisis de rutas: {:?}", result);
                                // Aquí se podría guardar el resultado o actualizar otro modelo/vista
                            }
                            AnalysisUpdate::Error(err_msg) => {
                                state.set_running(false);
                                state.set_message(
                                    std::format!("Error en análisis: {}", err_msg).into(), // Asegurar std::format!
                                );
                            }
                        }
                    })
                    .expect("Failed to upgrade UI handle for analysis update");
            }
        },
    );
    std::mem::forget(timer);
}
