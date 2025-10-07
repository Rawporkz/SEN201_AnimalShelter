//
// main.rs
//
// This file started the Tauri application.
// It does not contain any business logic.
//

// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    // Start the Tauri application
    animal_shelter_manager_lib::run()
}
