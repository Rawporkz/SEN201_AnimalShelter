//
// database_service/types.rs
//
// This module contains all database-related type definitions including structs and enums
// for animals, adoption requests, and their associated data types.
//

use rusqlite::{types::FromSql, ToSql};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Status of an animal in the shelter system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum AnimalStatus {
    /// Animal is available for adoption
    Available,
    /// Animal has been requested for adoption but not yet approved
    Requested,
    /// Animal has been successfully adopted
    Adopted,
    /// Animal has passed away
    PassedAway,
}

/// Implement ToSql and FromSql for AnimalStatus to store it as a string in the database
impl ToSql for AnimalStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.to_string()))
    }
}
impl FromSql for AnimalStatus {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        String::column_result(value)?.parse().map_err(|e| {
            rusqlite::types::FromSqlError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e,
            )))
        })
    }
}

/// Status of an adoption request in the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RequestStatus {
    /// Request is pending review
    Pending,
    /// Request has been rejected
    Rejected,
    /// Request has been approved
    Approved,
}

/// Implement ToSql and FromSql for RequestStatus to store it as a string in the database
impl ToSql for RequestStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.to_string()))
    }
}
impl FromSql for RequestStatus {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        String::column_result(value)?.parse().map_err(|e| {
            rusqlite::types::FromSqlError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e,
            )))
        })
    }
}

/// Represents an animal in the shelter system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animal {
    /// Unique identifier for the animal
    pub id: String,
    /// Name of the animal
    pub name: String,
    /// Species of the animal (e.g., "Dog", "Cat")
    pub specie: String,
    /// Breed of the animal
    pub breed: String,
    /// Sex of the animal (e.g., "Male", "Female")
    pub sex: String,
    /// Birth month of the animal (1-12)
    pub birth_month: i32,
    /// Birth year of the animal
    pub birth_year: i32,
    /// Whether the animal has been neutered
    pub neutered: bool,
    /// Timestamp when the animal was admitted to the shelter
    pub admission_timestamp: i64,
    /// Current status of the animal
    pub status: AnimalStatus,
    /// Path to the animal's image file
    pub image_path: Option<String>,
}

/// Simplified animal information for listing views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimalSummary {
    /// Unique identifier for the animal
    pub id: String,
    /// Name of the animal
    pub name: String,
    /// Species of the animal
    pub specie: String,
    /// Breed of the animal
    pub breed: String,
    /// Sex of the animal
    pub sex: String,
    /// Timestamp when the animal was admitted to the shelter
    pub admission_timestamp: i64,
    /// Path to the animal's image file
    pub image_path: Option<String>,
}

/// Represents an adoption request in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionRequest {
    /// Unique identifier for the adoption request
    pub id: String,
    /// ID of the animal being requested for adoption
    pub animal_id: String,
    /// Full name of the person requesting adoption
    pub name: String,
    /// Email address of the requester
    pub email: String,
    /// Telephone number of the requester
    pub tel_number: String,
    /// Address of the requester
    pub address: String,
    /// Occupation of the requester
    pub occupation: String,
    /// Annual income of the requester
    pub annual_income: String,
    /// Number of people in the household
    pub num_people: i32,
    /// Number of children in the household
    pub num_children: i32,
    /// Timestamp when the request was submitted
    pub request_timestamp: i64,
    /// Timestamp when the adoption was completed (0 if not completed)
    pub adoption_timestamp: i64,
    /// Current status of the request
    pub status: RequestStatus,
}

/// Simplified adoption request information for listing views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionRequestSummary {
    /// Unique identifier for the adoption request
    pub id: String,
    /// ID of the animal being requested for adoption
    pub animal_id: String,
    /// Full name of the person requesting adoption
    pub name: String,
    /// Email address of the requester
    pub email: String,
    /// Timestamp when the request was submitted
    pub request_timestamp: i64,
}
