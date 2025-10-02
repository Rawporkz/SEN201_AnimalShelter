//
// lib.rs
//
// This file defines all functions that can be called from the frontend (TypeScript)
// Other modules/services are expoerted to the front end through this file and no where else
//

mod authentication_service;
mod database_service;
mod file_service;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// TODO: Delete this example command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
