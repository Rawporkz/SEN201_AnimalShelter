//
// lib.rs
//
// This file defines all functions that can be called from the frontend (TypeScript)
// Other modules/services are expoerted to the front end through this file and no where else
//

mod authentication_service;
mod database_service;
mod file_service;

use anyhow::Result;
use authentication_service::{
    types::{LoginResult, UserRole},
    AuthenticationService, CurrentUser,
};
use database_service::{
    types::{AdoptionRequest, Animal, AnimalSummary, FilterCriteria, FilterValue},
    DatabaseService,
};
use file_service::FileService;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

/// Global state of the app
#[derive(Default)]
struct AppState {
    /// Service for handling file operations
    file_service: Option<FileService>,
    /// Service for handling database operations
    database_service: Option<DatabaseService>,
    /// Service for handling authentication operations
    authentication_service: Option<AuthenticationService>,
}

/// Lazily initializes the FileService if it hasn't been created yet
///
/// # Arguments
/// * `state` - Mutable reference to the application state
/// * `app_handle` - Reference to the Tauri application handle
async fn init_file_service_once(
    state: &mut AppState,
    app_handle: &AppHandle,
) -> Result<(), String> {
    if state.file_service.is_none() {
        log::info!("Initializing FileService");

        // Initialize FileService with application app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?;
        match FileService::new(app_data_dir) {
            Ok(service) => state.file_service = Some(service),
            Err(e) => return Err(format!("Failed to create FileService: {}", e)),
        }
    }
    Ok(())
}

/// Lazily initializes the DatabaseService if it hasn't been created yet
///
/// # Arguments
/// * `state` - Mutable reference to the application state
/// * `app_handle` - Reference to the Tauri application handle
async fn init_database_service_once(
    state: &mut AppState,
    app_handle: &AppHandle,
) -> Result<(), String> {
    if state.database_service.is_none() {
        log::info!("Initializing DatabaseService");

        // Initialize DatabaseService with application app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?;
        let db_path = app_data_dir.join("animal_shelter.db");

        match DatabaseService::new(db_path) {
            Ok(service) => state.database_service = Some(service),
            Err(e) => return Err(format!("Failed to create DatabaseService: {}", e)),
        }
    }
    Ok(())
}

/// Lazily initializes the AuthenticationService if it hasn't been created yet
///
/// # Arguments
/// * `state` - Mutable reference to the application state
/// * `app_handle` - Reference to the Tauri application handle
async fn init_authentication_service_once(
    state: &mut AppState,
    app_handle: &AppHandle,
) -> Result<(), String> {
    if state.authentication_service.is_none() {
        log::info!("Initializing AuthenticationService");

        // Initialize AuthenticationService with its own database in app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?;
        let auth_db_path = app_data_dir.join("authentication.db");

        match AuthenticationService::new(auth_db_path) {
            Ok(service) => state.authentication_service = Some(service),
            Err(e) => return Err(format!("Failed to create AuthenticationService: {}", e)),
        }
    }
    Ok(())
}

// ==================== ANIMAL TABLE COMMANDS ====================

/// Command to retrieve animals from the database, with optional filtering
///
/// # Arguments
/// * `filters` - Optional map of filter criteria and values
///
/// # Returns
/// * `Ok(Vec<AnimalSummary>)` - List of animal summaries if successful
/// * `Err(String)` - An error message if the query fails
#[tauri::command]
async fn get_animals(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    filters: Option<HashMap<FilterCriteria, Option<FilterValue>>>,
) -> Result<Vec<AnimalSummary>, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Query animals with filters
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .query_animals(filters)
    {
        Ok(animals) => Ok(animals),
        Err(e) => Err(format!("Failed to retrieve animals: {}", e)),
    }
}

/// Command to retrieve a specific animal by ID
///
/// # Arguments
/// * `animal_id` - The ID of the animal to retrieve
///
/// # Returns
/// * `Ok(Some(Animal))` - The animal data if found
/// * `Ok(None)` - If no animal with the given ID exists
/// * `Err(String)` - An error message if the query fails
#[tauri::command]
async fn get_animal_by_id(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    animal_id: String,
) -> Result<Option<Animal>, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Query animal by ID
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .query_animal_by_id(&animal_id)
    {
        Ok(animal) => Ok(animal),
        Err(e) => Err(format!(
            "Failed to retrieve animal with ID {}: {}",
            animal_id, e
        )),
    }
}

/// Command to insert a new animal into the database
///
/// # Arguments
/// * `animal` - The animal data to insert
///
/// # Returns
/// * `Ok(())` - If the animal was successfully inserted
/// * `Err(String)` - An error message if the insertion fails
#[tauri::command]
async fn create_animal(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    animal: Animal,
) -> Result<(), String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Insert animal
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .insert_animal(&animal)
    {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to create animal: {}", e)),
    }
}

