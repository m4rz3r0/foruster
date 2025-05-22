// SPDX-License-Identifier: GPL-3.0-or-later
use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::domain::traits::ProfileRepository;
use crate::domain::ProfileItem;

#[derive(Clone)]
pub struct ProfilesController {
    profiles_model: ProfilesModel,
}

impl ProfilesController {
    pub fn new(repo: Rc<dyn ProfileRepository>) -> Self {
        Self {
            profiles_model: ProfilesModel::new(repo),
        }
    }

    pub fn toggle_selected(&self, index: usize) {
        self.profiles_model.toggle_selected(index);
    }

    pub fn profiles_model(&self) -> ModelRc<ProfileItem> {
        ModelRc::new(self.profiles_model.clone())
    }

    pub fn get_all_profiles(&self) -> Vec<ProfileItem> {
        self.profiles_model.get_all_profiles()
    }

    pub fn selected_profile_count(&self) -> usize {
        self.profiles_model.selected_profile_count()
    }
}

#[derive(Clone)]
struct ProfilesModel {
    repo: Rc<dyn ProfileRepository>,
    notify: Rc<ModelNotify>,
}

impl ProfilesModel {
    pub fn new(repo: Rc<dyn ProfileRepository>) -> Self {
        Self {
            repo,
            notify: Rc::new(Default::default()),
        }
    }

    fn toggle_selected(&self, index: usize) {
        if !self.repo.toggle_selected(index) {
            return;
        }

        self.notify.row_changed(index);
    }

    fn get_all_profiles(&self) -> Vec<ProfileItem> {
        self.repo.get_all_profiles()
    }

    fn selected_profile_count(&self) -> usize {
        self.repo.selected_profile_count()
    }
}

impl Model for ProfilesModel {
    type Data = ProfileItem;

    fn row_count(&self) -> usize {
        self.repo.profile_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_profile(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}
