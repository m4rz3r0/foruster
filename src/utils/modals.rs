// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;
use document::eval;

use crate::ModalInfo;

pub fn show_error(mut modal_info: Signal<ModalInfo>, error: &str) {
    modal_info.write().title = "¡Error!".to_string();
    modal_info.write().content = error.to_string();

    eval(r#"error_modal.showModal()"#);
}

pub fn show_success(mut modal_info: Signal<ModalInfo>, success: &str) {
    modal_info.write().title = "¡Éxito!".to_string();
    modal_info.write().content = success.to_string();

    eval(r#"success_modal.showModal()"#);
}