#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn init_should_create_db_file_if_not_exists() {
        // Should create the SQLite database file if it doesn't exist.
        let db_path = "target/test_dbs/init_should_create_db_file_if_not_exists.db";
        let path = std::path::Path::new(db_path);
        // Ensure the directory exists
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        // Ensure the file does not exist before the test
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let catalog = Catalog::new(db_path);
        let result = catalog.init();
        assert!(result.is_ok());
        assert!(path.exists());
        // Clean up the created file
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn init_should_not_fail_if_db_file_exists() {
        // Should not fail if the database file already exists.
        let db_path = "target/test_dbs/init_should_not_fail_if_db_file_exists.db";
        let path = std::path::Path::new(db_path);
        // Ensure the directory exists
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        // Create the DB file first
        let catalog = Catalog::new(db_path);
        catalog.init().unwrap();
        // Calling init() again should not fail
        let result = catalog.init();
        assert!(result.is_ok());
        assert!(path.exists());
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn init_should_return_error_for_invalid_path() {
        // Should return an error for an invalid path or permissions issue.
        // Using a directory as a path should fail.
        let invalid_path = "target/test_dbs/an_invalid_path_dir";
        std::fs::create_dir_all(invalid_path).unwrap();
        let catalog = Catalog::new(invalid_path);
        let result = catalog.init();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FailedTo::InitDatabase);
        // Clean up
        std::fs::remove_dir_all(invalid_path).unwrap();
    }
}
