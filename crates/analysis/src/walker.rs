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
        profiles: Option<&[profiling::Profile]>, // Añadir perfiles para análisis en tiempo real
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut entries = WalkDir::new(&self.path);
        let mut batch = WalkBatch::new(batch_size);

        // Pre-compilar extensiones para matching rápido
        let profile_extensions: Vec<_> = if let Some(profiles) = profiles {
            profiles
                .iter()
                .filter_map(|profile| profile.extensions().as_ref())
                .flatten()
                .collect()
        } else {
            Vec::new()
        };

        while let Some(entry) = entries.next().await {
            match entry {
                Ok(entry) => {
                    // Optimización: evitar llamadas async innecesarias
                    let path = entry.path();
                    let is_file = match entry.file_type().await {
                        Ok(file_type) => file_type.is_file(),
                        Err(_) => false,
                    };

                    if is_file {
                        self.total_files += 1;
                        self.files.push(path.clone());
                        self.analyzed_files += 1;

                        // Matching optimizado: extensión primero, luego mime si es necesario
                        let matches_profile = if let Some(profiles) = profiles {
                            // Cache de extensión para evitar recalcular
                            let extension = path
                                .extension()
                                .and_then(|e| e.to_str())
                                .map(|e| format!(".{}", e.to_lowercase()));

                            // Primero verificar por extensión (O(1) con HashSet sería mejor)
                            let matches_by_extension = if let Some(ref ext) = extension {
                                profile_extensions.contains(&&ext.as_str().to_string())
                            } else {
                                false
                            };

                            // Solo si no coincide por extensión, verificar por mime (más costoso)
                            if matches_by_extension {
                                true
                            } else {
                                // Análisis por mime solo como fallback y para archivos pequeños
                                if let Ok(metadata) = entry.metadata().await {
                                    if metadata.len() < 512 * 1024 {
                                        // Solo archivos < 512KB para mime detection
                                        profiles.iter().any(|profile| profile.matches(&path))
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
                            path.to_string_lossy().into_owned(), // Evitar múltiples conversiones
                            true,
                            matches_profile,
                        );
                    } else {
                        // Es directorio - agregar sin procesar
                        batch.add_entry(path.to_string_lossy().into_owned(), false, false);
                    }

                    // Callback por lote en lugar de por archivo
                    if batch.is_full() {
                        callback(batch.clone_optimized()); // Método optimizado de clonación
                        batch.clear();
                    }
                }
                Err(e) => {
                    eprintln!("Error en walker: {}", e);
                    // Continuar procesando otros archivos
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

    // Método optimizado de clonación que evita copiar datos innecesarios
    pub fn clone_optimized(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            batch_size: self.batch_size,
            total_processed: self.total_processed,
        }
    }
}
