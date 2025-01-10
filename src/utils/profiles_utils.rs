// SPDX-License-Identifier: GPL-3.0-or-later
use charts_rs::Color;
use dioxus::prelude::*;

use crate::{read_magic_bytes, AppState, FileEntry, Profile, ProfileType, Report, APLLICATION_PROFILE_COLOR, ARCHIVE_PROFILE_COLOR, AUDIO_PROFILE_COLOR, BOOK_PROFILE_COLOR, CUSTOM_PROFILE_COLOR, DEFAULT_PROFILES_URL, DOCUMENT_PROFILE_COLOR, IMAGE_PROFILE_COLOR, VIDEO_PROFILE_COLOR};

pub fn get_profiles() -> Result<Vec<Profile>, std::io::Error> {
    let s = std::fs::read_to_string(format!("dist{}", DEFAULT_PROFILES_URL))?;

    let mut profiles = serde_json::from_str::<Vec<Profile>>(&s)?;
    for (profile_index, profile) in profiles.iter_mut().enumerate() {
        profile.set_id(profile_index);
    }

    Ok(profiles)
}

pub fn get_profile_color(profile_type: &ProfileType) -> Color {
    match profile_type {
        ProfileType::Image => IMAGE_PROFILE_COLOR,
        ProfileType::Video => VIDEO_PROFILE_COLOR,
        ProfileType::Audio => AUDIO_PROFILE_COLOR,
        ProfileType::Archive => ARCHIVE_PROFILE_COLOR,
        ProfileType::Book => BOOK_PROFILE_COLOR,
        ProfileType::Document => DOCUMENT_PROFILE_COLOR,
        ProfileType::Application => APLLICATION_PROFILE_COLOR,
        ProfileType::Custom => CUSTOM_PROFILE_COLOR,
    }
}

pub fn match_extensions(extension: &str, magic_bytes: &[u8]) -> Option<bool> {
    Some(match extension {
        // Application
        "coff" => infer::app::is_coff(magic_bytes),
        "coff_i386" => infer::app::is_coff_i386(magic_bytes),
        "coff_ia64" => infer::app::is_coff_ia64(magic_bytes),
        "coff_x64" => infer::app::is_coff_x64(magic_bytes),
        "der" => infer::app::is_der(magic_bytes),
        "dex" => infer::app::is_dex(magic_bytes),
        "dey" => infer::app::is_dey(magic_bytes),
        "dll" => infer::app::is_dll(magic_bytes),
        "elf" => infer::app::is_elf(magic_bytes),
        "exe" => infer::app::is_exe(magic_bytes),
        "java" => infer::app::is_java(magic_bytes),
        "llvm" => infer::app::is_llvm(magic_bytes),
        "mach" => infer::app::is_mach(magic_bytes),
        "pem" => infer::app::is_pem(magic_bytes),
        "wasm" => infer::app::is_wasm(magic_bytes),

        // Archive
        "7z" => infer::archive::is_7z(magic_bytes),
        "ar" => infer::archive::is_ar(magic_bytes),
        "bz2" => infer::archive::is_bz2(magic_bytes),
        "cab" => infer::archive::is_cab(magic_bytes),
        "cpio" => infer::archive::is_cpio(magic_bytes),
        "crx" => infer::archive::is_crx(magic_bytes),
        "dcm" => infer::archive::is_dcm(magic_bytes),
        "deb" => infer::archive::is_deb(magic_bytes),
        "eot" => infer::archive::is_eot(magic_bytes),
        "gz" => infer::archive::is_gz(magic_bytes),
        "lz" => infer::archive::is_lz(magic_bytes),
        "msi" => infer::archive::is_msi(magic_bytes),
        "nes" => infer::archive::is_nes(magic_bytes),
        "pdf" => infer::archive::is_pdf(magic_bytes),
        "ps" => infer::archive::is_ps(magic_bytes),
        "rar" => infer::archive::is_rar(magic_bytes),
        "rpm" => infer::archive::is_rpm(magic_bytes),
        "rtf" => infer::archive::is_rtf(magic_bytes),
        "sqlite" => infer::archive::is_sqlite(magic_bytes),
        "swf" => infer::archive::is_swf(magic_bytes),
        "tar" => infer::archive::is_tar(magic_bytes),
        "xz" => infer::archive::is_xz(magic_bytes),
        "z" => infer::archive::is_z(magic_bytes),
        "zip" => infer::archive::is_zip(magic_bytes),
        "zst" => infer::archive::is_zst(magic_bytes),

        // Audio
        "aac" => infer::audio::is_aac(magic_bytes),
        "aiff" => infer::audio::is_aiff(magic_bytes),
        "amr" => infer::audio::is_amr(magic_bytes),
        "ape" => infer::audio::is_ape(magic_bytes),
        "dsf" => infer::audio::is_dsf(magic_bytes),
        "flac" => infer::audio::is_flac(magic_bytes),
        "m4a" => infer::audio::is_m4a(magic_bytes),
        "midi" => infer::audio::is_midi(magic_bytes),
        "mp3" => infer::audio::is_mp3(magic_bytes),
        "ogg" => infer::audio::is_ogg(magic_bytes),
        "ogg_opus" => infer::audio::is_ogg_opus(magic_bytes),
        "wav" => infer::audio::is_wav(magic_bytes),

        // Book
        "epub" => infer::book::is_epub(magic_bytes),
        "mobi" => infer::book::is_mobi(magic_bytes),

        // Doc
        "doc" => infer::doc::is_doc(magic_bytes),
        "docx" => infer::doc::is_docx(magic_bytes),
        "ppt" => infer::doc::is_ppt(magic_bytes),
        "pptx" => infer::doc::is_pptx(magic_bytes),
        "xls" => infer::doc::is_xls(magic_bytes),
        "xlsx" => infer::doc::is_xlsx(magic_bytes),

        // Image
        "avif" => infer::image::is_avif(magic_bytes),
        "bmp" => infer::image::is_bmp(magic_bytes),
        "cr2" => infer::image::is_cr2(magic_bytes),
        "djvu" => infer::image::is_djvu(magic_bytes),
        "gif" => infer::image::is_gif(magic_bytes),
        "heif" => infer::image::is_heif(magic_bytes),
        "ico" => infer::image::is_ico(magic_bytes),
        "jpg" | "jpeg" => infer::image::is_jpeg(magic_bytes),
        "jpeg2000" => infer::image::is_jpeg2000(magic_bytes),
        "jxl" => infer::image::is_jxl(magic_bytes),
        "jxr" => infer::image::is_jxr(magic_bytes),
        "ora" => infer::image::is_ora(magic_bytes),
        "png" => infer::image::is_png(magic_bytes),
        "psd" => infer::image::is_psd(magic_bytes),
        "tiff" => infer::image::is_tiff(magic_bytes),
        "webp" => infer::image::is_webp(magic_bytes),

        // ODF
        "odp" => infer::odf::is_odp(magic_bytes),
        "ods" => infer::odf::is_ods(magic_bytes),
        "odt" => infer::odf::is_odt(magic_bytes),

        // Text
        "html" => infer::text::is_html(magic_bytes),
        "shellscript" => infer::text::is_shellscript(magic_bytes),
        "xml" => infer::text::is_xml(magic_bytes),

        // Video
        "avi" => infer::video::is_avi(magic_bytes),
        "flv" => infer::video::is_flv(magic_bytes),
        "m4v" => infer::video::is_m4v(magic_bytes),
        "mkv" => infer::video::is_mkv(magic_bytes),
        "mov" => infer::video::is_mov(magic_bytes),
        "mp4" => infer::video::is_mp4(magic_bytes),
        "mpeg" => infer::video::is_mpeg(magic_bytes),
        "webm" => infer::video::is_webm(magic_bytes),
        "wmv" => infer::video::is_wmv(magic_bytes),

        _ => return None,
    })
}

