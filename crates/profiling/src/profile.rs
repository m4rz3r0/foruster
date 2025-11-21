// SPDX-License-Identifier: GPL-3.0-or-later
use crate::profile_style::{IconSource, ProfileStyle};
use file_format::FileFormat;
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
    profile_style: ProfileStyle,
}

impl Profile {
    pub fn new(
        name: String,
        categories: Option<Vec<FileCategory>>,
        mime_types: Option<Vec<String>>,
        extensions: Option<Vec<String>>,
        bg_color: u32,
        icon_source: IconSource,
    ) -> Self {
        Profile {
            name,
            categories,
            mime_types,
            extensions,
            profile_style: ProfileStyle::new(bg_color, icon_source),
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

    pub fn bg_color(&self) -> u32 {
        self.profile_style.bg_color()
    }

    pub fn icon_source(&self) -> &IconSource {
        self.profile_style.icon_source()
    }

    pub fn matches(&self, path: &Path, format_opt: Option<&FileFormat>) -> bool {
        // First, check by extension, as it's the fastest.
        if let Some(exts) = &self.extensions {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext = format!(".{}", ext.to_lowercase());
                if exts.contains(&ext) {
                    return true;
                }
            }
        }

        // If we have pre-calculated format, use it for mime/category matching.
        if let Some(format) = format_opt {
            let mime = format.media_type();
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
        } else {
            // Fallback to reading the file if format was not provided
            // This maintains compatibility but is less efficient.
            if let Ok(format) = FileFormat::from_file(path) {
                let mime = format.media_type();
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
        }

        false
    }
}

pub fn default_profiles() -> Vec<Profile> {
    vec![
        Profile::new(
            "Aplicaciones".to_string(),
            Some(vec![FileCategory::Application]),
            Some(vec![
                "application/vnd.microsoft.portable-executable".to_string(),
                "application/x-msdownload".to_string(),
                "application/x-msi".to_string(),
                "application/java-archive".to_string(),
                "application/vnd.appimage".to_string(),
                "application/x-executable".to_string(),
                "application/vnd.debian.binary-package".to_string(),
                "application/x-rpm".to_string(),
                "application/x-apple-diskimage".to_string(),
            ]),
            None,
            0xFF4CAF50,
            IconSource::App,
        ),
        Profile::new(
            "Audios".to_string(),
            Some(vec![FileCategory::Audio]),
            Some(vec![
                "audio/mpeg".to_string(),
                "audio/wav".to_string(),
                "audio/flac".to_string(),
                "audio/aac".to_string(),
                "audio/ogg".to_string(),
                "audio/mp4".to_string(),
                "audio/x-m4a".to_string(),
                "audio/x-ms-wma".to_string(),
            ]),
            None,
            0xFF2196F3,
            IconSource::Audio,
        ),
        Profile::new(
            "Imágenes".to_string(),
            Some(vec![FileCategory::Image]),
            Some(vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "image/bmp".to_string(),
                "image/svg+xml".to_string(),
                "image/webp".to_string(),
                "image/tiff".to_string(),
                "image/vnd.microsoft.icon".to_string(),
            ]),
            None,
            0xFFFF9800,
            IconSource::Image,
        ),
        Profile::new(
            "Modelos".to_string(),
            Some(vec![FileCategory::Model]),
            Some(vec![
                "model/obj".to_string(),
                "model/stl".to_string(),
                "model/gltf+json".to_string(),
                "model/gltf-binary".to_string(),
                "model/vnd.collada+xml".to_string(),
                "application/x-3ds".to_string(),
            ]),
            None,
            0xFF9C27B0,
            IconSource::Model,
        ),
        Profile::new(
            "Texto".to_string(),
            Some(vec![FileCategory::Text]),
            Some(vec![
                "text/plain".to_string(),
                "text/html".to_string(),
                "text/css".to_string(),
                "text/javascript".to_string(),
                "application/json".to_string(),
                "application/xml".to_string(),
                "text/xml".to_string(),
                "text/markdown".to_string(),
                "application/rtf".to_string(),
            ]),
            None,
            0xFF607D8B,
            IconSource::Text,
        ),
        Profile::new(
            "Vídeos".to_string(),
            Some(vec![FileCategory::Video]),
            Some(vec![
                "video/mp4".to_string(),
                "video/webm".to_string(),
                "video/ogg".to_string(),
                "video/x-msvideo".to_string(), // AVI
                "video/quicktime".to_string(), // MOV
                "video/x-matroska".to_string(), // MKV
                "video/x-ms-wmv".to_string(),   // WMV
            ]),
            None,
            0xFFF44336,
            IconSource::Video,
        ),
        Profile::new(
            "Otros".to_string(),
            Some(vec![FileCategory::Other]),
            None,
            None,
            0xFF9E9E9E,
            IconSource::App,
        ),
        Profile::new(
            "Comprimidos".to_string(),
            None,
            Some(vec![
                "application/zip".to_string(),
                "application/x-zip-compressed".to_string(),
                "application/x-7z-compressed".to_string(),
                "application/vnd.rar".to_string(),
                "application/x-rar-compressed".to_string(),
                "application/x-tar".to_string(),
                "application/gzip".to_string(),
                "application/x-bzip2".to_string(),
            ]),
            Some(vec![
                ".zip".to_string(),
                ".rar".to_string(),
                ".7z".to_string(),
                ".tar".to_string(),
                ".gz".to_string(),
                ".tar.gz".to_string(),
                ".bz2".to_string(),
                ".tar.bz2".to_string(),
            ]),
            0xFF795548,
            IconSource::App,
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
                "application/vnd.oasis.opendocument.text".to_string(),
            ]),
            Some(vec![
                ".pdf".to_string(),
                ".doc".to_string(),
                ".docx".to_string(),
                ".xls".to_string(),
                ".xlsx".to_string(),
                ".ppt".to_string(),
                ".pptx".to_string(),
                ".odt".to_string(),
            ]),
            0xFF3F51B5,
            IconSource::Text,
        ),
    ]
}