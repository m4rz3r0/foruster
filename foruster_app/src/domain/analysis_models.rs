// SPDX-License-Identifier: GPL-3.0-or-later
use serde::{Serialize, Deserialize};

// Ejemplo para resultados de análisis de rutas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathAnalysisResult {
    pub total_files: u64,
    pub total_size: u64,
    // Puedes añadir más campos relevantes aquí
    // pub largest_file: Option<String>,
    // pub file_types_summary: std::collections::HashMap<String, u64>,
}

// Enum para comunicar el estado del análisis a la UI
#[derive(Debug, Clone)]
pub enum AnalysisUpdate<T> {
    Progress { percentage: f32, message: String },
    Completed(T),
    Error(String),
}
