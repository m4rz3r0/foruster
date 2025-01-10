// SPDX-License-Identifier: GPL-3.0-or-later
use infer::{is_app, is_archive, is_audio, is_book, is_document, is_image, is_video};
use serde::{Deserialize, Serialize};

use crate::{match_extensions, FilterOptions};

use super::FileEntry;

#[derive(Debug, Clone, PartialEq)]
pub enum ProfileType {
    Image,
    Video,
    Audio,
    Archive,
    Book,
    Document,
    Application,
    Custom,
}

impl<'de> Deserialize<'de> for ProfileType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "image" => Ok(ProfileType::Image),
            "video" => Ok(ProfileType::Video),
            "audio" => Ok(ProfileType::Audio),
            "archive" => Ok(ProfileType::Archive),
            "book" => Ok(ProfileType::Book),
            "document" => Ok(ProfileType::Document),
            "application" => Ok(ProfileType::Application),
            "custom" => Ok(ProfileType::Custom),
            _ => Err(serde::de::Error::custom("Invalid profile type")),
        }
    }
}

impl Serialize for ProfileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Self::Image => "image",
            Self::Video => "video",
            Self::Audio => "audio",
            Self::Archive => "archive",
            Self::Book => "book",
            Self::Document => "document",
            Self::Application => "application",
            Self::Custom => "custom",
        };

        serializer.serialize_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(skip)]
    id: usize,
    name: String,
    #[serde(rename = "type")]
    profile_type: ProfileType,
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(skip)]
    files: Vec<FileEntry>,
    #[serde(skip)]
    filter_options: Option<FilterOptions>,
}

impl Profile {
    pub fn new(
        id: usize,
        name: String,
        extensions: Vec<String>,
        profile_type: ProfileType,
    ) -> Self {
        Self {
            id,
            name,
            profile_type,
            extensions,
            files: Vec::new(),
            filter_options: None,
        }
    }

    pub fn matches(&self, file_entry: &FileEntry) -> bool {
        let magic_bytes = file_entry.magic_bytes();

        match self.profile_type {
            ProfileType::Image => is_image(magic_bytes),
            ProfileType::Video => is_video(magic_bytes),
            ProfileType::Audio => is_audio(magic_bytes),
            ProfileType::Archive => is_archive(magic_bytes),
            ProfileType::Book => is_book(magic_bytes),
            ProfileType::Document => is_document(magic_bytes),
            ProfileType::Application => is_app(magic_bytes),
            ProfileType::Custom => self
                .extensions
                .iter()
                .any(|ext| matches!(match_extensions(ext, magic_bytes), Some(true))),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn profile_type(&self) -> &ProfileType {
        &self.profile_type
    }

    pub fn extensions(&self) -> &Vec<String> {
        &self.extensions
    }

    pub fn files(&self) -> &Vec<FileEntry> {
        &self.files
    }

    pub fn filter_options(&self) -> Option<&FilterOptions> {
        self.filter_options.as_ref()
    }

    pub fn set_filter_options(&mut self, options: &FilterOptions) {
        self.filter_options = Some(options.clone());
    }

    pub fn files_mut(&mut self) -> &mut Vec<FileEntry> {
        &mut self.files
    }

    pub fn set_files(&mut self, files: Vec<FileEntry>) {
        self.files = files;
    }

    pub fn add_file(&mut self, file: &FileEntry) {
        self.files.push(file.clone());
    }

    pub fn filter_files(&mut self) {
        if let Some(options) = &self.filter_options {
            self.files.retain(|f| options.filter(f));
        }
    }
}
