// SPDX-License-Identifier: GPL-3.0-or-later
use async_walkdir::WalkDir;
use file_format::{FileFormat, Kind};
use futures_lite::stream::StreamExt;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::finding::SuspicionReason;

// A comprehensive map of common file extensions to their expected file "Kind".
// This is now used for BOTH content-mismatch and deceptive filename detection.
static EXT_TO_KIND_MAP: Lazy<HashMap<&'static str, Kind>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Executable
    m.insert("exe", Kind::Executable);
    m.insert("dll", Kind::Executable);
    m.insert("so", Kind::Executable);
    m.insert("elf", Kind::Executable);
    m.insert("com", Kind::Executable);
    m.insert("bat", Kind::Executable);
    m.insert("cmd", Kind::Executable);
    m.insert("scr", Kind::Executable);
    m.insert("msi", Kind::Executable);
    m.insert("jar", Kind::Executable);
    m.insert("sh", Kind::Executable);
    m.insert("ps1", Kind::Executable);
    // Archive
    m.insert("7z", Kind::Archive);
    m.insert("zip", Kind::Archive);
    m.insert("rar", Kind::Archive);
    m.insert("tar", Kind::Archive);
    m.insert("gz", Kind::Archive);
    m.insert("bz2", Kind::Archive);
    m.insert("xz", Kind::Archive);
    m.insert("zst", Kind::Archive);
    m.insert("deb", Kind::Archive);
    m.insert("rpm", Kind::Archive);
    // Audio
    m.insert("mp3", Kind::Audio);
    m.insert("wav", Kind::Audio);
    m.insert("flac", Kind::Audio);
    m.insert("ogg", Kind::Audio);
    m.insert("m4a", Kind::Audio);
    m.insert("aac", Kind::Audio);
    // Document
    m.insert("pdf", Kind::Document);
    m.insert("doc", Kind::Document);
    m.insert("docx", Kind::Document);
    m.insert("odt", Kind::Document);
    m.insert("rtf", Kind::Document);
    m.insert("xls", Kind::Spreadsheet);
    m.insert("xlsx", Kind::Spreadsheet);
    m.insert("ods", Kind::Spreadsheet);
    m.insert("ppt", Kind::Presentation);
    m.insert("pptx", Kind::Presentation);
    m.insert("odp", Kind::Presentation);
    // Image
    m.insert("jpg", Kind::Image);
    m.insert("jpeg", Kind::Image);
    m.insert("png", Kind::Image);
    m.insert("gif", Kind::Image);
    m.insert("bmp", Kind::Image);
    m.insert("tif", Kind::Image);
    m.insert("tiff", Kind::Image);
    m.insert("webp", Kind::Image);
    m.insert("heic", Kind::Image);
    m.insert("heif", Kind::Image);
    m.insert("ico", Kind::Image);
    m.insert("psd", Kind::Image);
    // Video
    m.insert("mp4", Kind::Video);
    m.insert("mkv", Kind::Video);
    m.insert("mov", Kind::Video);
    m.insert("avi", Kind::Video);
    m.insert("wmv", Kind::Video);
    m.insert("webm", Kind::Video);
    m.insert("flv", Kind::Video);
    m
});

/// **NEW**: Checks for deceptive filename patterns like "file.jpg.exe" or "archive.zip.txt".
/// This detects attempts to hide a file's true nature from users who might have
/// "hide known extensions" enabled.
fn has_suspicious_extension_pattern(path: &Path) -> Option<String> {
    let filename = match path.file_name().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => return None,
    };
    let parts: Vec<&str> = filename.split('.').collect();
    if parts.len() < 3 {
        return None;
    }
    let middle_ext = parts[parts.len() - 2].to_lowercase();
    if EXT_TO_KIND_MAP.contains_key(middle_ext.as_str()) {
        println!(
            "Suspicious extension pattern detected: {:?} (hidden extension '.{}')",
            path, middle_ext
        );
        return Some(middle_ext);
    }
    None
}


/// **REVISED**: Determines if a file is suspicious using a two-pronged approach.
fn is_file_suspicious(path: &Path, format_result: &Result<FileFormat, std::io::Error>) -> Option<SuspicionReason> {
    if let Some(hidden_ext) = has_suspicious_extension_pattern(path) {
        return Some(SuspicionReason::DeceptiveExtension { hidden_ext });
    }

    let detected_kind = match format_result {
        Ok(fmt) => fmt.kind(),
        Err(_) => return None,
    };

    let expected_kind_opt = path
        .extension()
        .and_then(|s| s.to_str())
        .and_then(|ext| EXT_TO_KIND_MAP.get(ext.to_lowercase().as_str()));

    let expected_kind = match expected_kind_opt {
        Some(kind) => kind,
        None => return None,
    };

    if detected_kind != *expected_kind {
        let is_office_archive_exception = (*expected_kind == Kind::Document
            || *expected_kind == Kind::Spreadsheet
            || *expected_kind == Kind::Presentation)
            && detected_kind == Kind::Archive;

        if is_office_archive_exception {
            return None;
        }

        println!(
            "Suspicious content mismatch detected: {:?}, extension implies {:?}, but content is {:?}",
            path, expected_kind, detected_kind
        );
        return Some(SuspicionReason::ContentMismatch { expected: *expected_kind, actual: detected_kind });
    }

    None
}


