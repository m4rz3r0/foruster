// SPDX-License-Identifier: GPL-3.0-or-later
use std::{collections::HashMap, path::PathBuf};

#[derive(PartialEq, Eq, Hash)]
pub enum HashFunction {
    Md5,
    Sha1,
    Sha256,
    Blake3,
}

pub struct FileEntry {
    path: PathBuf,
    size: usize,
    hash: HashMap<HashFunction, String>,
}

impl FileEntry {
    pub fn new(path: PathBuf, size: usize) -> Self {
        Self {
            path,
            size,
            hash: HashMap::new(),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn hash(&self, hash_function: &HashFunction) -> Option<&String> {
        self.hash.get(hash_function)
    }

    pub fn add_hash(&mut self, hash_function: HashFunction, hash_result: String) {
        self.hash.insert(hash_function, hash_result);
    }
}