/// Command to update an existing animal in the database
///
/// # Arguments
/// * `animal` - The updated animal data
///
/// # Returns
/// * `Ok(bool)` - True if animal was found and updated, false if not found
/// * `Err(String)` - An error message if the update fails
#[tauri::command]
async fn update_animal(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    animal: Animal,
) -> Result<bool, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Update animal
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .update_animal(&animal)
    {
        Ok(updated) => Ok(updated),
        Err(e) => Err(format!("Failed to update animal: {}", e)),
    }
}

/// Command to delete an animal from the database
///
/// # Arguments
/// * `animal_id` - The ID of the animal to delete
///
/// # Returns
/// * `Ok(bool)` - True if animal was found and deleted, false if not found
/// * `Err(String)` - An error message if the deletion fails
#[tauri::command]
async fn delete_animal(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    animal_id: String,
) -> Result<bool, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Delete animal
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .delete_animal(&animal_id)
    {
        Ok(deleted) => Ok(deleted),
        Err(e) => Err(format!(
            "Failed to delete animal with ID {}: {}",
            animal_id, e
        )),
    }
}

// ==================== ADOPTION REQUEST TABLE COMMANDS ====================

/// Command to retrieve a specific adoption request by ID
///
/// # Arguments
/// * `request_id` - The ID of the adoption request to retrieve
///
/// # Returns
/// * `Ok(Some(AdoptionRequest))` - The adoption request data if found
/// * `Ok(None)` - If no request with the given ID exists
/// * `Err(String)` - An error message if the query fails
#[tauri::command]
async fn get_adoption_request_by_id(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    request_id: String,
) -> Result<Option<AdoptionRequest>, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Query adoption request by ID
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .query_adoption_request_by_id(&request_id)
    {
        Ok(request) => Ok(request),
        Err(e) => Err(format!(
            "Failed to retrieve adoption request with ID {}: {}",
            request_id, e
        )),
    }
}

/// Command to insert a new adoption request into the database
///
/// # Arguments
/// * `request` - The adoption request data to insert
///
/// # Returns
/// * `Ok(())` - If the adoption request was successfully inserted
/// * `Err(String)` - An error message if the insertion fails
#[tauri::command]
async fn create_adoption_request(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    request: AdoptionRequest,
) -> Result<(), String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Insert adoption request
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .insert_adoption_request(&request)
    {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to create adoption request: {}", e)),
    }
}

/// Command to update an existing adoption request in the database
///
/// # Arguments
/// * `request` - The updated adoption request data
///
/// # Returns
/// * `Ok(bool)` - True if request was found and updated, false if not found
/// * `Err(String)` - An error message if the update fails
#[tauri::command]
async fn update_adoption_request(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    request: AdoptionRequest,
) -> Result<bool, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Update adoption request
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .update_adoption_request(&request)
    {
        Ok(updated) => Ok(updated),
        Err(e) => Err(format!("Failed to update adoption request: {}", e)),
    }
}

/// Command to delete an adoption request from the database
///
/// # Arguments
/// * `request_id` - The ID of the adoption request to delete
///
/// # Returns
/// * `Ok(bool)` - True if request was found and deleted, false if not found
/// * `Err(String)` - An error message if the deletion fails
#[tauri::command]
async fn delete_adoption_request(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    request_id: String,
) -> Result<bool, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Delete adoption request
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .delete_adoption_request(&request_id)
    {
        Ok(deleted) => Ok(deleted),
        Err(e) => Err(format!(
            "Failed to delete adoption request with ID {}: {}",
            request_id, e
        )),
    }
}

/// Command to retrieve all adoption requests from the database for a specific animal ID
///
/// # Arguments
/// * `animal_id` - The ID of the animal to retrieve requests for
///
/// # Returns
/// * `Ok(Vec<AdoptionRequest>)` - List of adoption requests if successful
/// * `Err(String)` - An error message if the query fails
#[tauri::command]
async fn get_adoption_requests_by_animal_id(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    animal_id: String,
) -> Result<Vec<AdoptionRequest>, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Query adoption requests by animal ID
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .query_adoption_requests_by_animal_id(&animal_id)
    {
        Ok(requests) => Ok(requests),
        Err(e) => Err(format!(
            "Failed to retrieve adoption requests for animal ID {}: {}",
            animal_id, e
        )),
    }
}

/// Command to retrieve all adoption requests from the database for a specific user name
///
/// # Arguments
/// * `username` - The user name to retrieve requests for
///
/// # Returns
/// * `Ok(Vec<AdoptionRequest>)` - List of adoption requests if successful
/// * `Err(String)` - An error message if the query fails
#[tauri::command]
async fn get_adoption_requests_by_username(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    username: String,
) -> Result<Vec<AdoptionRequest>, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the database service
    init_database_service_once(&mut state_guard, &app_handle).await?;

    // Query adoption requests by user name
    match state_guard
        .database_service
        .as_ref()
        .unwrap()
        .query_adoption_requests_by_username(&username)
    {
        Ok(requests) => Ok(requests),
        Err(e) => Err(format!(
            "Failed to retrieve adoption requests for user name {}: {}",
            username, e
        )),
    }
}

