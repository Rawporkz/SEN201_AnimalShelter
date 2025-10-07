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
use rusqlite::{params, Connection};
use std::path::Path;
use types::{AdoptionRequest, AdoptionRequestSummary, Animal, AnimalSummary};

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
                birth_month INTEGER NOT NULL,
                birth_year INTEGER NOT NULL,
                neutered BOOLEAN NOT NULL,
                admission_timestamp INTEGER NOT NULL,
                status TEXT NOT NULL
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

    /// Retrieves summary information for all animals in the database
    ///
    /// # Returns
    /// * `Result<Vec<AnimalSummary>>` - List of animal summaries or error
    pub fn query_all_animals(&self) -> Result<Vec<AnimalSummary>> {
        let mut statement = self
            .connection
            .prepare("SELECT id, name, specie, breed, sex, admission_timestamp FROM animals")
            .context("Failed to prepare query for all animals")?;

        let animal_iter = statement
            .query_map([], |row| {
                Ok(AnimalSummary {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    specie: row.get(2)?,
                    breed: row.get(3)?,
                    sex: row.get(4)?,
                    admission_timestamp: row.get(5)?,
                })
            })
            .context("Failed to execute query for all animals")?;

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
            "SELECT id, name, specie, breed, sex, birth_month, birth_year, neutered, admission_timestamp, status FROM animals WHERE id = ?1"
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
        let rows_affected = self.connection.execute(
            "INSERT INTO animals (id, name, specie, breed, sex, birth_month, birth_year, neutered, admission_timestamp, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
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
                animal.status
            ]
        ).context("Failed to insert animal into database")?;

        if rows_affected == 1 {
            log::info!("Successfully inserted animal with ID: {}", animal.id);
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
            "UPDATE animals SET name = ?2, specie = ?3, breed = ?4, sex = ?5, birth_month = ?6, birth_year = ?7, neutered = ?8, admission_timestamp = ?9, status = ?10 WHERE id = ?1",
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
                animal.status
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

    /// Retrieves summary information for all adoption requests in the database
    ///
    /// # Returns
    /// * `Result<Vec<AdoptionRequestSummary>>` - List of adoption request summaries or error
    pub fn query_all_adoption_requests(&self) -> Result<Vec<AdoptionRequestSummary>> {
        let mut statement = self
            .connection
            .prepare("SELECT id, animal_id, name, email, request_timestamp FROM adoption_requests")
            .context("Failed to prepare query for all adoption requests")?;

        let request_iter = statement
            .query_map([], |row| {
                Ok(AdoptionRequestSummary {
                    id: row.get(0)?,
                    animal_id: row.get(1)?,
                    name: row.get(2)?,
                    email: row.get(3)?,
                    request_timestamp: row.get(4)?,
                })
            })
            .context("Failed to execute query for all adoption requests")?;

        let mut requests = Vec::new();
        for request in request_iter {
            requests.push(request.context("Failed to parse adoption request row")?);
        }

        log::debug!(
            "Retrieved {} adoption requests from database",
            requests.len()
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
        let mut statement = self.connection.prepare(
            "SELECT id, animal_id, name, email, tel_number, address, occupation, annual_income, num_people, num_children, request_timestamp, adoption_timestamp, status FROM adoption_requests WHERE id = ?1"
        ).context("Failed to prepare query for adoption request by ID")?;

        let mut rows = statement
            .query_map(params![request_id], |row| {
                Ok(AdoptionRequest {
                    id: row.get(0)?,
                    animal_id: row.get(1)?,
                    name: row.get(2)?,
                    email: row.get(3)?,
                    tel_number: row.get(4)?,
                    address: row.get(5)?,
                    occupation: row.get(6)?,
                    annual_income: row.get(7)?,
                    num_people: row.get(8)?,
                    num_children: row.get(9)?,
                    request_timestamp: row.get(10)?,
                    adoption_timestamp: row.get(11)?,
                    status: row.get(12)?,
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
        let rows_affected = self.connection.execute(
            "INSERT INTO adoption_requests (id, animal_id, name, email, tel_number, address, occupation, annual_income, num_people, num_children, request_timestamp, adoption_timestamp, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                request.id,
                request.animal_id,
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
                request.status
            ]
        ).context("Failed to insert adoption request into database")?;

        if rows_affected == 1 {
            log::info!(
                "Successfully inserted adoption request with ID: {}",
                request.id
            );
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
        let rows_affected = self.connection.execute(
            "UPDATE adoption_requests SET animal_id = ?2, name = ?3, email = ?4, tel_number = ?5, address = ?6, occupation = ?7, annual_income = ?8, num_people = ?9, num_children = ?10, request_timestamp = ?11, adoption_timestamp = ?12, status = ?13 WHERE id = ?1",
            params![
                request.id,
                request.animal_id,
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
                request.status
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
