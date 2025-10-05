//
// authentication_service/mod.rs
//
// This module provides authentication-related functionality including
// user registration, login/logout, and session management.
// Passwords are securely hashed using bcrypt.
//

mod test;

use crate::database_service::{types::UserAuthentication, types::UserRole, DatabaseService};
use anyhow::{bail, Context, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

/// Service for handling authentication operations in the animal shelter application
pub struct AuthenticationService {
    /// Current logged-in username, None if no user is logged in
    current_user: Option<String>,
}

/// Represents the current user's information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CurrentUser {
    /// Username of the current user
    pub username: String,
    /// Role of the current user
    pub role: UserRole,
}

impl AuthenticationService {
    /// Creates a new AuthenticationService instance
    ///
    /// # Returns
    /// * `AuthenticationService` - New authentication service instance
    pub fn new() -> Self {
        log::debug!("Authentication service initialized");
        AuthenticationService { current_user: None }
    }

    /// Registers a new user with the given credentials and logs them in
    ///
    /// # Arguments
    /// * `database_service` - Reference to the database service
    /// * `username` - Username for the new account
    /// * `password` - Plain text password (will be hashed securely)
    /// * `role` - Role to assign to the new user
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn sign_up(
        &mut self,
        database_service: &DatabaseService,
        username: &str,
        password: &str,
        role: UserRole,
    ) -> Result<()> {
        // Validate input parameters
        if username.trim().is_empty() {
            bail!("Username cannot be empty");
        }
        if password.len() < 6 {
            bail!("Password must be at least 6 characters long");
        }

        // Hash the password securely
        let password_hash = hash(password, DEFAULT_COST).context("Failed to hash password")?;

        // Create user authentication record
        let user_auth = UserAuthentication {
            username: username.to_string(),
            password_hash,
            role,
        };

        // Insert user into database
        database_service
            .insert_user(&user_auth)
            .context("Failed to create user account")?;

        // Automatically log in the user after successful registration
        self.current_user = Some(username.to_string());

        log::info!(
            "User account created and logged in successfully for username: {}",
            username
        );
        Ok(())
    }

    /// Attempts to log in a user with the given credentials
    ///
    /// # Arguments
    /// * `database_service` - Reference to the database service
    /// * `username` - Username to log in
    /// * `password` - Plain text password to verify
    ///
    /// # Returns
    /// * `Result<bool>` - True if login successful, false if credentials invalid
    pub fn log_in(
        &mut self,
        database_service: &DatabaseService,
        username: &str,
        password: &str,
    ) -> Result<bool> {
        // Retrieve password hash from database
        let stored_hash = match database_service.get_password_hash(username)? {
            Some(hash) => hash,
            None => {
                log::warn!("Login attempt for non-existent username: {}", username);
                return Ok(false);
            }
        };

        // Verify password against stored hash
        let password_valid = verify(password, &stored_hash).context("Failed to verify password")?;

        if password_valid {
            // Set current user on successful login
            self.current_user = Some(username.to_string());
            log::info!("User logged in successfully: {}", username);
            Ok(true)
        } else {
            log::warn!("Invalid password for username: {}", username);
            Ok(false)
        }
    }

    /// Retrieves information about the current logged-in user
    ///
    /// # Arguments
    /// * `database_service` - Reference to the database service
    ///
    /// # Returns
    /// * `Result<Option<CurrentUser>>` - Current user info if logged in, None otherwise
    pub fn get_current_user(
        &self,
        database_service: &DatabaseService,
    ) -> Result<Option<CurrentUser>> {
        match &self.current_user {
            Some(username) => {
                // Get user role from database
                let role = database_service
                    .get_user_role(username)?
                    .context("Current user not found in database")?;

                log::debug!("Retrieved current user info for: {}", username);
                Ok(Some(CurrentUser {
                    username: username.clone(),
                    role,
                }))
            }
            None => {
                log::debug!("No user currently logged in");
                Ok(None)
            }
        }
    }

    /// Logs out the current user
    ///
    /// # Returns
    /// * `()` - Always succeeds
    pub fn log_out(&mut self) {
        match &self.current_user {
            Some(username) => {
                log::info!("User logged out: {}", username);
                self.current_user = None;
            }
            None => {
                log::warn!("No user was logged in to log out");
            }
        }
    }
}
