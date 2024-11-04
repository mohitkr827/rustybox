use std::collections::HashMap;
use std::path::PathBuf;
use crate::file_meta::FileMeta;

#[derive(Debug, Clone)]
pub struct SyncState {
    pub files: HashMap<PathBuf, FileMeta>, // Make the field public
}

impl SyncState {
    pub fn new() -> Self {
        SyncState {
            files: HashMap::new(),
        }
    }

    pub fn update_file(&mut self, path: PathBuf, checksum: String) {
        self.files.insert(
            path.clone(),
            FileMeta {
                path,
                checksum,
            },
        );
    }
}
