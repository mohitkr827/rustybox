// src/file_meta.rs
use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileMeta {
    pub path: PathBuf,
    pub checksum: String,
}


