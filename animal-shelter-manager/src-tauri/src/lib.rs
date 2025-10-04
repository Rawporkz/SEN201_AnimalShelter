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
use file_service::FileService;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

/// Global state of the app
#[derive(Default)]
struct AppState {
    /// Service for handling file operations
    file_service: Option<FileService>,
}

/// Lazily initializes the FileService if it hasn't been created yet
///
/// # Arguments
/// * `state` - Mutable reference to the application state
/// * `app_handle` - Reference to the Tauri application handle
async fn lazy_init_file_service(
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
    lazy_init_file_service(&mut state_guard, &app_handle).await?;

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
    lazy_init_file_service(&mut state_guard, &app_handle).await?;

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
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![upload_file, delete_file])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
