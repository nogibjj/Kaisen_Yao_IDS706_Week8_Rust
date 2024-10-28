#[cfg(test)]
mod tests {
    use rust_proj::*;
    use std::path::Path;
    #[test]
    fn test_load_with_default_dataset() {
        // Test using default dataset
        let result = load("");
        assert!(result.is_ok());

        // Verify that the database file was created
        let db_path = Path::new("../data/US_births_DB.db");
        assert!(db_path.exists());
    }

    #[test]
    fn test_load_with_custom_dataset() {
        // Test using custom dataset path
        let result = load("../data/US_births.csv");
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_with_invalid_path() {
        // Test using invalid dataset path
        let result = load("invalid/path/to/file.csv");
        assert!(result.is_err());
    }

    #[test]
    fn test_database_path_returned() {
        // Test that the returned database path is correct
        if let Ok(path) = load("") {
            assert_eq!(path, "../data/US_births_DB.db");
        }
    }
}
