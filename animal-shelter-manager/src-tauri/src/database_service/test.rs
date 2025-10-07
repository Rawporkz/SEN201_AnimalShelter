//
// database_service/test.rs
//
// This file contains unit tests for the database service module.
//

#[cfg(test)]
mod database_service_tests {
    use super::super::{
        types::{AdoptionRequest, Animal, AnimalStatus, RequestStatus},
        DatabaseService,
    };
    use chrono::Utc;
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
        let mut db_path = PathBuf::from("test_artifacts/database_service");
        db_path.push(test_name);

        // Ensure directory exists
        fs::create_dir_all(&db_path).expect("Failed to create test artifacts directory");

        // Add database filename
        db_path.push("test.db");

        // Remove existing database if it exists
        let _ = fs::remove_file(&db_path);

        DatabaseService::new(db_path).expect("Failed to create test db service")
    }

    /// Helper function to create a sample animal for testing
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the animal
    ///
    /// # Returns
    /// * `Animal` - Sample animal with test data
    fn sample_animal(id: &str) -> Animal {
        Animal {
            id: id.to_string(),
            name: "Buddy".to_string(),
            specie: "Dog".to_string(),
            breed: "Golden Retriever".to_string(),
            sex: "Male".to_string(),
            birth_month: 6,
            birth_year: 2020,
            neutered: true,
            admission_timestamp: Utc::now().timestamp(),
            status: AnimalStatus::Available,
        }
    }

    /// Helper function to create a sample adoption request for testing
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the adoption request
    /// * `animal_id` - ID of the animal being requested
    ///
    /// # Returns
    /// * `AdoptionRequest` - Sample adoption request with test data
    fn sample_request(id: &str, animal_id: &str) -> AdoptionRequest {
        AdoptionRequest {
            id: id.to_string(),
            animal_id: animal_id.to_string(),
            name: "Jira Pit".to_string(),
            email: "jira.pit@gmail.com".to_string(),
            tel_number: "0123456789".to_string(),
            address: "Bangkok, Thailand".to_string(),
            occupation: "Software Engineer".to_string(),
            annual_income: "50000".to_string(),
            num_people: 2,
            num_children: 0,
            request_timestamp: Utc::now().timestamp(),
            adoption_timestamp: 0,
            status: RequestStatus::Pending,
        }
    }

    // ==================== ANIMALS TESTS ====================

    #[test]
    fn test_animals() {
        let db = create_test_db("test_animals");
        let mut animal = sample_animal("a1");

        // Test empty query initially
        let animals = db.query_all_animals().unwrap();
        assert_eq!(animals.len(), 0);

        // Test insert
        db.insert_animal(&animal).unwrap();

        // Test query all after insert
        let animals = db.query_all_animals().unwrap();
        assert_eq!(animals.len(), 1);
        assert_eq!(animals[0].id, "a1");
        assert_eq!(animals[0].name, "Buddy");

        // Test query by ID
        let found = db.query_animal_by_id("a1").unwrap().unwrap();
        assert_eq!(found.id, "a1");
        assert_eq!(found.breed, "Golden Retriever");
        assert!(found.neutered);

        // Test query non-existent
        let not_found = db.query_animal_by_id("nonexistent").unwrap();
        assert!(not_found.is_none());

        // Test update
        animal.name = "Updated Buddy".to_string();
        animal.status = AnimalStatus::Adopted;
        let updated = db.update_animal(&animal).unwrap();
        assert!(updated);

        let found = db.query_animal_by_id("a1").unwrap().unwrap();
        assert_eq!(found.name, "Updated Buddy");
        assert_eq!(found.status, AnimalStatus::Adopted);

        // Test update non-existent
        let fake_animal = sample_animal("fake");
        let not_updated = db.update_animal(&fake_animal).unwrap();
        assert!(!not_updated);

        // Test delete
        let deleted = db.delete_animal("a1").unwrap();
        assert!(deleted);

        let not_found = db.query_animal_by_id("a1").unwrap();
        assert!(not_found.is_none());

        // Test delete non-existent
        let not_deleted = db.delete_animal("nonexistent").unwrap();
        assert!(!not_deleted);
    }

    #[test]
    fn test_animals_duplicate_insert() {
        let db = create_test_db("test_animals_duplicate_insert");
        let animal = sample_animal("a1");

        // First insert should succeed
        db.insert_animal(&animal).unwrap();

        // Duplicate insert should fail
        let duplicate_result = db.insert_animal(&animal);
        assert!(duplicate_result.is_err());
    }

    #[test]
    fn test_animals_multiple_records() {
        let db = create_test_db("test_animals_multiple_records");
        let animal1 = sample_animal("a1");
        let mut animal2 = sample_animal("a2");
        animal2.name = "Max".to_string();
        animal2.specie = "Cat".to_string();

        // Insert multiple animals
        db.insert_animal(&animal1).unwrap();
        db.insert_animal(&animal2).unwrap();

        // Verify all are returned
        let animals = db.query_all_animals().unwrap();
        assert_eq!(animals.len(), 2);

        let ids: Vec<&str> = animals.iter().map(|a| a.id.as_str()).collect();
        assert!(ids.contains(&"a1"));
        assert!(ids.contains(&"a2"));
    }

    // ==================== ADOPTION REQUESTS TESTS ====================

    #[test]
    fn test_requests() {
        let db = create_test_db("test_requests");
        let animal = sample_animal("a1");
        let mut request = sample_request("r1", "a1");

        // Insert animal first (for foreign key)
        db.insert_animal(&animal).unwrap();

        // Test empty query initially
        let requests = db.query_all_adoption_requests().unwrap();
        assert_eq!(requests.len(), 0);

        // Test insert
        db.insert_adoption_request(&request).unwrap();

        // Test query all after insert
        let requests = db.query_all_adoption_requests().unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].id, "r1");
        assert_eq!(requests[0].name, "Jira Pit");

        // Test query by ID
        let found = db.query_adoption_request_by_id("r1").unwrap().unwrap();
        assert_eq!(found.id, "r1");
        assert_eq!(found.animal_id, "a1");
        assert_eq!(found.email, "jira.pit@gmail.com");
        assert_eq!(found.adoption_timestamp, 0);

        // Test query non-existent
        let not_found = db.query_adoption_request_by_id("nonexistent").unwrap();
        assert!(not_found.is_none());

        // Test update
        request.name = "Non Prajogo".to_string();
        request.status = RequestStatus::Approved;
        request.adoption_timestamp = Utc::now().timestamp();
        let updated = db.update_adoption_request(&request).unwrap();
        assert!(updated);

        let found = db.query_adoption_request_by_id("r1").unwrap().unwrap();
        assert_eq!(found.name, "Non Prajogo");
        assert_eq!(found.status, RequestStatus::Approved);
        assert_ne!(found.adoption_timestamp, 0);

        // Test update non-existent
        let fake_request = sample_request("fake", "a1");
        let not_updated = db.update_adoption_request(&fake_request).unwrap();
        assert!(!not_updated);

        // Test delete
        let deleted = db.delete_adoption_request("r1").unwrap();
        assert!(deleted);

        let not_found = db.query_adoption_request_by_id("r1").unwrap();
        assert!(not_found.is_none());

        // Test delete non-existent
        let not_deleted = db.delete_adoption_request("nonexistent").unwrap();
        assert!(!not_deleted);
    }

    #[test]
    fn test_requests_duplicate_insert() {
        let db = create_test_db("test_requests_duplicate_insert");
        let animal = sample_animal("a1");
        let request = sample_request("r1", "a1");

        db.insert_animal(&animal).unwrap();

        // First insert should succeed
        db.insert_adoption_request(&request).unwrap();

        // Duplicate insert should fail
        let duplicate_result = db.insert_adoption_request(&request);
        assert!(duplicate_result.is_err());
    }

    #[test]
    fn test_requests_foreign_key() {
        let db = create_test_db("test_requests_foreign_key");
        let request = sample_request("r1", "nonexistent_animal");

        // Should fail due to foreign key constraint
        let result = db.insert_adoption_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_requests_multiple_records() {
        let db = create_test_db("test_requests_multiple_records");
        let animal = sample_animal("a1");
        let request1 = sample_request("r1", "a1");
        let mut request2 = sample_request("r2", "a1");
        request2.name = "Non Prajogo".to_string();
        request2.email = "non.prajogo@gmail.com".to_string();
        request2.tel_number = "9876543210".to_string();
        request2.annual_income = "50000".to_string();

        db.insert_animal(&animal).unwrap();
        db.insert_adoption_request(&request1).unwrap();
        db.insert_adoption_request(&request2).unwrap();

        // Verify all are returned
        let requests = db.query_all_adoption_requests().unwrap();
        assert_eq!(requests.len(), 2);

        let ids: Vec<&str> = requests.iter().map(|r| r.id.as_str()).collect();
        assert!(ids.contains(&"r1"));
        assert!(ids.contains(&"r2"));
    }
}
