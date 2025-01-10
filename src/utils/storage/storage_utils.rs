// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::Path;

use tokio::io::AsyncReadExt;

pub fn mb_to_bytes(size: f32) -> usize {
    (size * 1024.0 * 1024.0).round() as usize
}

pub fn bytes_to_mb(size: usize) -> f32 {
    format!("{:.2}", size as f64 / 1024.0 / 1024.0)
        .parse()
        .unwrap()
}

pub fn format_size(size: usize) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut size = size as f64;
    let mut i = 0;
    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }
    format!("{:.2} {}", size, units[i])
}

pub async fn read_magic_bytes(file_path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let file = tokio::fs::File::open(file_path).await?;

    let limit = std::cmp::min(file.metadata().await?.len(), 8192) as usize + 1;

    let mut bytes = Vec::with_capacity(limit);
    file.take(8192).read_to_end(&mut bytes).await?;

    Ok(bytes)
}
