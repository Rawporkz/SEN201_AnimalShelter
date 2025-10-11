//
// authentication_service/test.rs
//
// This file contains unit tests for the authentication service module.
//

#[cfg(test)]
mod authentication_service_tests {
    use super::super::{
        types::{LoginResult, UserRole},
        AuthenticationService,
    };
    use std::fs;
    use std::path::PathBuf;

    /// Helper function to create a test authentication service with proper test artifacts directory
    ///
    /// # Arguments
    /// * `test_name` - Name of the test for unique database path
    ///
    /// # Returns
    /// * `AuthenticationService` - A new authentication service instance with test database
    fn create_test_auth_service(test_name: &str) -> AuthenticationService {
        // Create test artifacts directory structure
        let mut db_path = PathBuf::from("test_artifacts/authentication_service");
        db_path.push(test_name);

        // Ensure directory exists
        fs::create_dir_all(&db_path).expect("Failed to create test artifacts directory");

        // Add database filename
        db_path.push("test_auth.db");

        // Remove existing database if it exists
        let _ = fs::remove_file(&db_path);

        AuthenticationService::new(db_path).expect("Failed to create test auth service")
    }

    #[test]
    fn test_sign_up() {
        let mut auth_service = create_test_auth_service("test_sign_up_valid_user");

        // Test successful sign up
        let result = auth_service.sign_up("testuser", "password123", UserRole::Customer);
        assert!(result.is_ok());

        // Verify user can now log in (implicit verification that user was created)
        let login_result = auth_service.log_in("testuser", "password123");
        assert!(login_result.is_ok());
        assert_eq!(login_result.unwrap(), LoginResult::Success);
    }

    #[test]
    fn test_sign_up_duplicate_user() {
        let mut auth_service = create_test_auth_service("test_sign_up_duplicate_user");

        // First sign up should succeed
        let result1 = auth_service.sign_up("testuser", "password123", UserRole::Customer);
        assert!(result1.is_ok());

        // Duplicate sign up should fail
        let result2 = auth_service.sign_up("testuser", "password456", UserRole::Staff);
        assert!(result2.is_err());
    }

    #[test]
    fn test_sign_up_invalid_input() {
        let mut auth_service = create_test_auth_service("test_sign_up_invalid_input");

        // Empty username should fail
        let result1 = auth_service.sign_up("", "password123", UserRole::Customer);
        assert!(result1.is_err());
        assert!(result1
            .unwrap_err()
            .to_string()
            .contains("Username cannot be empty"));

        // Whitespace-only username should fail
        let result2 = auth_service.sign_up("   ", "password123", UserRole::Customer);
        assert!(result2.is_err());
        assert!(result2
            .unwrap_err()
            .to_string()
            .contains("Username cannot be empty"));

        // Short password should fail
        let result3 = auth_service.sign_up("testuser", "123", UserRole::Customer);
        assert!(result3.is_err());
        assert!(result3
            .unwrap_err()
            .to_string()
            .contains("Password must be at least 6 characters"));
    }

    #[test]
    fn test_log_in_valid_password() {
        let mut auth_service = create_test_auth_service("test_log_in_valid_credentials");

        // Create a user first
        auth_service
            .sign_up("testuser", "password123", UserRole::Customer)
            .unwrap();

        // Test successful login
        let login_result = auth_service.log_in("testuser", "password123").unwrap();
        assert_eq!(login_result, LoginResult::Success);

        // Verify user is now logged in
        let current_user = auth_service.get_current_user().unwrap().unwrap();
        assert_eq!(current_user.username, "testuser");
        assert_eq!(current_user.role, UserRole::Customer);
    }

    #[test]
    fn test_log_in_invalid_password() {
        let mut auth_service = create_test_auth_service("test_log_in_invalid_password");

        // Create a user first (this will automatically log them in)
        auth_service
            .sign_up("testuser", "password123", UserRole::Customer)
            .unwrap();

        // Log out the user first to test fresh login attempt
        auth_service.log_out();

        // Test login with wrong password
        let login_result = auth_service.log_in("testuser", "wrongpassword").unwrap();
        assert_eq!(login_result, LoginResult::InvalidPassword);

        // Verify no user is logged in
        let current_user = auth_service.get_current_user().unwrap();
        assert!(current_user.is_none());
    }

    #[test]
    fn test_log_in_nonexistent_user() {
        let mut auth_service = create_test_auth_service("test_log_in_nonexistent_user");

        // Test login with non-existent username
        let login_result = auth_service.log_in("nonexistent", "password123").unwrap();
        assert_eq!(login_result, LoginResult::UserNotFound);

        // Verify no user is logged in
        let current_user = auth_service.get_current_user().unwrap();
        assert!(current_user.is_none());
    }

    #[test]
    fn test_get_current_user_when_logged_out() {
        let auth_service = create_test_auth_service("test_get_current_user_when_logged_out");

        // Test when no user is logged in
        let current_user = auth_service.get_current_user().unwrap();
        assert!(current_user.is_none());
    }

    #[test]
    fn test_get_current_user_when_logged_in() {
        let mut auth_service = create_test_auth_service("test_get_current_user_when_logged_in");

        // Create and login user
        auth_service
            .sign_up("testuser", "password123", UserRole::Staff)
            .unwrap();
        let login_result = auth_service.log_in("testuser", "password123").unwrap();
        assert_eq!(login_result, LoginResult::Success);

        // Test current user retrieval
        let current_user = auth_service.get_current_user().unwrap().unwrap();
        assert_eq!(current_user.username, "testuser");
        assert_eq!(current_user.role, UserRole::Staff);
    }

    #[test]
    fn test_log_out_when_logged_in() {
        let mut auth_service = create_test_auth_service("test_log_out_when_logged_in");

        // Create and login user
        auth_service
            .sign_up("testuser", "password123", UserRole::Customer)
            .unwrap();
        let login_result = auth_service.log_in("testuser", "password123").unwrap();
        assert_eq!(login_result, LoginResult::Success);

        // Verify user is logged in
        let current_user_before = auth_service.get_current_user().unwrap();
        assert!(current_user_before.is_some());

        // Test logout
        auth_service.log_out();

        // Verify user is logged out
        let current_user_after = auth_service.get_current_user().unwrap();
        assert!(current_user_after.is_none());
    }

    #[test]
    fn test_log_out_when_already_logged_out() {
        let mut auth_service = create_test_auth_service("test_log_out_when_already_logged_out");

        // Test logout when no user is logged in (should not panic)
        auth_service.log_out();

        // Verify still no user logged in
        let current_user = auth_service.get_current_user().unwrap();
        assert!(current_user.is_none());
    }
}
