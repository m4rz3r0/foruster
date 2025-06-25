// SPDX-License-Identifier: GPL-3.0-or-later
use std::{collections::HashMap, hash::Hasher, path::PathBuf};

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
    hash: HashMap<HashFunction, Box<dyn Hasher>>,
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

    pub fn hash(&self, hash_function: &HashFunction) -> Option<&dyn Hasher> {
        self.hash.get(hash_function).map(|b| b.as_ref())
    }

    pub fn add_hash(&mut self, hash_function: HashFunction, hasher: Box<dyn Hasher>) {
        self.hash.insert(hash_function, hasher);
    }
}