pub struct Walker {
    path: PathBuf,
    files: Vec<PathBuf>,
    total_files: usize,
    analyzed_files: usize,
}

impl Walker {
    pub fn new<P: Into<PathBuf>>(path: P) -> Walker {
        let path = path.into();

        Self {
            path,
            files: Vec::new(),
            total_files: 0,
            analyzed_files: 0,
        }
    }

    // ... (rest of Walker impl functions: files(), total_files(), analyzed_files(), start() remain unchanged)

    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }

    pub fn total_files(&self) -> usize {
        self.total_files
    }

    pub fn analyzed_files(&self) -> usize {
        self.analyzed_files
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut entries = WalkDir::new(&self.path);

        while let Some(entry) = entries.next().await {
            match entry {
                Ok(entry) => {
                    match entry.file_type().await {
                        Ok(file_type) if file_type.is_file() => {
                            self.total_files += 1; // Contar todos los archivos
                            self.files.push(entry.path());
                            self.analyzed_files += 1; // Solo incrementar para archivos procesados exitosamente
                        }
                        Ok(_) => {
                            // Es un directorio u otro tipo, no contar
                        }
                        Err(e) => {
                            eprintln!(
                                "Error obteniendo tipo de archivo para {:?}: {}",
                                entry.path(),
                                e
                            );
                            self.total_files += 1; // Contar archivo aunque haya error
                            // No incrementar analyzed_files para archivos con error
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error accediendo a entrada: {}", e);
                    // No podemos determinar si es archivo o directorio, no contar
                }
            }
        }

        Ok(())
    }

    pub async fn start_with_batch_callback(
        &mut self,
        callback: Box<dyn Fn(WalkBatch) + Send + Sync>,
        batch_size: usize,
        profiles: Option<&[profiling::Profile]>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut entries = WalkDir::new(&self.path);
        let mut batch = WalkBatch::new(batch_size);

        while let Some(entry) = entries.next().await {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    let is_file = match entry.file_type().await {
                        Ok(file_type) => file_type.is_file(),
                        Err(_) => false,
                    };

                    if is_file {
                        self.total_files += 1;
                        self.files.push(path.clone());
                        self.analyzed_files += 1;

                        // Read the file format ONCE for both suspicious check and profile matching.
                        let format_result = FileFormat::from_file(&path);
                        let suspicion_reason = is_file_suspicious(&path, &format_result);
                        let format_opt = format_result.ok();

                        let matched_profiles: Vec<String> = if let Some(profiles) = profiles {
                            profiles
                                .iter()
                                .filter(|profile| profile.matches(&path, format_opt.as_ref()))
                                .map(|profile| profile.name().clone())
                                .collect()
                        } else {
                            Vec::new()
                        };

                        batch.add_entry(
                            path.to_string_lossy().into_owned(),
                            true,
                            matched_profiles,
                            suspicion_reason,
                        );
                    } else {
                        batch.add_entry(path.to_string_lossy().into_owned(), false, Vec::new(), None);
                    }

                    if batch.is_full() {
                        callback(batch.clone_optimized());
                        batch.clear();
                    }
                }
                Err(e) => {
                    eprintln!("Error in walker: {}", e);
                }
            }
        }

        if !batch.is_empty() {
            callback(batch);
        }

        Ok(())
    }
}

// ... (WalkBatch and WalkEntry structs remain exactly the same)

#[derive(Clone, Debug)]
pub struct WalkBatch {
    pub entries: Vec<WalkEntry>,
    pub batch_size: usize,
    pub total_processed: usize,
}

#[derive(Clone, Debug)]
pub struct WalkEntry {
    pub path: String,
    pub is_file: bool,
    pub matched_profiles: Vec<String>,
    pub suspicion_reason: Option<SuspicionReason>,
}

impl WalkBatch {
    pub fn new(batch_size: usize) -> Self {
        Self {
            entries: Vec::with_capacity(batch_size),
            batch_size,
            total_processed: 0,
        }
    }

    pub fn add_entry(&mut self, path: String, is_file: bool, matched_profiles: Vec<String>, suspicion_reason: Option<SuspicionReason>) {
        self.entries.push(WalkEntry {
            path,
            is_file,
            matched_profiles,
            suspicion_reason // changed from is_suspicious
        });
        self.total_processed += 1;
    }

    pub fn is_full(&self) -> bool {
        self.entries.len() >= self.batch_size
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }



    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn files_count(&self) -> usize {
        self.entries.iter().filter(|e| e.is_file).count()
    }

    pub fn matched_files_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.is_file && !e.matched_profiles.is_empty()) // Check if the list is not empty
            .count()
    }

    pub fn suspicious_files_count(&self) -> usize {
        self.entries.iter().filter(|e| e.suspicion_reason.is_some()).count()
    }


    pub fn clone_optimized(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            batch_size: self.batch_size,
            total_processed: self.total_processed,
        }
    }
}