// ==================== AUTHENTICATION COMMANDS ====================

/// Command to register a new user account
///
/// # Arguments
/// * `username` - Username for the new account
/// * `password` - Password for the new account
/// * `role` - Role to assign to the user (Staff or Customer)
///
/// # Returns
/// * `Ok(())` - If the user was successfully registered and logged in
/// * `Err(String)` - An error message if registration fails
#[tauri::command]
async fn sign_up(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    username: String,
    password: String,
    role: UserRole,
) -> Result<(), String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the authentication service
    init_authentication_service_once(&mut state_guard, &app_handle).await?;

    // Get reference to authentication service
    let auth_service = state_guard.authentication_service.as_mut().unwrap();

    // Register user with new account
    let result = auth_service.sign_up(&username, &password, role);

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to register user: {}", e)),
    }
}

/// Command to log in an existing user
///
/// # Arguments
/// * `username` - Username to log in
/// * `password` - Password for authentication
///
/// # Returns
/// * `Ok(LoginResult)` - Login result indicating success, invalid password, or user not found
/// * `Err(String)` - An error message if login process fails
#[tauri::command]
async fn log_in(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    username: String,
    password: String,
) -> Result<LoginResult, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the authentication service
    init_authentication_service_once(&mut state_guard, &app_handle).await?;

    // Get reference to authentication service
    let auth_service = state_guard.authentication_service.as_mut().unwrap();

    // Authenticate user credentials
    let result = auth_service.log_in(&username, &password);

    match result {
        Ok(login_result) => Ok(login_result),
        Err(e) => Err(format!("Failed to log in: {}", e)),
    }
}

/// Command to get current logged-in user information
///
/// # Returns
/// * `Ok(Some(CurrentUser))` - Current user info if logged in
/// * `Ok(None)` - If no user is currently logged in
/// * `Err(String)` - An error message if retrieval fails
#[tauri::command]
async fn get_current_user(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
) -> Result<Option<CurrentUser>, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the authentication service
    init_authentication_service_once(&mut state_guard, &app_handle).await?;

    // Get reference to authentication service
    let auth_service = state_guard.authentication_service.as_ref().unwrap();

    // Get current user
    let result = auth_service.get_current_user();

    match result {
        Ok(user) => Ok(user),
        Err(e) => Err(format!("Failed to get current user: {}", e)),
    }
}

/// Command to log out the current user
///
/// # Returns
/// * `Ok(())` - Always succeeds
#[tauri::command]
async fn log_out(state: State<'_, Mutex<AppState>>, app_handle: AppHandle) -> Result<(), String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the authentication service
    init_authentication_service_once(&mut state_guard, &app_handle).await?;

    // Log out user
    state_guard
        .authentication_service
        .as_mut()
        .unwrap()
        .log_out();

    Ok(())
}

// ==================== FILE SERVICE COMMANDS ====================

/// Command to upload a file selected by the user
///
/// # Returns
/// * `Ok(Some(PathBuf))` - The path of the uploaded file if successful
/// * `Ok(None)` - If the user cancels the file selection
/// * `Err(String)` - An error message if the upload fails
#[tauri::command]
async fn upload_file(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
) -> Result<Option<PathBuf>, String> {
    // Lock the state for safe concurrent access
    let mut state_guard = state.lock().await;

    // Lazily initialize the file service
    init_file_service_once(&mut state_guard, &app_handle).await?;

    // Perform file upload
    match state_guard
        .file_service
        .as_ref()
        .unwrap()
        .upload_file(&app_handle)
        .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to upload file: {}", e)),
    }
}

/// Command to delete a file from the specified path
///
/// # Arguments
/// * `file_path` - The path of the file to be deleted
#[tauri::command]
async fn delete_file(
    state: State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    file_path: String,
) -> Result<(), String> {
    // Lock the state for safe concurrent accesss
    let mut state_guard = state.lock().await;

    // Lazily initialize the file service
    init_file_service_once(&mut state_guard, &app_handle).await?;

    // Perform file deletion
    match state_guard
        .file_service
        .as_ref()
        .unwrap()
        .delete_file(file_path)
        .await
    {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to delete file: {}", e)),
    }
}

/// Runs the Tauri application
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            // Authentication commands
            sign_up,
            log_in,
            get_current_user,
            log_out,
            // Animal commands
            get_animals,
            get_animal_by_id,
            create_animal,
            update_animal,
            delete_animal,
            // Adoption request commands
            get_adoption_request_by_id,
            get_adoption_requests_by_animal_id,
            get_adoption_requests_by_username,
            create_adoption_request,
            update_adoption_request,
            delete_adoption_request,
            // File commands
            upload_file,
            delete_file
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
