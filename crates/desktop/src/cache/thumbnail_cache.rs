// SPDX-License-Identifier: GPL-3.0-or-later
use once_cell::sync::Lazy;
use slint::{Rgba8Pixel, SharedPixelBuffer};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// A thread-safe representation of a thumbnail's raw data.
#[derive(Clone)]
pub struct CachedThumbnail {
    pub buffer: SharedPixelBuffer<Rgba8Pixel>,
    // Storing width/height isn't strictly necessary as the buffer has them,
    // but it can be convenient.
    // pub width: u32,
    // pub height: u32,
}

// The global, thread-safe cache now stores our thread-safe struct.
static THUMBNAIL_CACHE: Lazy<RwLock<HashMap<PathBuf, CachedThumbnail>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Tries to retrieve a cached thumbnail's raw data for the given path.
pub fn get(path: &Path) -> Option<CachedThumbnail> {
    let cache = THUMBNAIL_CACHE.read().unwrap();
    cache.get(path).cloned()
}

/// Inserts newly generated thumbnail data into the cache.
pub fn insert(path: PathBuf, thumbnail_data: CachedThumbnail) {
    let mut cache = THUMBNAIL_CACHE.write().unwrap();
    cache.insert(path, thumbnail_data);
}