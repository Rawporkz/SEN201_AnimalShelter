//
// authentication_service/test.rs
//
// This file contains unit tests for the authentication service module.
//

#[cfg(test)]
mod authentication_service_tests {
    use super::super::AuthenticationService;
    use crate::database_service::{types::UserRole, DatabaseService};
    use std::fs;
    use std::path::PathBuf;

    /// Helper function to create a test database service with proper test artifacts directory
    ///
    /// # Arguments
    /// * `test_name` - Name of the test for unique database path
    ///
    /// # Returns
    /// * `DatabaseService` - A new database service instance with test database
    fn create_test_db(test_name: &str) -> DatabaseService {
        // Create test artifacts directory structure
        let mut db_path = PathBuf::from("test_artifacts/authentication_service");
        db_path.push(test_name);

        // Ensure directory exists
        fs::create_dir_all(&db_path).expect("Failed to create test artifacts directory");

        // Add database filename
        db_path.push("test.db");

        // Remove existing database if it exists
        let _ = fs::remove_file(&db_path);

        DatabaseService::new(db_path).expect("Failed to create test db service")
    }

    #[test]
    fn test_sign_up() {
        let db = create_test_db("test_sign_up_valid_user");
        let mut auth_service = AuthenticationService::new();

        // Test successful sign up
        let result = auth_service.sign_up(&db, "testuser", "password123", UserRole::Customer);
        assert!(result.is_ok());

        // Verify user was created in database
        let password_hash = db.get_password_hash("testuser").unwrap();
        assert!(password_hash.is_some());

        let user_role = db.get_user_role("testuser").unwrap();
        assert_eq!(user_role.unwrap(), UserRole::Customer);
    }

    #[test]
    fn test_sign_up_duplicate_user() {
        let db = create_test_db("test_sign_up_duplicate_user");
        let mut auth_service = AuthenticationService::new();

        // First sign up should succeed
        let result1 = auth_service.sign_up(&db, "testuser", "password123", UserRole::Customer);
        assert!(result1.is_ok());

        // Duplicate sign up should fail
        let result2 = auth_service.sign_up(&db, "testuser", "password456", UserRole::Staff);
        assert!(result2.is_err());
    }

    #[test]
    fn test_sign_up_invalid_input() {
        let db = create_test_db("test_sign_up_invalid_input");
        let mut auth_service = AuthenticationService::new();

        // Empty username should fail
        let result1 = auth_service.sign_up(&db, "", "password123", UserRole::Customer);
        assert!(result1.is_err());
        assert!(result1
            .unwrap_err()
            .to_string()
            .contains("Username cannot be empty"));

        // Whitespace-only username should fail
        let result2 = auth_service.sign_up(&db, "   ", "password123", UserRole::Customer);
        assert!(result2.is_err());
        assert!(result2
            .unwrap_err()
            .to_string()
            .contains("Username cannot be empty"));

        // Short password should fail
        let result3 = auth_service.sign_up(&db, "testuser", "123", UserRole::Customer);
        assert!(result3.is_err());
        assert!(result3
            .unwrap_err()
            .to_string()
            .contains("Password must be at least 6 characters"));
    }

    #[test]
    fn test_log_in_valid_password() {
        let db = create_test_db("test_log_in_valid_credentials");
        let mut auth_service = AuthenticationService::new();

        // Create a user first
        auth_service
            .sign_up(&db, "testuser", "password123", UserRole::Customer)
            .unwrap();

        // Test successful login
        let login_result = auth_service.log_in(&db, "testuser", "password123").unwrap();
        assert!(login_result);

        // Verify user is now logged in
        let current_user = auth_service.get_current_user(&db).unwrap().unwrap();
        assert_eq!(current_user.username, "testuser");
        assert_eq!(current_user.role, UserRole::Customer);
    }

    #[test]
    fn test_log_in_invalid_password() {
        let db = create_test_db("test_log_in_invalid_password");
        let mut auth_service = AuthenticationService::new();

        // Create a user first
        auth_service
            .sign_up(&db, "testuser", "password123", UserRole::Customer)
            .unwrap();

        // Test login with wrong password
        let login_result = auth_service.log_in(&db, "testuser", "wrongpassword").unwrap();
        assert!(!login_result);

        // Verify no user is logged in
        let current_user = auth_service.get_current_user(&db).unwrap();
        assert!(current_user.is_none());
    }

    #[test]
    fn test_log_in_nonexistent_user() {
        let db = create_test_db("test_log_in_nonexistent_user");
        let mut auth_service = AuthenticationService::new();

        // Test login with non-existent username
        let login_result = auth_service.log_in(&db, "nonexistent", "password123").unwrap();
        assert!(!login_result);

        // Verify no user is logged in
        let current_user = auth_service.get_current_user(&db).unwrap();
        assert!(current_user.is_none());
    }

    #[test]
    fn test_get_current_user_when_logged_out() {
        let db = create_test_db("test_get_current_user_when_logged_out");
        let auth_service = AuthenticationService::new();

        // Test when no user is logged in
        let current_user = auth_service.get_current_user(&db).unwrap();
        assert!(current_user.is_none());
    }

    #[test]
    fn test_get_current_user_when_logged_in() {
        let db = create_test_db("test_get_current_user_when_logged_in");
        let mut auth_service = AuthenticationService::new();

        // Create and login user
        auth_service
            .sign_up(&db, "testuser", "password123", UserRole::Staff)
            .unwrap();
        auth_service.log_in(&db, "testuser", "password123").unwrap();

        // Test current user retrieval
        let current_user = auth_service.get_current_user(&db).unwrap().unwrap();
        assert_eq!(current_user.username, "testuser");
        assert_eq!(current_user.role, UserRole::Staff);
    }

    #[test]
    fn test_log_out_when_logged_in() {
        let db = create_test_db("test_log_out_when_logged_in");
        let mut auth_service = AuthenticationService::new();

        // Create and login user
        auth_service
            .sign_up(&db, "testuser", "password123", UserRole::Customer)
            .unwrap();
        auth_service.log_in(&db, "testuser", "password123").unwrap();

        // Verify user is logged in
        let current_user_before = auth_service.get_current_user(&db).unwrap();
        assert!(current_user_before.is_some());

        // Test logout
        auth_service.log_out();

        // Verify user is logged out
        let current_user_after = auth_service.get_current_user(&db).unwrap();
        assert!(current_user_after.is_none());
    }

    #[test]
    fn test_log_out_when_already_logged_out() {
        let db = create_test_db("test_log_out_when_already_logged_out");
        let mut auth_service = AuthenticationService::new();

        // Test logout when no user is logged in (should not panic)
        auth_service.log_out();

        // Verify still no user logged in
        let current_user = auth_service.get_current_user(&db).unwrap();
        assert!(current_user.is_none());
    }
}
