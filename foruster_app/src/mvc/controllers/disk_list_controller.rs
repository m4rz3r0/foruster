// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::mvc;
use crate::mvc::repositories::traits;
use crate::Callback;

#[derive(Clone)]
pub struct DiskListController {
    disk_model: DiskModel,
    analyze_disks_callback: Rc<Callback<(), ()>>,
}

impl DiskListController {
    pub fn new(repo: impl traits::DiskRepository + 'static) -> Self {
        Self {
            disk_model: DiskModel::new(repo),
            analyze_disks_callback: Rc::new(Callback::default()),
        }
    }

    pub fn disk_model(&self) -> ModelRc<mvc::models::DiskModel> {
        ModelRc::new(self.disk_model.clone())
    }

    pub fn toggle_checked(&self, index: usize) {
        self.disk_model.toggle_checked(index)
    }

    pub fn update_disks(&self) {
        self.disk_model.update_disks()
    }

    pub fn analyze_disks(&self) {
        self.analyze_disks_callback.invoke(&());
    }

    pub fn num_selected_disks(&self) -> usize {
        self.disk_model.num_selected_disks()
    }

    pub fn on_show_analyze_disks(&self, mut callback: impl FnMut() + 'static) {
        self.analyze_disks_callback.on(move |()| {
            callback();
        });
    }
}

#[derive(Clone)]
struct DiskModel {
    repo: Rc<dyn traits::DiskRepository>,
    notify: Rc<ModelNotify>,
}

impl DiskModel {
    fn new(repo: impl traits::DiskRepository + 'static) -> Self {
        Self { repo: Rc::new(repo), notify: Rc::new(Default::default()) }
    }

    fn toggle_checked(&self, index: usize) {
        if !self.repo.toggle_checked(index) {
            return;
        }

        self.notify.row_changed(index)
    }

    fn num_selected_disks(&self) -> usize {
        self.repo.checked_disk_count()
    }

    fn update_disks(&self) {
        self.repo.update_disks();
        self.notify.reset();
    }
}

impl Model for DiskModel {
    type Data = mvc::models::DiskModel;

    fn row_count(&self) -> usize {
        self.repo.disk_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_disk(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}