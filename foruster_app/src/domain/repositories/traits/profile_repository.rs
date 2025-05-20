// SPDX-License-Identifier: GPL-3.0-or-later
use crate::domain::ProfileItem;

pub trait ProfileRepository {
    fn get_profile(&self, index: usize) -> Option<ProfileItem>;
    fn get_all_profiles(&self) -> Vec<ProfileItem>;
    fn clear_profiles(&self);

    fn selected_profile_count(&self) -> usize;
}
