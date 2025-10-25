//
// database_service/mod.rs
//
// This module provides database-related functionality to other components,
// including operations for managing animals and adoption requests.
// The database is powered by SQLite.
//

mod test;
pub mod types;

use anyhow::{bail, Context, Result};
use chrono::{Datelike, Duration, Utc};
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::path::Path;
use types::{AdoptionRequest, Animal, AnimalSummary, FilterCriteria, FilterValue};

/// Service for handling database operations in the animal shelter application
pub struct DatabaseService {
    /// SQLite database connection
    connection: Connection,
}

impl DatabaseService {
    /// Creates a new DatabaseService instance and initializes the database
    ///
    /// # Arguments
    /// * `db_path` - Path where the SQLite database file should be created/opened
    ///
    /// # Returns
    /// * `Result<DatabaseService>` - New DatabaseService instance or error
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        // Create database connection
        let connection = Connection::open(db_path.as_ref()).context(format!(
            "Failed to open database at path: {:?}",
            db_path.as_ref()
        ))?;

        // Create service instance
        let service = DatabaseService { connection };

        // Initialize database tables
        service
            .initialize_tables()
            .context("Failed to initialize database tables")?;

        log::info!(
            "Database service initialized successfully at path: {:?}",
            db_path.as_ref()
        );

