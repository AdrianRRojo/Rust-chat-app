// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use serde::Serialize;
use crate::auth::models::User;
mod auth;
// use std::io;
// #[derive(Debug, Serialize)]
// struct User {
//     id: i32,
//     username: String,
// }
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn login(username: &str, password: &str) -> Result<Vec<User>, String> {
    match auth::login::login_user(&username, &password) {
        Ok(Some(user)) => Ok(vec![user]), // Convert the Option<User> to Vec<User>
        Ok(None) => Err("No user found".to_string()),
        Err(e) => Err(e),
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
