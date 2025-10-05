//
// database_service/test.rs
//
// This file contains unit tests for the database service module.
//

#[cfg(test)]
mod database_service_tests {
    use super::super::{
        types::{
            AdoptionRequest, Animal, AnimalStatus, RequestStatus, UserAuthentication, UserRole,
        },
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
            birth_year: 2004,
            num_people: 2,
            num_children: 0,
            request_timestamp: Utc::now().timestamp(),
            adoption_timestamp: 0,
            status: RequestStatus::Pending,
        }
    }

    /// Helper function to create a sample user authentication for testing
    ///
    /// # Arguments
    /// * `username` - Username for the authentication record
    /// * `password_hash` - Hashed password for the record
    /// * `role` - User role for the record
    ///
    /// # Returns
    /// * `UserAuthentication` - Sample user authentication with test data
    fn sample_user_auth(username: &str, password_hash: &str, role: UserRole) -> UserAuthentication {
        UserAuthentication {
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            role,
        }
    }

    // ==================== USER AUTHENTICATION TESTS ====================

    #[test]
    fn test_user_authentication() {
        let db = create_test_db("test_user_authentication");
        let user = sample_user_auth("admin", "hashed_password_123", UserRole::Staff);

        // Test get password hash for non-existent user
        let password_hash = db.get_password_hash("admin").unwrap();
        assert!(password_hash.is_none());

        // Test get role for non-existent user
        let role = db.get_user_role("admin").unwrap();
        assert!(role.is_none());

        // Test insert user
        db.insert_user(&user).unwrap();

        // Test get password hash after insert
        let password_hash = db.get_password_hash("admin").unwrap().unwrap();
        assert_eq!(password_hash, "hashed_password_123");

        // Test get role after insert
        let role = db.get_user_role("admin").unwrap().unwrap();
        assert_eq!(role, UserRole::Staff);

        // Test get password hash for non-existent user
        let not_found = db.get_password_hash("nonexistent").unwrap();
        assert!(not_found.is_none());

        // Test get role for non-existent user
        let role_not_found = db.get_user_role("nonexistent").unwrap();
        assert!(role_not_found.is_none());

        // Test delete user
        let deleted = db.delete_user("admin").unwrap();
        assert!(deleted);

        let not_found = db.get_password_hash("admin").unwrap();
        assert!(not_found.is_none());

        let role_not_found = db.get_user_role("admin").unwrap();
        assert!(role_not_found.is_none());

        // Test delete non-existent user
        let not_deleted = db.delete_user("nonexistent").unwrap();
        assert!(!not_deleted);
    }

    #[test]
    fn test_user_authentication_duplicate_insert() {
        let db = create_test_db("test_user_authentication_duplicate_insert");
        let user = sample_user_auth("admin", "hashed_password_123", UserRole::Staff);

        // First insert should succeed
        db.insert_user(&user).unwrap();

        // Duplicate insert should fail
        let duplicate_result = db.insert_user(&user);
        assert!(duplicate_result.is_err());
    }

    #[test]
    fn test_user_authentication_multiple_users() {
        let db = create_test_db("test_user_authentication_multiple_users");
        let user1 = sample_user_auth("admin", "admin_hash_123", UserRole::Staff);
        let user2 = sample_user_auth("user", "user_hash_456", UserRole::Customer);
        let user3 = sample_user_auth("guest", "guest_hash_789", UserRole::Customer);

        // Insert multiple users
        db.insert_user(&user1).unwrap();
        db.insert_user(&user2).unwrap();
        db.insert_user(&user3).unwrap();

        // Verify each user's password hash and role
        let admin_hash = db.get_password_hash("admin").unwrap().unwrap();
        assert_eq!(admin_hash, "admin_hash_123");
        let admin_role = db.get_user_role("admin").unwrap().unwrap();
        assert_eq!(admin_role, UserRole::Staff);

        let user_hash = db.get_password_hash("user").unwrap().unwrap();
        assert_eq!(user_hash, "user_hash_456");
        let user_role = db.get_user_role("user").unwrap().unwrap();
        assert_eq!(user_role, UserRole::Customer);

        let guest_hash = db.get_password_hash("guest").unwrap().unwrap();
        assert_eq!(guest_hash, "guest_hash_789");
        let guest_role = db.get_user_role("guest").unwrap().unwrap();
        assert_eq!(guest_role, UserRole::Customer);

        // Delete one user and verify others still exist
        let deleted = db.delete_user("user").unwrap();
        assert!(deleted);

        let not_found = db.get_password_hash("user").unwrap();
        assert!(not_found.is_none());
        let role_not_found = db.get_user_role("user").unwrap();
        assert!(role_not_found.is_none());

        let admin_still_exists = db.get_password_hash("admin").unwrap();
        assert!(admin_still_exists.is_some());
        let admin_role_exists = db.get_user_role("admin").unwrap();
        assert!(admin_role_exists.is_some());

        let guest_still_exists = db.get_password_hash("guest").unwrap();
        assert!(guest_still_exists.is_some());
        let guest_role_exists = db.get_user_role("guest").unwrap();
        assert!(guest_role_exists.is_some());
    }

    #[test]
    fn test_user_authentication_empty_username() {
        let db = create_test_db("test_user_authentication_empty_username");
        let user = sample_user_auth("", "password_hash", UserRole::Customer);

        // Insert should succeed (empty username is valid in SQLite)
        db.insert_user(&user).unwrap();

        // Should be able to retrieve the password hash for empty username
        let password_hash = db.get_password_hash("").unwrap().unwrap();
        assert_eq!(password_hash, "password_hash");

        // Should be able to retrieve the role for empty username
        let role = db.get_user_role("").unwrap().unwrap();
        assert_eq!(role, UserRole::Customer);
    }

    #[test]
    fn test_user_role_variants() {
        let db = create_test_db("test_user_role_variants");
        let staff_user = sample_user_auth("staff_user", "staff_hash", UserRole::Staff);
        let customer_user = sample_user_auth("customer_user", "customer_hash", UserRole::Customer);

        // Insert users with different roles
        db.insert_user(&staff_user).unwrap();
        db.insert_user(&customer_user).unwrap();

        // Verify roles are stored and retrieved correctly
        let staff_role = db.get_user_role("staff_user").unwrap().unwrap();
        assert_eq!(staff_role, UserRole::Staff);

        let customer_role = db.get_user_role("customer_user").unwrap().unwrap();
        assert_eq!(customer_role, UserRole::Customer);

        // Verify roles are different
        assert_ne!(staff_role, customer_role);
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
        request2.birth_year = 2005;

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
