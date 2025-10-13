//
// authentication_service/mod.rs
//
// This module provides authentication-related functionality including
// user registration, login/logout, and session management.
// Passwords are securely hashed using bcrypt.
//

mod test;
pub mod types;

use anyhow::{bail, Context, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::{params, Connection};
use std::path::Path;
use types::{LoginResult, UserAuthentication, UserRole};

/// Service for handling authentication operations in the animal shelter application
pub struct AuthenticationService {
    /// Current logged-in username, None if no user is logged in
    current_user: Option<String>,
    /// SQLite database connection for authentication data
    connection: Connection,
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
    /// Creates a new AuthenticationService instance and initializes the authentication database
    ///
    /// # Arguments
    /// * `db_path` - Path where the authentication SQLite database file should be created/opened
    ///
    /// # Returns
    /// * `Result<AuthenticationService>` - New authentication service instance or error
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        // Create database connection
        let connection = Connection::open(db_path.as_ref()).context(format!(
            "Failed to open authentication database at path: {:?}",
            db_path.as_ref()
        ))?;

        // Create service instance
        let service = AuthenticationService {
            current_user: None,
            connection,
        };

        // Initialize database tables
        service
            .initialize_tables()
            .context("Failed to initialize authentication database tables")?;

        log::info!(
            "Authentication service initialized successfully at path: {:?}",
            db_path.as_ref()
        );

        Ok(service)
    }

    /// Initializes the authentication database tables if they don't exist
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    fn initialize_tables(&self) -> Result<()> {
        // Create user_authentication table
        self.connection
            .execute(
                "
            CREATE TABLE IF NOT EXISTS user_authentication (
                username TEXT PRIMARY KEY,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL
            )
            ",
                [],
            )
            .context("Failed to create user_authentication table")?;

        Ok(())
    }

    /// Registers a new user with the given credentials and logs them in
    ///
    /// # Arguments
    /// * `username` - Username for the new account
    /// * `password` - Plain text password (will be hashed securely)
    /// * `role` - Role to assign to the new user
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn sign_up(&mut self, username: &str, password: &str, role: UserRole) -> Result<()> {
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
        self.insert_user(&user_auth)
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
    /// * `username` - Username to log in
    /// * `password` - Plain text password to verify
    ///
    /// # Returns
    /// * `Result<LoginResult>` - Login result indicating success, invalid password, or user not found
    pub fn log_in(&mut self, username: &str, password: &str) -> Result<LoginResult> {
        // Retrieve password hash from database
        let stored_hash = match self.get_password_hash(username)? {
            Some(hash) => hash,
            None => {
                log::warn!("Login attempt for non-existent username: {}", username);
                return Ok(LoginResult::UserNotFound);
            }
        };

        // Verify password against stored hash
        let password_valid = verify(password, &stored_hash).context("Failed to verify password")?;

        if password_valid {
            // Set current user on successful login
            self.current_user = Some(username.to_string());
            log::info!("User logged in successfully: {}", username);
            Ok(LoginResult::Success)
        } else {
            log::warn!("Invalid password for username: {}", username);
            Ok(LoginResult::InvalidPassword)
        }
    }

    /// Retrieves information about the current logged-in user
    ///
    /// # Returns
    /// * `Result<Option<CurrentUser>>` - Current user info if logged in, None otherwise
    pub fn get_current_user(&self) -> Result<Option<CurrentUser>> {
        match &self.current_user {
            Some(username) => {
                // Get user role from database
                let role = self
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

    // ==================== PRIVATE DATABASE OPERATIONS ====================

    /// Retrieves the password hash for a specific username
    ///
    /// # Arguments
    /// * `username` - The username to look up
    ///
    /// # Returns
    /// * `Result<Option<String>>` - Password hash if user exists, None if not found
    fn get_password_hash(&self, username: &str) -> Result<Option<String>> {
        let mut statement = self
            .connection
            .prepare("SELECT password_hash FROM user_authentication WHERE username = ?1")
            .context("Failed to prepare query for password hash")?;

        let mut rows = statement
            .query_map(params![username], |row| {
                let password_hash: String = row.get(0)?;
                Ok(password_hash)
            })
            .context("Failed to execute query for password hash")?;

        match rows.next() {
            Some(row) => {
                let password_hash = row.context("Failed to parse password hash row")?;
                log::debug!("Retrieved password hash for username: {}", username);
                Ok(Some(password_hash))
            }
            None => {
                log::debug!("No user found with username: {}", username);
                Ok(None)
            }
        }
    }

    /// Inserts a new user authentication record into the database
    ///
    /// # Arguments
    /// * `user_auth` - The user authentication data to insert
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    fn insert_user(&self, user_auth: &UserAuthentication) -> Result<()> {
        let rows_affected = self
            .connection
            .execute(
                "INSERT INTO user_authentication (username, password_hash, role) VALUES (?1, ?2, ?3)",
                params![user_auth.username, user_auth.password_hash, user_auth.role],
            )
            .context("Failed to insert user into database")?;

        if rows_affected == 1 {
            log::info!(
                "Successfully inserted user with username: {}",
                user_auth.username
            );
            Ok(())
        } else {
            bail!(
                "Unexpected number of rows affected when inserting user: {}",
                rows_affected
            );
        }
    }

    /// Retrieves the role for a specific username
    ///
    /// # Arguments
    /// * `username` - The username to look up
    ///
    /// # Returns
    /// * `Result<Option<UserRole>>` - User role if user exists, None if not found
    fn get_user_role(&self, username: &str) -> Result<Option<UserRole>> {
        let mut statement = self
            .connection
            .prepare("SELECT role FROM user_authentication WHERE username = ?1")
            .context("Failed to prepare query for user role")?;

        let mut rows = statement
            .query_map(params![username], |row| {
                let role: UserRole = row.get(0)?;
                Ok(role)
            })
            .context("Failed to execute query for user role")?;

        match rows.next() {
            Some(row) => {
                let role = row.context("Failed to parse user role row")?;
                log::debug!("Retrieved role for username: {}", username);
                Ok(Some(role))
            }
            None => {
                log::debug!("No user found with username: {}", username);
                Ok(None)
            }
        }
    }
}
