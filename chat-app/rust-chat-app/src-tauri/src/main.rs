// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
// use std::io;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn login(username: &str, password: &str) -> Result<String, String> {
    // format!("Hello, {}, {}! You've been greeted from Rust!", username, password)
    match auth::login::login_user(&username, &password) {
        Ok(_) => Ok("Login successful".to_string()),
        Err(e) => Err(format!("Login failed: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
