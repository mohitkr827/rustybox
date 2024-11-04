// src/file_meta.rs
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileMeta {
    pub path: PathBuf,
    pub checksum: String,
}


