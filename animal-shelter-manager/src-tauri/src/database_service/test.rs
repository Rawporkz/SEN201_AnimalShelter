//
// database_service/test.rs
//
// This file contains unit tests for the database service module.
//

#[cfg(test)]
mod database_service_tests {
    use super::super::{
        types::{
            AdoptionRequest, Animal, AnimalStatus, FilterCriteria, FilterValue, RequestStatus,
        },
        DatabaseService,
    };
    use chrono::Utc;
    use std::collections::HashMap;
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
            birth_month: Some(6),
            birth_year: Some(2020),
            neutered: true,
            admission_timestamp: Utc::now().timestamp(),
            status: AnimalStatus::Available,
            image_path: Some("/test/images/buddy.jpg".to_string()),
            appearance: "Golden coat with friendly eyes".to_string(),
            bio: "Buddy is a friendly and energetic dog who loves playing fetch and going on walks. He gets along well with children and other pets.".to_string(),
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
            username: "JiraPit".to_string(),
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
            country: "Thailand".to_string(),
        }
    }

    // ==================== ANIMALS TESTS ====================

    #[test]
    fn test_animals() {
        let db = create_test_db("test_animals");
        let mut animal = sample_animal("a1");

        // Test empty query initially
        let animals = db.query_animals(None).unwrap();
        assert_eq!(animals.len(), 0);

        // Test insert
        db.insert_animal(&animal).unwrap();

        // Test query all after insert
        let animals = db.query_animals(None).unwrap();
        assert_eq!(animals.len(), 1);
        assert_eq!(animals[0].id, "a1");
        assert_eq!(animals[0].name, "Buddy");
        assert_eq!(
            animals[0].image_path,
            Some("/test/images/buddy.jpg".to_string())
        );

        // Test query by ID
        let found = db.query_animal_by_id("a1").unwrap().unwrap();
        assert_eq!(found.id, "a1");
        assert_eq!(found.breed, "Golden Retriever");
        assert!(found.neutered);
        assert_eq!(found.image_path, Some("/test/images/buddy.jpg".to_string()));
        assert_eq!(found.appearance, "Golden coat with friendly eyes");
        assert_eq!(found.bio, "Buddy is a friendly and energetic dog who loves playing fetch and going on walks. He gets along well with children and other pets.");

        // Test query non-existent
        let not_found = db.query_animal_by_id("nonexistent").unwrap();
        assert!(not_found.is_none());

        // Test update
        animal.name = "Updated Buddy".to_string();
        animal.status = AnimalStatus::Adopted;
        animal.image_path = Some("/test/images/updated_buddy.jpg".to_string());
        animal.appearance = "Updated golden coat with wise eyes".to_string();
        animal.bio = "Updated bio: Buddy is now a mature and well-trained dog.".to_string();
        let updated = db.update_animal(&animal).unwrap();
        assert!(updated);

        let found = db.query_animal_by_id("a1").unwrap().unwrap();
        assert_eq!(found.name, "Updated Buddy");
        assert_eq!(found.status, AnimalStatus::Adopted);
        assert_eq!(
            found.image_path,
            Some("/test/images/updated_buddy.jpg".to_string())
        );
        assert_eq!(found.appearance, "Updated golden coat with wise eyes");
        assert_eq!(
            found.bio,
            "Updated bio: Buddy is now a mature and well-trained dog."
        );

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
        animal2.image_path = Some("/test/images/max.jpg".to_string());
        animal2.appearance = "Sleek black fur with green eyes".to_string();
        animal2.bio =
            "Max is a calm and independent cat who enjoys sunny spots and gentle pets.".to_string();

        // Insert multiple animals
        db.insert_animal(&animal1).unwrap();
        db.insert_animal(&animal2).unwrap();

        // Verify all are returned
        let animals = db.query_animals(None).unwrap();
        assert_eq!(animals.len(), 2);

        let ids: Vec<&str> = animals.iter().map(|a| a.id.as_str()).collect();
        assert!(ids.contains(&"a1"));
        assert!(ids.contains(&"a2"));
    }

    #[test]
    fn test_animals_filter() {
        let db = create_test_db("test_animals_filter");

        let mut animal1 = sample_animal("a1");
        animal1.name = "Buddy".to_string();
        animal1.specie = "Dog".to_string();
        animal1.breed = "Golden Retriever".to_string();
        animal1.sex = "Male".to_string();
        animal1.status = AnimalStatus::Available;
        animal1.admission_timestamp = Utc::now().timestamp() - 86400 * 30; // 30 days ago

        let mut animal2 = sample_animal("a2");
        animal2.name = "Lucy".to_string();
        animal2.specie = "Cat".to_string();
        animal2.breed = "Siamese".to_string();
        animal2.sex = "Female".to_string();
        animal2.status = AnimalStatus::Available;
        animal2.admission_timestamp = Utc::now().timestamp() - 86400; // 1 day ago

        let mut animal3 = sample_animal("a3");
        animal3.name = "Rocky".to_string();
        animal3.specie = "Dog".to_string();
        animal3.breed = "German Shepherd".to_string();
        animal3.sex = "Male".to_string();
        animal3.status = AnimalStatus::Adopted;
        animal3.admission_timestamp = Utc::now().timestamp() - 86400 * 60; // 60 days ago

        db.insert_animal(&animal1).unwrap();
        db.insert_animal(&animal2).unwrap();
        db.insert_animal(&animal3).unwrap();

        // For adoption date filter test
        let mut animal4 = sample_animal("a4");
        animal4.name = "Milo".to_string();
        animal4.specie = "Dog".to_string();
        animal4.breed = "Labrador".to_string();
        animal4.sex = "Male".to_string();
        animal4.status = AnimalStatus::Adopted;
        animal4.admission_timestamp = Utc::now().timestamp() - 86400 * 10; // 10 days ago
        db.insert_animal(&animal4).unwrap();

        let mut request1 = sample_request("r1", "a4");
        request1.status = RequestStatus::Approved;
        request1.adoption_timestamp = Utc::now().timestamp() - 86400 * 2; // 2 days ago
        db.insert_adoption_request(&request1).unwrap();

        // another adopted animal but outside date range
        let mut animal5 = sample_animal("a5");
        animal5.name = "Coco".to_string();
        animal5.specie = "Cat".to_string();
        animal5.breed = "Persian".to_string();
        animal5.sex = "Female".to_string();
        animal5.status = AnimalStatus::Adopted;
        animal5.admission_timestamp = Utc::now().timestamp() - 86400 * 20; // 20 days ago
        db.insert_animal(&animal5).unwrap();

        let mut request2 = sample_request("r2", "a5");
        request2.status = RequestStatus::Approved;
        request2.adoption_timestamp = Utc::now().timestamp() - 86400 * 15; // 15 days ago
        db.insert_adoption_request(&request2).unwrap();

        // Test filter by status
        let mut filters = HashMap::new();
        filters.insert(
            FilterCriteria::Status,
            Some(FilterValue::ChooseMany(vec![
                AnimalStatus::Available.to_string()
            ])),
        );
        let animals = db.query_animals(Some(filters.clone())).unwrap();
        assert_eq!(animals.len(), 2);
        assert!(animals.iter().any(|a| a.id == "a1"));
        assert!(animals.iter().any(|a| a.id == "a2"));

        // Test filter by sex
        filters.clear();
        filters.insert(
            FilterCriteria::Sex,
            Some(FilterValue::ChooseMany(vec!["Male".to_string()])),
        );
        let animals = db.query_animals(Some(filters.clone())).unwrap();
        assert_eq!(animals.len(), 3);
        assert!(animals.iter().any(|a| a.id == "a1"));
        assert!(animals.iter().any(|a| a.id == "a3"));
        assert!(animals.iter().any(|a| a.id == "a4"));

        // Test filter by species and breeds
        filters.clear();
        let mut species_map = HashMap::new();
        species_map.insert("Dog".to_string(), vec!["Golden Retriever".to_string()]);
        filters.insert(
            FilterCriteria::SpeciesAndBreeds,
            Some(FilterValue::NestedChooseMany(species_map)),
        );
        let animals = db.query_animals(Some(filters.clone())).unwrap();
        assert_eq!(animals.len(), 1);
        assert_eq!(animals[0].id, "a1");

        // Test combined filters (status and sex)
        filters.clear();
        filters.insert(
            FilterCriteria::Status,
            Some(FilterValue::ChooseMany(vec![
                AnimalStatus::Available.to_string()
            ])),
        );
        filters.insert(
            FilterCriteria::Sex,
            Some(FilterValue::ChooseMany(vec!["Female".to_string()])),
        );
        let animals = db.query_animals(Some(filters.clone())).unwrap();
        assert_eq!(animals.len(), 1);
        assert_eq!(animals[0].id, "a2");

        // Test filter by admission date
        filters.clear();
        filters.insert(
            FilterCriteria::AdmissionDate,
            Some(FilterValue::ChooseOne("this_week".to_string())),
        );
        let animals = db.query_animals(Some(filters.clone())).unwrap();
        assert_eq!(animals.len(), 1);
        assert!(animals.iter().any(|a| a.id == "a2"));

        // Test filter by adoption date
        filters.clear();
        filters.insert(
            FilterCriteria::AdoptionDate,
            Some(FilterValue::ChooseOne("this_week".to_string())),
        );
        let animals = db.query_animals(Some(filters)).unwrap();
        assert_eq!(animals.len(), 1);
        assert!(animals.iter().any(|a| a.id == "a4"));
    }

    // ==================== ADOPTION REQUESTS TESTS ====================

    #[test]
    fn test_requests() {
        let db = create_test_db("test_requests");
        let animal = sample_animal("a1");
        let mut request = sample_request("r1", "a1");

        // Insert animal first (for foreign key)
        db.insert_animal(&animal).unwrap();

        // Test insert
        db.insert_adoption_request(&request).unwrap();

        // Test query by ID
        let found = db.query_adoption_request_by_id("r1").unwrap().unwrap();
        assert_eq!(found.id, "r1");
        assert_eq!(found.animal_id, "a1");
        assert_eq!(found.email, "jira.pit@gmail.com");
        assert_eq!(found.adoption_timestamp, 0);
        assert_eq!(found.country, "Thailand");

        // Test query non-existent
        let not_found = db.query_adoption_request_by_id("nonexistent").unwrap();
        assert!(not_found.is_none());

        // Test update
        request.name = "Non Prajogo".to_string();
        request.status = RequestStatus::Approved;
        request.adoption_timestamp = Utc::now().timestamp();
        request.country = "Indonesia".to_string();
        let updated = db.update_adoption_request(&request).unwrap();
        assert!(updated);

        let found = db.query_adoption_request_by_id("r1").unwrap().unwrap();
        assert_eq!(found.name, "Non Prajogo");
        assert_eq!(found.status, RequestStatus::Approved);
        assert_ne!(found.adoption_timestamp, 0);
        assert_eq!(found.country, "Indonesia");

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
    fn test_query_adoption_requests_by_animal_id() {
        let db = create_test_db("test_query_adoption_requests_by_animal_id");
        let animal1 = sample_animal("a1");
        let animal2 = sample_animal("a2");

        db.insert_animal(&animal1).unwrap();
        db.insert_animal(&animal2).unwrap();

        let request1 = sample_request("r1", "a1");
        let request2 = sample_request("r2", "a1"); // Another request for animal1
        let request3 = sample_request("r3", "a2");

        db.insert_adoption_request(&request1).unwrap();
        db.insert_adoption_request(&request2).unwrap();
        db.insert_adoption_request(&request3).unwrap();

        // Query requests for animal1
        let requests_for_a1 = db.query_adoption_requests_by_animal_id("a1").unwrap();
        assert_eq!(requests_for_a1.len(), 2);
        assert!(requests_for_a1.iter().any(|r| r.id == "r1"));
        assert!(requests_for_a1.iter().any(|r| r.id == "r2"));
        assert_eq!(requests_for_a1[0].animal_id, "a1");
        assert_eq!(requests_for_a1[0].name, "Jira Pit"); // Check a field from AdoptionRequest

        // Query requests for animal2
        let requests_for_a2 = db.query_adoption_requests_by_animal_id("a2").unwrap();
        assert_eq!(requests_for_a2.len(), 1);
        assert!(requests_for_a2.iter().any(|r| r.id == "r3"));
        assert_eq!(requests_for_a2[0].animal_id, "a2");

        // Query for non-existent animal
        let requests_for_nonexistent = db
            .query_adoption_requests_by_animal_id("nonexistent")
            .unwrap();
        assert_eq!(requests_for_nonexistent.len(), 0);
    }
}
