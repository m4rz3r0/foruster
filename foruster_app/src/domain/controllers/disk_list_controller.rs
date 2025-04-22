// SPDX-License-Identifier: GPL-3.0-or-later
use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::domain::repositories::traits;
use crate::domain::DiskItem;

#[derive(Clone)]
pub struct DiskListController {
    disk_model: DiskModel,
}

impl DiskListController {
    pub fn new(repo: Rc<dyn traits::DiskRepository>) -> Self {
        Self {
            disk_model: DiskModel::new(repo),
        }
    }

    pub fn disk_model(&self) -> ModelRc<DiskItem> {
        ModelRc::new(self.disk_model.clone())
    }

    pub fn toggle_selected(&self, index: usize) {
        self.disk_model.toggle_selected(index)
    }

    pub fn update_disks(&self) {
        self.disk_model.update_disks()
    }

    pub fn num_selected_disks(&self) -> usize {
        self.disk_model.num_selected_disks()
    }

    pub fn check_for_changes(&self) -> bool {
        self.disk_model.check_for_device_changes()
    }
}

#[derive(Clone)]
struct DiskModel {
    repo: Rc<dyn traits::DiskRepository>,
    notify: Rc<ModelNotify>,
}

impl DiskModel {
    fn new(repo: Rc<dyn traits::DiskRepository>) -> Self {
        Self {
            repo,
            notify: Rc::new(Default::default()),
        }
    }

    fn toggle_selected(&self, index: usize) {
        if !self.repo.toggle_selected(index) {
            return;
        }

        self.notify.row_changed(index)
    }

    fn num_selected_disks(&self) -> usize {
        self.repo.selected_disk_count()
    }

    fn update_disks(&self) {
        self.repo.update_disks();
        self.notify.reset();
    }

    fn check_for_device_changes(&self) -> bool {
        self.repo.check_for_device_changes()
    }
}

impl Model for DiskModel {
    type Data = DiskItem;

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
