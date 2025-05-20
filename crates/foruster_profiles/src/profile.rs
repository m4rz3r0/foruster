// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum FileCategory {
    Application,
    Audio,
    Image,
    Model,
    Text,
    Video,
    Other,
}

impl From<String> for FileCategory {
    fn from(mime_type: String) -> Self {
        match mime_type.as_str() {
            "application" => FileCategory::Application,
            "audio" => FileCategory::Audio,
            "image" => FileCategory::Image,
            "model" => FileCategory::Model,
            "text" => FileCategory::Text,
            "video" => FileCategory::Video,
            _ => FileCategory::Other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    name: String,
    categories: Option<Vec<FileCategory>>,
    mime_types: Option<Vec<String>>,
    extensions: Option<Vec<String>>,
}

impl Profile {
    pub fn new(
        name: String,
        categories: Option<Vec<FileCategory>>,
        mime_types: Option<Vec<String>>,
        extensions: Option<Vec<String>>,
    ) -> Self {
        Profile {
            name,
            categories,
            mime_types,
            extensions,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn categories(&self) -> &Option<Vec<FileCategory>> {
        &self.categories
    }

    pub fn mime_types(&self) -> &Option<Vec<String>> {
        &self.mime_types
    }

    pub fn extensions(&self) -> &Option<Vec<String>> {
        &self.extensions
    }

    pub fn matches(&self, path: &Path) -> bool {
        if let Some(exts) = &self.extensions {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext = format!(".{}", ext.to_lowercase());
                if exts.contains(&ext) {
                    return true;
                }
            }
        }

        if let Some(mime) = tree_magic_mini::from_filepath(path) {
            if let Some(mimes) = &self.mime_types {
                if mimes.contains(&mime.to_string()) {
                    return true;
                }
            }

            if let Some(file_cat) = mime.split('/').next() {
                let cat = FileCategory::from(file_cat.to_string());
                if let Some(cats) = &self.categories {
                    if cats.contains(&cat) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub fn default_profiles() -> Vec<Profile> {
    vec![
        Profile::new(
            "Aplicaciones".to_string(),
            Some(vec![FileCategory::Application]),
            None,
            None,
        ),
        Profile::new(
            "Audios".to_string(),
            Some(vec![FileCategory::Audio]),
            None,
            None,
        ),
        Profile::new(
            "Imágenes".to_string(),
            Some(vec![FileCategory::Image]),
            None,
            None,
        ),
        Profile::new(
            "Modelos".to_string(),
            Some(vec![FileCategory::Model]),
            None,
            None,
        ),
        Profile::new(
            "Texto".to_string(),
            Some(vec![FileCategory::Text]),
            None,
            None,
        ),
        Profile::new(
            "Vídeos".to_string(),
            Some(vec![FileCategory::Video]),
            None,
            None,
        ),
        Profile::new(
            "Otros".to_string(),
            Some(vec![FileCategory::Other]),
            None,
            None,
        ),
        Profile::new(
            "Archivos comprimidos".to_string(),
            None,
            Some(vec![
                "application/zip".to_string(),
                "application/x-zip-compressed".to_string(),
                "application/x-zip".to_string(),
                "application/x-compress".to_string(),
                "application/x-compressed".to_string(),
                "application/gzip".to_string(),
                "application/x-gzip".to_string(),
                "application/x-tar".to_string(),
                "application/x-bzip2".to_string(),
                "application/x-bzip".to_string(),
            ]),
            Some(vec![
                ".zip".to_string(),
                ".tar.gz".to_string(),
                ".tar.bz2".to_string(),
            ]),
        ),
        Profile::new(
            "Documentos".to_string(),
            None,
            Some(vec![
                "application/pdf".to_string(),
                "application/msword".to_string(),
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                    .to_string(),
                "application/vnd.ms-excel".to_string(),
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
                "application/vnd.ms-powerpoint".to_string(),
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                    .to_string(),
            ]),
            Some(vec![
                ".pdf".to_string(),
                ".doc".to_string(),
                ".docx".to_string(),
                ".xls".to_string(),
                ".xlsx".to_string(),
                ".ppt".to_string(),
                ".pptx".to_string(),
            ]),
        ),
    ]
}