pub async fn classify_files(mut app_state: Signal<AppState>) {
    use std::time::Instant;
    let now = Instant::now();

    let mut files = app_state.peek().files.clone();
    let profiles = app_state.peek().profiles.clone();
    let mut classifying_progress = app_state.peek().classifying_progress;

    let files_length = files.len() as f32;
    let res = tokio::task::spawn_blocking(move || async move {
        let mut profiles = profiles.clone();
        for (files_index, file) in files.iter_mut().enumerate() {
            let path = file.into_path();
            let magic_bytes = match read_magic_bytes(&path).await {
                Ok(magic_bytes) => {
                    file.set_magic_bytes(&magic_bytes);
                    magic_bytes
                }
                Err(_) => continue,
            };
            if let Some(ext) = file.extension() {
                let check = match_extensions(ext.to_str().unwrap(), &magic_bytes);
                if check.is_some_and(|c| !c) || check.is_none() {
                    file.set_suspicious();
                }
            } else {
                file.set_suspicious();
            }

            for profile in profiles.iter_mut() {
                if profile.matches(file) {
                    profile.add_file(file);
                }
            }

            let progress = (files_index as f32 / files_length * 100.0) as u8;
            if progress - *classifying_progress.peek() >= 1 {
                *classifying_progress.write() = progress;
            }
        }

        profiles
    })
    .await;

    let updated_profiles = match res {
        Ok(result) => result.await,
        Err(_) => Vec::new(),
    };

    app_state.write().profiles = updated_profiles;
    *classifying_progress.write() = 100;

    let elapsed = now.elapsed();
    tracing::info!("Tiempo transcurrido clasificando archivos: {:.4?}", elapsed);
}

pub async fn calculate_hashes(app_state: Signal<AppState>, mut report: Signal<Report>) {
    use std::time::Instant;
    let now = Instant::now();

    let mut hashing_progress = app_state.peek().hashing_progress;
    let mut profiles = report.peek().selected_profiles.clone();

    let files_length = (profiles.iter().fold(0, |acc, x| acc + x.files().len())) as f32;
    let res = tokio::task::spawn_blocking(move || {
        let mut files_index = 0;
        for profile in &mut profiles {
            for file in profile.files_mut() {
                file.calculate_hash();

                let progress = (files_index as f32 / files_length * 100.0) as u8;
                if progress - hashing_progress() >= 1 {
                    *hashing_progress.write() = progress;
                }

                files_index += 1;
            }
        }

        profiles
    })
    .await;

    let updated_profiles = match res {
        Ok(result) => result,
        Err(_) => Vec::new(),
    };

    report.write().selected_profiles = updated_profiles;
    *hashing_progress.write() = 100;
    
    let elapsed = now.elapsed();
    tracing::info!("Tiempo transcurrido generando hashes: {:.4?}", elapsed);
}

pub fn get_suspicious_files(report: Signal<Report>) -> Vec<FileEntry> {
    let mut suspicious_files: Vec<_> = report
        .peek()
        .selected_profiles
        .iter()
        .flat_map(|p| p.files())
        .filter(|f| f.suspicious())
        .cloned()
        .collect();

    suspicious_files.sort_by(|f1, f2| f1.path().cmp(f2.path()));
    suspicious_files.dedup_by(|f1, f2| f1.path().eq(f2.path()));

    suspicious_files
}