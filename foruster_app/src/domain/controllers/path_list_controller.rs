// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;
use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::domain::repositories::traits;
use crate::domain::PathItem;

#[derive(Clone)]
pub struct PathListController {
    path_model: PathModel,
}

impl PathListController {
    pub fn new(repo: Rc<dyn traits::PathRepository>) -> Self {
        Self {
            path_model: PathModel::new(repo),
        }
    }

    pub fn path_model(&self) -> ModelRc<PathItem> {
        ModelRc::new(self.path_model.clone())
    }

    pub fn add_path(&self, path: PathBuf) {
        self.path_model.add_path(path)
    }

    pub fn remove_path(&self, index: usize) {
        self.path_model.remove_path(index)
    }

    pub fn update_path(&self, index: usize, path: PathBuf) {
        self.path_model.update_path(index, path)
    }

    pub fn redundant_count(&self) -> usize {
        self.path_model.redundant_count()
    }
}

#[derive(Clone)]
struct PathModel {
    repo: Rc<dyn traits::PathRepository>,
    notify: Rc<ModelNotify>,
}

impl PathModel {
    fn new(repo: Rc<dyn traits::PathRepository>) -> Self {
        Self {
            repo,
            notify: Rc::new(Default::default()),
        }
    }

    fn add_path(&self, path: PathBuf) {
        self.repo.add_path(path);
        self.notify.row_added(self.repo.path_count() - 1, 1);

        self.check_redundant_paths();
    }

    fn remove_path(&self, index: usize) {
        self.repo.remove_path(index);
        self.notify.row_removed(index, 1);

        self.check_redundant_paths();
    }

    fn update_path(&self, index: usize, path: PathBuf) {
        self.repo.update_path(index, path);
        self.notify.row_changed(index);

        self.check_redundant_paths();
    }

    fn redundant_count(&self) -> usize {
        self.repo.redundant_count()
    }

    fn check_redundant_paths(&self) {
        let redundant_paths = self.repo.check_redundant_paths();

        println!("Redundant changes: {:?}", redundant_paths);

        for index in redundant_paths {
            self.notify.row_changed(index);
        }
    }
}

impl Model for PathModel {
    type Data = PathItem;

    fn row_count(&self) -> usize {
        self.repo.path_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_path(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}
