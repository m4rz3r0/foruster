// SPDX-License-Identifier: GPL-3.0-or-later
mod profile;
pub use profile::{default_profiles, FileCategory, Profile};

#[cfg(test)]
mod tests {
    use crate::profile::default_profiles;
    use crate::profile::FileCategory;

    #[test]
    fn test_profile() {
        let profiles = default_profiles();
        assert_eq!(profiles.len(), 9);
        assert_eq!(profiles[0].name(), "Aplicaciones");
        assert_eq!(
            *profiles[0].categories(),
            Some(vec![FileCategory::Application])
        );
        assert_eq!(profiles[1].name(), "Audios");
        assert_eq!(*profiles[1].categories(), Some(vec![FileCategory::Audio]));
        assert_eq!(profiles[2].name(), "Imágenes");
        assert_eq!(*profiles[2].categories(), Some(vec![FileCategory::Image]));
        assert_eq!(profiles[3].name(), "Modelos");
        assert_eq!(*profiles[3].categories(), Some(vec![FileCategory::Model]));
        assert_eq!(profiles[4].name(), "Texto");
        assert_eq!(*profiles[4].categories(), Some(vec![FileCategory::Text]));
        assert_eq!(profiles[5].name(), "Vídeos");
        assert_eq!(*profiles[5].categories(), Some(vec![FileCategory::Video]));
        assert_eq!(profiles[6].name(), "Otros");
        assert_eq!(*profiles[6].categories(), Some(vec![FileCategory::Other]));

        assert_eq!(profiles[7].name(), "Archivos comprimidos");
        assert_eq!(*profiles[7].categories(), None);
        assert_eq!(
            *profiles[7].mime_types(),
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
                "application/x-bzip".to_string()
            ])
        );
        assert_eq!(
            *profiles[7].extensions(),
            Some(vec![
                ".zip".to_string(),
                ".tar.gz".to_string(),
                ".tar.bz2".to_string()
            ])
        );

        assert_eq!(profiles[8].name(), "Documentos");
        assert_eq!(*profiles[8].categories(), None);
        assert_eq!(
            *profiles[8].mime_types(),
            Some(vec![
                "application/pdf".to_string(),
                "application/msword".to_string(),
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                    .to_string(),
                "application/vnd.ms-excel".to_string(),
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
                "application/vnd.ms-powerpoint".to_string(),
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                    .to_string()
            ])
        );
        assert_eq!(
            *profiles[8].extensions(),
            Some(vec![
                ".pdf".to_string(),
                ".doc".to_string(),
                ".docx".to_string(),
                ".xls".to_string(),
                ".xlsx".to_string(),
                ".ppt".to_string(),
                ".pptx".to_string()
            ])
        );
    }
}
