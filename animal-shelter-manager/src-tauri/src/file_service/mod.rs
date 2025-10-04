//
// file_service/mod.rs
//
// This module provides file-related functionality to other components,
// including file upload with user selection dialogs and secure file deletion.
// All file operations are performed within a designated root directory.
//

use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use tokio::fs;

mod test;

/// Service for handling file operations in the application
pub struct FileService {
    /// Root directory where all application files are stored
    root_path: PathBuf,
}

impl FileService {
    /// Creates a new FileService instance with the specified root directory
    ///
    /// # Arguments
    /// * `root_path` - The root directory path where all files will be stored
    ///
    /// # Returns
    /// * `Result<FileService>` - New FileService instance or error
    pub fn new<P: AsRef<Path>>(root_path: P) -> Result<Self> {
        // Convert the root path parameter to a PathBuf for ownership
        let root_path = root_path.as_ref().to_path_buf();

        // Ensure the root directory exists
        if !root_path.exists() {
            std::fs::create_dir_all(&root_path)
                .context(format!("Failed to create root directory: {:?}", root_path))?;
        }

        Ok(FileService { root_path })
    }

    /// Allows user to select and upload a file from their computer
    ///
    /// # Arguments
    /// * `app_handle` - Tauri application handle for accessing dialog plugin
    ///
    /// # Returns
    /// * `Result<Option<PathBuf>>` - Path where the file was saved, or None if cancelled
    pub async fn upload_file(&self, app_handle: &AppHandle) -> Result<Option<PathBuf>> {
        // Open file selection dialog using tokio oneshot channel for async handling
        let (tx, rx) = tokio::sync::oneshot::channel();
        app_handle.dialog().file().pick_file(move |file_path| {
            let _ = tx.send(file_path);
        });

        // Wait for user to select a file or cancel
        let file_path = rx
            .await
            .context("Failed to receive file selection result")?;

        // Handle the selected file
        match file_path {
            Some(selected_path) => {
                // Convert FilePath to PathBuf
                let selected_path_buf = selected_path.into_path()?;

                // Generate unique filename using timestamp in milliseconds
                let timestamp = Utc::now().timestamp_millis();

                // Get file extension from original file
                let extension = selected_path_buf
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");

                let filename = if extension.is_empty() {
                    format!("{}", timestamp)
                } else {
                    format!("{}.{}", timestamp, extension)
                };

                let destination_path = self.root_path.join(filename);

                // Copy the selected file to our storage location
                fs::copy(&selected_path_buf, &destination_path)
                    .await
                    .context(format!(
                        "Failed to copy file from {:?} to {:?}",
                        selected_path_buf, destination_path
                    ))?;

                log::info!(
                    "File uploaded successfully: {:?} -> {:?}",
                    selected_path_buf,
                    destination_path
                );

                Ok(Some(destination_path))
            }
            None => {
                log::info!("File selection was cancelled by user");
                Ok(None)
            }
        }
    }

    /// Deletes a file from the specified path
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to be deleted
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub async fn delete_file<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        let file_path = file_path.as_ref();

        // Check if file exists
        if !file_path.exists() {
            log::error!("File does not exist: {:?}", file_path);
            bail!("File does not exist: {:?}", file_path);
        }

        // Ensure the file is within our root directory for security
        let canonical_file_path = file_path
            .canonicalize()
            .context(format!("Failed to resolve file path: {:?}", file_path))?;
        let canonical_root_path = self
            .root_path
            .canonicalize()
            .context(format!("Failed to resolve root path: {:?}", self.root_path))?;
        if !canonical_file_path.starts_with(&canonical_root_path) {
            bail!(
                "Security violation: Attempted to delete file outside root directory. File: {:?}, Root: {:?}",
                canonical_file_path, canonical_root_path
            );
        }

        // Delete the file
        fs::remove_file(file_path)
            .await
            .context(format!("Failed to delete file: {:?}", file_path))?;

        log::info!("File deleted successfully: {:?}", file_path);
        Ok(())
    }
}
