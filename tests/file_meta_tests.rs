#[cfg(test)]
mod tests {
    use rustybox::file_meta::FileMeta;
    use std::path::PathBuf;

    #[test]
    fn test_create_file_meta() {
        let path = PathBuf::from("test_file.txt");
        let checksum = String::from("dummy_checksum");
        let file_meta = FileMeta { path: path.clone(), checksum: checksum.clone() };

        assert_eq!(file_meta.path, path);
        assert_eq!(file_meta.checksum, checksum);
    }
}