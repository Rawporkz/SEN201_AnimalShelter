//
// file_service/test.rs
//
// This file contains unit tests for the file service module.
//

#[cfg(test)]
mod file_service_tests {
    use crate::file_service::FileService;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    /// Helper function to create a test file service with a dedicated test directory.
    ///
    /// # Arguments
    /// * `test_name` - Name of the test for unique directory path.
    ///
    /// # Returns
    /// * `(FileService, PathBuf)` - A tuple containing the file service and the root path for the test.
    fn create_test_fs(test_name: &str) -> (FileService, PathBuf) {
        // Create a unique path for the test artifacts.
        let mut root_path = PathBuf::from("test_artifacts/file_service");
        root_path.push(test_name);

        // Ensure the directory exists and is clean.
        if root_path.exists() {
            fs::remove_dir_all(&root_path).expect("Failed to remove existing test directory");
        }
        fs::create_dir_all(&root_path).expect("Failed to create test artifacts directory");

        let file_service =
            FileService::new(&root_path).expect("Failed to create test file service");
        (file_service, root_path)
    }

    // Note: The `upload_file` function is not tested here because it requires a running
    // Tauri application handle (`AppHandle`) and user interaction for the file dialog,
    // which are not available in a standard unit test environment.

    #[tokio::test]
    async fn test_delete_file_success() {
        let (file_service, root_path) = create_test_fs("test_delete_file_success");
        let file_path = root_path.join("test_file.txt");

        // Create a dummy file to delete.
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "Hello, test!").expect("Failed to write to test file");
        assert!(file_path.exists());

        // Call the delete function.
        let result = file_service.delete_file(&file_path).await;

        // Assert that the deletion was successful and the file is gone.
        assert!(result.is_ok());
        assert!(!file_path.exists());
    }

    #[tokio::test]
    async fn test_delete_nonexistent_file() {
        let (file_service, root_path) = create_test_fs("test_delete_nonexistent_file");
        let file_path = root_path.join("nonexistent_file.txt");

        // Call the delete function on a nonexistent file.
        let result = file_service.delete_file(&file_path).await;

        // Assert that the operation failed.
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_file_outside_root_fails() {
        let (file_service, _root_path) = create_test_fs("test_delete_file_outside_root_fails");

        // Create a file outside of the service's root directory.
        let mut outside_dir = PathBuf::from("test_artifacts/file_service");
        outside_dir.push("outside_test_dir");
        fs::create_dir_all(&outside_dir).expect("Failed to create outside directory");
        let outside_file_path = outside_dir.join("outside_file.txt");

        let mut file = fs::File::create(&outside_file_path).expect("Failed to create outside file");
        writeln!(file, "I'm outside!").expect("Failed to write to outside file");
        assert!(outside_file_path.exists());

        // Attempt to delete the file using the service.
        let result = file_service.delete_file(&outside_file_path).await;

        // Assert that the operation failed with a security violation.
        assert!(result.is_err());
        if let Some(err) = result.err() {
            assert!(err.to_string().contains("Security violation"));
        }

        // Ensure the file was not deleted.
        assert!(outside_file_path.exists());

        // Clean up the outside file and directory.
        fs::remove_file(&outside_file_path).expect("Failed to clean up outside file");
        fs::remove_dir(&outside_dir).expect("Failed to clean up outside directory");
    }
}

