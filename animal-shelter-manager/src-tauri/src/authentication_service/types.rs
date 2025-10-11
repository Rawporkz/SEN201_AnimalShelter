//
// authentication_service/types.rs
//
// This module contains authentication-related type definitions including structs and enums
// for user authentication and roles.
//

use rusqlite::{types::FromSql, ToSql};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// User role in the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum UserRole {
    /// Staff member with elevated privileges
    Staff,
    /// Customer using the system
    Customer,
}

/// Implement ToSql and FromSql for UserRole to store it as a string in the database
impl ToSql for UserRole {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.to_string()))
    }
}
impl FromSql for UserRole {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        String::column_result(value)?.parse().map_err(|e| {
            rusqlite::types::FromSqlError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e,
            )))
        })
    }
}

/// Result of a login attempt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoginResult {
    /// Login was successful
    Success,
    /// User exists but password is incorrect
    InvalidPassword,
    /// Username does not exist in the system
    UserNotFound,
}

/// Represents user authentication data in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthentication {
    /// Username for authentication
    pub username: String,
    /// Hashed password for security
    pub password_hash: String,
    /// User role in the system
    pub role: UserRole,
}
