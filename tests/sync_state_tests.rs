#[cfg(test)]
mod tests {
    use rustybox::sync_state::SyncState;
    use std::path::PathBuf;

    #[test]
    fn test_update_file() {
        let mut state = SyncState::new();
        let path = PathBuf::from("test_file.txt");
        let checksum = String::from("dummy_checksum");

        state.update_file(path.clone(), checksum.clone());

        let file_meta = state.files.get(&path).unwrap();
        assert_eq!(file_meta.path, path);
        assert_eq!(file_meta.checksum, checksum);
    }
}
