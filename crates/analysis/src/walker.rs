// SPDX-License-Identifier: GPL-3.0-or-later
use async_walkdir::WalkDir;
use futures_lite::stream::StreamExt;
use std::path::PathBuf;

pub struct Walker {
    path: PathBuf,
    files: Vec<PathBuf>,
    total_files: usize,    // Todos los archivos encontrados (incluyendo errores)
    analyzed_files: usize, // Solo archivos procesados exitosamente
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
                            self.files.push(entry.path().into());
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
        profiles: Option<&[profiling::Profile]>, // Añadir perfiles para análisis en tiempo real
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut entries = WalkDir::new(&self.path);
        let mut batch = WalkBatch::new(batch_size);

        while let Some(entry) = entries.next().await {
            match entry {
                Ok(entry) => {
                    match entry.file_type().await {
                        Ok(file_type) if file_type.is_file() => {
                            self.total_files += 1; // Contar todos los archivos
                            self.files.push(entry.path().into());
                            self.analyzed_files += 1; // Solo incrementar para archivos procesados exitosamente

                            // Análisis optimizado - primero por extensión, luego por mime
                            let matches_profile = if let Some(profiles) = profiles {
                                // Cache de extensiones para evitar recalcular
                                let extension = entry
                                    .path()
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .map(|e| format!(".{}", e.to_lowercase()));

                                // Primero verificar por extensión (más rápido)
                                let matches_by_extension = if let Some(ref ext) = extension {
                                    profiles.iter().any(|profile| {
                                        profile
                                            .extensions()
                                            .as_ref()
                                            .map(|exts| exts.contains(ext))
                                            .unwrap_or(false)
                                    })
                                } else {
                                    false
                                };

                                // Solo si no coincide por extensión, verificar por mime (más lento)
                                if matches_by_extension {
                                    true
                                } else {
                                    // Análisis por mime solo como fallback y para archivos pequeños
                                    if let Ok(metadata) = entry.metadata().await {
                                        if metadata.len() < 1024 * 1024 {
                                            // Solo archivos < 1MB para mime detection
                                            profiles
                                                .iter()
                                                .any(|profile| profile.matches(&entry.path()))
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                }
                            } else {
                                false
                            };

                            batch.add_entry(
                                entry.path().to_string_lossy().to_string(),
                                true,
                                matches_profile,
                            );
                        }
                        Ok(_) => {
                            // Es un directorio u otro tipo, no contar
                            batch.add_entry(
                                entry.path().to_string_lossy().to_string(),
                                false,
                                false,
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "Error obteniendo tipo de archivo para {:?}: {}",
                                entry.path(),
                                e
                            );
                            self.total_files += 1; // Contar archivo aunque haya error
                            // No incrementar analyzed_files para archivos con error
                            batch.add_entry(
                                entry.path().to_string_lossy().to_string(),
                                false,
                                false,
                            );
                        }
                    }

                    // Callback por lote en lugar de por archivo
                    if batch.is_full() {
                        callback(batch.clone());
                        batch.clear();
                    }
                }
                Err(e) => {
                    eprintln!("Error en walker: {}", e);
                    // No podemos determinar si es archivo, no contar
                }
            }
        }

        // Procesar último lote si no está vacío
        if !batch.is_empty() {
            callback(batch);
        }

        Ok(())
    }
}

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
    pub matches_profile: bool, // Nuevo campo para indicar si coincide con algún perfil
}

impl WalkBatch {
    pub fn new(batch_size: usize) -> Self {
        Self {
            entries: Vec::with_capacity(batch_size),
            batch_size,
            total_processed: 0,
        }
    }

    pub fn add_entry(&mut self, path: String, is_file: bool, matches_profile: bool) {
        self.entries.push(WalkEntry {
            path,
            is_file,
            matches_profile,
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
            .filter(|e| e.is_file && e.matches_profile)
            .count()
    }
}
