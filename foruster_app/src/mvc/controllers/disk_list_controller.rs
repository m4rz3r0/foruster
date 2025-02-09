// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use std::rc::Rc;

use foruster_core::Disk;
use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::mvc;
use crate::mvc::repositories::traits;
//use crate::Callback;

#[derive(Clone)]
pub struct DiskListController {
    disk_model: DiskModel,
    //show_create_disk_callback: Rc<Callback<(), ()>>,
}

impl DiskListController {
    pub fn new(repo: impl traits::DiskRepository + 'static) -> Self {
        Self {
            disk_model: DiskModel::new(repo),
            //show_create_disk_callback: Rc::new(Callback::default()),
        }
    }

    pub fn disk_model(&self) -> ModelRc<mvc::models::DiskModel> {
        ModelRc::new(self.disk_model.clone())
    }

    pub fn toggle_checked(&self, index: usize) {
        self.disk_model.toggle_checked(index)
    }

    pub fn remove_disk(&self, index: usize) {
        self.disk_model.remove_disk(index)
    }

    pub fn create_disk(&self, disk_data: Disk) {
        self.disk_model.push_disk(mvc::models::DiskModel::new(disk_data))
    }

    /*pub fn show_create_disk(&self) {
        self.show_create_disk_callback.invoke(&());
    }

    pub fn on_show_create_disk(&self, mut callback: impl FnMut() + 'static) {
        self.show_create_disk_callback.on(move |()| {
            callback();
        });
    }*/
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

    fn remove_disk(&self, index: usize) {
        if !self.repo.remove_disk(index) {
            return;
        }

        self.notify.row_removed(index, 1)
    }

    fn push_disk(&self, disk: mvc::models::DiskModel) {
        if !self.repo.push_disk(disk) {
            return;
        }

        self.notify.row_added(self.row_count() - 1, 1);
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