        Ok(service)
    }

    /// Initializes the database tables if they don't exist
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    fn initialize_tables(&self) -> Result<()> {
        // Create animals table
        self.connection
            .execute(
                "
            CREATE TABLE IF NOT EXISTS animals (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                specie TEXT NOT NULL,
                breed TEXT NOT NULL,
                sex TEXT NOT NULL,
                birth_month INTEGER,
                birth_year INTEGER,
                neutered BOOLEAN NOT NULL,
                admission_timestamp INTEGER NOT NULL,
                status TEXT NOT NULL,
                image_path TEXT,
                appearance TEXT NOT NULL,
                bio TEXT NOT NULL
            )
            ",
                [],
            )
            .context("Failed to create animals table")?;

        // Create adoption_requests table
        self.connection
            .execute(
                "
            CREATE TABLE IF NOT EXISTS adoption_requests (
                id TEXT PRIMARY KEY,
                animal_id TEXT NOT NULL,
                username TEXT NOT NULL,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                tel_number TEXT NOT NULL,
                address TEXT NOT NULL,
                occupation TEXT NOT NULL,
                annual_income TEXT NOT NULL,
                num_people INTEGER NOT NULL,
                num_children INTEGER NOT NULL,
                request_timestamp INTEGER NOT NULL,
                adoption_timestamp INTEGER NOT NULL,
                status TEXT NOT NULL,
                country TEXT NOT NULL,
                FOREIGN KEY (animal_id) REFERENCES animals (id)
            )
            ",
                [],
            )
            .context("Failed to create adoption_requests table")?;

        log::debug!("Database tables initialized successfully");
        Ok(())
    }

    // ==================== ANIMALS TABLE OPERATIONS ====================

    /// Retrieves summary information for all animals in the database, with optional filtering
    ///
    /// # Arguments
    /// * `filters` - Optional map of filter criteria and values
    ///
    /// # Returns
    /// * `Result<Vec<AnimalSummary>>` - List of animal summaries or error
    pub fn query_animals(
        &self,
        filters: Option<HashMap<FilterCriteria, Option<FilterValue>>>,
    ) -> Result<Vec<AnimalSummary>> {
        let mut query = "SELECT id, name, specie, breed, sex, admission_timestamp, status, image_path FROM animals".to_string();
        let mut where_clauses: Vec<String> = Vec::new();
        let mut params: Vec<rusqlite::types::Value> = Vec::new();

        if let Some(filters_map) = filters {
            if !filters_map.is_empty() {
                for (criteria, value_option) in filters_map {
                    // Renamed value to value_option
                    if let Some(value) = value_option {
                        // Added unwrap for Option<FilterValue>
                        match criteria {
                            FilterCriteria::Status => {
                                if let FilterValue::ChooseMany(stati) = value {
                                    if stati.is_empty() {
                                        where_clauses.push("1=0".to_string()); // No matches if empty list
                                    } else {
                                        let placeholders: Vec<_> =
                                            stati.iter().map(|_| "?").collect();
                                        where_clauses.push(format!(
                                            "status IN ({})",
                                            placeholders.join(",")
                                        ));
                                        for s in stati {
                                            params.push(rusqlite::types::Value::from(s));
                                        }
                                    }
                                }
                            }
                            FilterCriteria::Sex => {
                                if let FilterValue::ChooseMany(sexes) = value {
                                    if sexes.is_empty() {
                                        where_clauses.push("1=0".to_string()); // No matches if empty list
                                    } else {
                                        let placeholders: Vec<_> =
                                            sexes.iter().map(|_| "?").collect();
                                        where_clauses
                                            .push(format!("sex IN ({})", placeholders.join(",")));
                                        for s in sexes {
                                            params.push(rusqlite::types::Value::from(s));
                                        }
                                    }
                                }
                            }
                            FilterCriteria::SpeciesAndBreeds => {
                                if let FilterValue::NestedChooseMany(species_map) = value {
                                    if species_map.is_empty() {
                                        where_clauses.push("1=0".to_string()); // No matches if empty map
                                    } else {
                                        let mut species_clauses = Vec::new();
                                        for (specie, breeds) in species_map {
                                            if !breeds.is_empty() {
                                                let breed_placeholders: Vec<_> =
                                                    breeds.iter().map(|_| "?").collect();
                                                species_clauses.push(format!(
                                                    "(specie = ? AND breed IN ({}))",
                                                    breed_placeholders.join(",")
                                                ));
                                                params.push(rusqlite::types::Value::from(specie));
                                                for b in breeds {
                                                    params.push(rusqlite::types::Value::from(b));
                                                }
                                            }
                                        }
                                        if species_clauses.is_empty() {
                                            where_clauses.push("1=0".to_string());
                                        // No matches if all nested breed lists are empty
                                        } else {
                                            where_clauses.push(format!(
                                                "({})",
                                                species_clauses.join(" OR ")
                                            ));
                                        }
                                    }
                                }
                            }
                            FilterCriteria::AdmissionDate => {
                                if let FilterValue::ChooseOne(date_option) = value {
                                    if date_option == "all_time" {
                                        continue;
                                    }
                                    let now = Utc::now();
                                    let start_of_period = match date_option.as_str() {
                                        "today" => now.date_naive().and_hms_opt(0, 0, 0).unwrap(),
                                        "this_week" => {
                                            let weekday = now.weekday();
                                            let days_to_subtract = weekday.num_days_from_monday();
                                            now.date_naive()
                                                .checked_sub_signed(Duration::days(
                                                    days_to_subtract as i64,
                                                ))
                                                .unwrap()
                                                .and_hms_opt(0, 0, 0)
                                                .unwrap()
                                        }
                                        "this_month" => now
                                            .date_naive()
                                            .with_day(1)
                                            .unwrap()
                                            .and_hms_opt(0, 0, 0)
                                            .unwrap(),
                                        "this_year" => now
                                            .date_naive()
                                            .with_ordinal(1)
                                            .unwrap()
                                            .and_hms_opt(0, 0, 0)
                                            .unwrap(),
                                        _ => continue,
                                    };
                                    let start_timestamp = start_of_period.and_utc().timestamp();
                                    where_clauses.push("admission_timestamp >= ?".to_string());
                                    params.push(rusqlite::types::Value::Integer(start_timestamp));
                                }
                            }
                            FilterCriteria::AdoptionDate => {
                                if let FilterValue::ChooseOne(date_option) = value {
                                    if date_option == "all_time" {
                                        continue;
                                    }
                                    let now = Utc::now();
                                    let start_of_period = match date_option.as_str() {
                                        "today" => now.date_naive().and_hms_opt(0, 0, 0).unwrap(),
                                        "this_week" => {
                                            let weekday = now.weekday();
                                            let days_to_subtract = weekday.num_days_from_monday();
                                            now.date_naive()
                                                .checked_sub_signed(Duration::days(
                                                    days_to_subtract as i64,
                                                ))
                                                .unwrap()
                                                .and_hms_opt(0, 0, 0)
                                                .unwrap()
                                        }
                                        "this_month" => now
                                            .date_naive()
                                            .with_day(1)
                                            .unwrap()
                                            .and_hms_opt(0, 0, 0)
                                            .unwrap(),
                                        "this_year" => now
                                            .date_naive()
                                            .with_ordinal(1)
                                            .unwrap()
                                            .and_hms_opt(0, 0, 0)
                                            .unwrap(),
                                        _ => continue,
                                    };
                                    let start_timestamp = start_of_period.and_utc().timestamp();
                                    where_clauses.push(
                                        "EXISTS (SELECT 1 FROM adoption_requests ar WHERE ar.animal_id = animals.id AND ar.status = 'approved' AND ar.adoption_timestamp >= ?)".to_string()
                                    );
                                    params.push(rusqlite::types::Value::Integer(start_timestamp));
                                }
                            }
                        }
                    }
                }
            }
        }

        if !where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&where_clauses.join(" AND "));
        }

        let mut statement = self
            .connection
            .prepare(&query)
            .context(format!("Failed to prepare query for animals: {}", query))?;

        let animal_iter = statement
            .query_map(rusqlite::params_from_iter(params.iter()), |row| {
                Ok(AnimalSummary {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    specie: row.get(2)?,
                    breed: row.get(3)?,
                    sex: row.get(4)?,
                    admission_timestamp: row.get(5)?,
                    status: row.get(6)?,
                    image_path: row.get(7)?,
                })
            })
            .context("Failed to execute query for animals")?;

        let mut animals = Vec::new();
        for animal in animal_iter {
            animals.push(animal.context("Failed to parse animal row")?);
        }

        log::debug!("Retrieved {} animals from database", animals.len());
        Ok(animals)
    }

    /// Retrieves complete information for a specific animal by ID
    ///
    /// # Arguments
    /// * `animal_id` - The ID of the animal to retrieve
    ///
    /// # Returns
    /// * `Result<Option<Animal>>` - Complete animal information or None if not found
    pub fn query_animal_by_id(&self, animal_id: &str) -> Result<Option<Animal>> {
        let mut statement = self.connection.prepare(
            "SELECT id, name, specie, breed, sex, birth_month, birth_year, neutered, admission_timestamp, status, image_path, appearance, bio FROM animals WHERE id = ?1"
        ).context("Failed to prepare query for animal by ID")?;

        let mut rows = statement
            .query_map(params![animal_id], |row| {
                Ok(Animal {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    specie: row.get(2)?,
                    breed: row.get(3)?,
                    sex: row.get(4)?,
                    birth_month: row.get(5)?,
                    birth_year: row.get(6)?,
                    neutered: row.get(7)?,
                    admission_timestamp: row.get(8)?,
                    status: row.get(9)?,
                    image_path: row.get(10)?,
                    appearance: row.get(11)?,
                    bio: row.get(12)?,
                })
            })
            .context("Failed to execute query for animal by ID")?;

        match rows.next() {
            Some(row) => {
                let animal = row.context("Failed to parse animal row")?;
                log::debug!("Retrieved animal with ID: {}", animal_id);
                Ok(Some(animal))
            }
            None => {
                log::debug!("No animal found with ID: {}", animal_id);
                Ok(None)
            }
        }
    }

    /// Inserts a new animal into the database
    ///
    /// # Arguments
    /// * `animal` - The animal information to insert
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn insert_animal(&self, animal: &Animal) -> Result<()> {
        // Auto-generate ID if not provided (or empty)
        let id = if animal.id.trim().is_empty() {
            let max_id: i64 = self
                .connection
                .query_row(
                    "SELECT COALESCE(MAX(CAST(id AS INTEGER)), 0) FROM animals",
                    [],
                    |row| row.get(0),
                )
                .context("Failed to query max animal ID")?;
            (max_id + 1).to_string()
        } else {
            animal.id.clone()
        };
        let rows_affected = self.connection.execute(
            "INSERT INTO animals (id, name, specie, breed, sex, birth_month, birth_year, neutered, admission_timestamp, status, image_path, appearance, bio) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                id,
                animal.name,
                animal.specie,
                animal.breed,
                animal.sex,
                animal.birth_month,
                animal.birth_year,
                animal.neutered,
                animal.admission_timestamp,
                animal.status,
                animal.image_path,
                animal.appearance,
                animal.bio
            ]
        ).context("Failed to insert animal into database")?;

        if rows_affected == 1 {
            log::info!("Successfully inserted animal with ID: {}", id);
            Ok(())
        } else {
            bail!(
                "Unexpected number of rows affected when inserting animal: {}",
                rows_affected
            );
        }
    }

    /// Updates an existing animal in the database
    ///
    /// # Arguments
    /// * `animal` - The updated animal information
    ///
    /// # Returns
    /// * `Result<bool>` - True if animal was found and updated, false if not found
    pub fn update_animal(&self, animal: &Animal) -> Result<bool> {
        let rows_affected = self.connection.execute(
            "UPDATE animals SET name = ?2, specie = ?3, breed = ?4, sex = ?5, birth_month = ?6, birth_year = ?7, neutered = ?8, admission_timestamp = ?9, status = ?10, image_path = ?11, appearance = ?12, bio = ?13 WHERE id = ?1",
            params![
                animal.id,
                animal.name,
                animal.specie,
                animal.breed,
                animal.sex,
                animal.birth_month,
                animal.birth_year,
                animal.neutered,
                animal.admission_timestamp,
                animal.status,
                animal.image_path,
                animal.appearance,
                animal.bio
            ]
        ).context("Failed to update animal in database")?;

        match rows_affected {
            1 => {
                log::info!("Successfully updated animal with ID: {}", animal.id);
                Ok(true)
            }
            0 => {
                log::warn!("No animal found with ID: {} for update", animal.id);
                Ok(false)
            }
            _ => {
                bail!(
                    "Unexpected number of rows affected when updating animal: {}",
                    rows_affected
                );
            }
        }
    }

    /// Deletes an animal from the database by ID
    ///
    /// # Arguments
    /// * `animal_id` - The ID of the animal to delete
    ///
    /// # Returns
    /// * `Result<bool>` - True if animal was found and deleted, false if not found
    pub fn delete_animal(&self, animal_id: &str) -> Result<bool> {
        let rows_affected = self
            .connection
            .execute("DELETE FROM animals WHERE id = ?1", params![animal_id])
            .context("Failed to delete animal from database")?;

        match rows_affected {
            1 => {
                log::info!("Successfully deleted animal with ID: {}", animal_id);
                Ok(true)
            }
            0 => {
                log::warn!("No animal found with ID: {} for deletion", animal_id);
                Ok(false)
            }
            _ => {
                bail!(
                    "Unexpected number of rows affected when deleting animal: {}",
                    rows_affected
                );
            }
        }
    }

    // ==================== ADOPTION_REQUESTS TABLE OPERATIONS ====================

    /// Retrieves complete information for all adoption requests associated with a specific animal ID
    ///
    /// # Arguments
    /// * `animal_id` - The ID of the animal to retrieve requests for
    ///
    /// # Returns
    /// * `Result<Vec<AdoptionRequest>>` - List of adoption requests or error
    pub fn query_adoption_requests_by_animal_id(
        &self,
        animal_id: &str,
    ) -> Result<Vec<AdoptionRequest>> {
        // SQL query to select adoption requests by animal ID
        let query =
                "SELECT id, animal_id, username, name, email, tel_number, address, occupation, annual_income, num_people, num_children, request_timestamp, adoption_timestamp, status, country FROM adoption_requests WHERE animal_id = ?1"
                    .to_string();

        let mut statement = self.connection.prepare(&query).context(format!(
            "Failed to prepare query for adoption requests by animal ID: {}",
            query
        ))?;

        let request_iter = statement
            .query_map(rusqlite::params![animal_id], |row| {
                Ok(AdoptionRequest {
                    id: row.get(0)?,
                    animal_id: row.get(1)?,
                    username: row.get(2)?,
                    name: row.get(3)?,
                    email: row.get(4)?,
                    tel_number: row.get(5)?,
                    address: row.get(6)?,
                    occupation: row.get(7)?,
                    annual_income: row.get(8)?,
                    num_people: row.get(9)?,
                    num_children: row.get(10)?,
                    request_timestamp: row.get(11)?,
                    adoption_timestamp: row.get(12)?,
                    status: row.get(13)?,
                    country: row.get(14)?,
                })
            })
            .context("Failed to execute query for adoption requests by animal ID")?;

        let mut requests = Vec::new();
        for request in request_iter {
            requests.push(request.context("Failed to parse adoption request row")?);
        }

        log::debug!(
            "Retrieved {} adoption requests for animal ID: {}",
            requests.len(),
            animal_id
        );
        Ok(requests)
    }

    /// Retrieves complete information for all adoption requests associated with a specific user name
    ///
    /// # Arguments
    /// * `username` - The user name to retrieve requests for
    ///
    /// # Returns
    /// * `Result<Vec<AdoptionRequest>>` - List of adoption requests or error
    pub fn query_adoption_requests_by_username(
        &self,
        username: &str,
    ) -> Result<Vec<AdoptionRequest>> {
        // SQL query to select adoption requests by username
        let query =
                "SELECT id, animal_id, username, name, email, tel_number, address, occupation, annual_income, num_people, num_children, request_timestamp, adoption_timestamp, status, country FROM adoption_requests WHERE username = ?1"
                    .to_string();

        let mut statement = self.connection.prepare(&query).context(format!(
            "Failed to prepare query for adoption requests by user name: {}",
            query
        ))?;

        let request_iter = statement
            .query_map(rusqlite::params![username], |row| {
                Ok(AdoptionRequest {
                    id: row.get(0)?,
                    animal_id: row.get(1)?,
                    username: row.get(2)?,
                    name: row.get(3)?,
                    email: row.get(4)?,
                    tel_number: row.get(5)?,
                    address: row.get(6)?,
                    occupation: row.get(7)?,
                    annual_income: row.get(8)?,
                    num_people: row.get(9)?,
                    num_children: row.get(10)?,
                    request_timestamp: row.get(11)?,
                    adoption_timestamp: row.get(12)?,
                    status: row.get(13)?,
                    country: row.get(14)?,
                })
            })
            .context("Failed to execute query for adoption requests by user name")?;

        let mut requests = Vec::new();
        for request in request_iter {
            requests.push(request.context("Failed to parse adoption request row")?);
        }

        log::debug!(
            "Retrieved {} adoption requests for user name: {}",
            requests.len(),
            username
        );
        Ok(requests)
    }

    /// Retrieves complete information for a specific adoption request by ID
    ///
    /// # Arguments
    /// * `request_id` - The ID of the adoption request to retrieve
    ///
    /// # Returns
    /// * `Result<Option<AdoptionRequest>>` - Complete adoption request information or None if not found
    pub fn query_adoption_request_by_id(
        &self,
        request_id: &str,
    ) -> Result<Option<AdoptionRequest>> {
        // Prepare the SQL statement
        let mut statement = self.connection.prepare(
                "SELECT id, animal_id, username, name, email, tel_number, address, occupation, annual_income, num_people, num_children, request_timestamp, adoption_timestamp, status, country FROM adoption_requests WHERE id = ?1"
            ).context("Failed to prepare query for adoption request by ID")?;
        let mut rows = statement
            .query_map(params![request_id], |row| {
                Ok(AdoptionRequest {
                    id: row.get(0)?,
                    animal_id: row.get(1)?,
                    username: row.get(2)?,
                    name: row.get(3)?,
                    email: row.get(4)?,
                    tel_number: row.get(5)?,
                    address: row.get(6)?,
                    occupation: row.get(7)?,
                    annual_income: row.get(8)?,
                    num_people: row.get(9)?,
                    num_children: row.get(10)?,
                    request_timestamp: row.get(11)?,
                    adoption_timestamp: row.get(12)?,
                    status: row.get(13)?,
                    country: row.get(14)?,
                })
            })
            .context("Failed to execute query for adoption request by ID")?;

        match rows.next() {
            Some(row) => {
                let request = row.context("Failed to parse adoption request row")?;
                log::debug!("Retrieved adoption request with ID: {}", request_id);
                Ok(Some(request))
            }
            None => {
                log::debug!("No adoption request found with ID: {}", request_id);
                Ok(None)
            }
        }
    }

    /// Inserts a new adoption request into the database
    ///
    /// # Arguments
    /// * `request` - The adoption request information to insert
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn insert_adoption_request(&self, request: &AdoptionRequest) -> Result<()> {
        // Auto-generate ID if not provided (or empty)
        let id = if request.id.trim().is_empty() {
            let max_id: i64 = self
                .connection
                .query_row(
                    "SELECT COALESCE(MAX(CAST(id AS INTEGER)), 0) FROM adoption_requests",
                    [],
                    |row| row.get(0),
                )
                .context("Failed to query max adoption request ID")?;
            (max_id + 1).to_string()
        } else {
            request.id.clone()
        };

        // Number of rows affected by the insert operation
        let rows_affected = self.connection.execute(
            "INSERT INTO adoption_requests (id, animal_id, username, name, email, tel_number, address, occupation, annual_income, num_people, num_children, request_timestamp, adoption_timestamp, status, country) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                id,
                request.animal_id,
                request.username,
                request.name,
                request.email,
                request.tel_number,
                request.address,
                request.occupation,
                request.annual_income,
                request.num_people,
                request.num_children,
                request.request_timestamp,
                request.adoption_timestamp,
                request.status,
                request.country
            ]
        ).context("Failed to insert adoption request into database")?;

        if rows_affected == 1 {
            log::info!("Successfully inserted adoption request with ID: {}", id);
            Ok(())
        } else {
            bail!(
                "Unexpected number of rows affected when inserting adoption request: {}",
                rows_affected
            );
        }
    }

    /// Updates an existing adoption request in the database
    ///
    /// # Arguments
    /// * `request` - The updated adoption request information
    ///
    /// # Returns
    /// * `Result<bool>` - True if request was found and updated, false if not found
    pub fn update_adoption_request(&self, request: &AdoptionRequest) -> Result<bool> {
        // Number of rows affected by the update operation
        let rows_affected = self.connection.execute(
            "UPDATE adoption_requests SET animal_id = ?2, username = ?3, name = ?4, email = ?5, tel_number = ?6, address = ?7, occupation = ?8, annual_income = ?9, num_people = ?10, num_children = ?11, request_timestamp = ?12, adoption_timestamp = ?13, status = ?14, country = ?15 WHERE id = ?1",
            params![
                request.id,
                request.animal_id,
                request.username,
                request.name,
                request.email,
                request.tel_number,
                request.address,
                request.occupation,
                request.annual_income,
                request.num_people,
                request.num_children,
                request.request_timestamp,
                request.adoption_timestamp,
                request.status,
                request.country
            ]
        ).context("Failed to update adoption request in database")?;

        match rows_affected {
            1 => {
                log::info!(
                    "Successfully updated adoption request with ID: {}",
                    request.id
                );
                Ok(true)
            }
            0 => {
                log::warn!(
                    "No adoption request found with ID: {} for update",
                    request.id
                );
                Ok(false)
            }
            _ => {
                bail!(
                    "Unexpected number of rows affected when updating adoption request: {}",
                    rows_affected
                );
            }
        }
    }

    /// Deletes an adoption request from the database by ID
    ///
    /// # Arguments
    /// * `request_id` - The ID of the adoption request to delete
    ///
    /// # Returns
    /// * `Result<bool>` - True if request was found and deleted, false if not found
    pub fn delete_adoption_request(&self, request_id: &str) -> Result<bool> {
        let rows_affected = self
            .connection
            .execute(
                "DELETE FROM adoption_requests WHERE id = ?1",
                params![request_id],
            )
            .context("Failed to delete adoption request from database")?;

        match rows_affected {
            1 => {
                log::info!(
                    "Successfully deleted adoption request with ID: {}",
                    request_id
                );
                Ok(true)
            }
            0 => {
                log::warn!(
                    "No adoption request found with ID: {} for deletion",
                    request_id
                );
                Ok(false)
            }
            _ => {
                bail!(
                    "Unexpected number of rows affected when deleting adoption request: {}",
                    rows_affected
                );
            }
        }
    }
}
