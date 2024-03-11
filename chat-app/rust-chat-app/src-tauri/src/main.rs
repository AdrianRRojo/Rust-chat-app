// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use serde::Serialize;
use crate::auth::models::User;
use crate::auth::models::Chatrooms;
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


// fn load_chats(userId: i32) -> Result<Vec<Chatrooms>, String> {
//     // auth::chatroom::load_chats(&user_id)
//     match auth::chatroom::load_chats(userId) {
//         Ok(Some(chats)) => Ok(vec![chats]), // Convert the Option<chats> to Vec<User>
//         Ok(None) => Err("No chats found".to_string()),
//         Err(e) => Err(e),
//     }

#[tauri::command]
fn load_chats(user_id: i32) -> Result<Vec<Chatrooms>, String> {        
    auth::chatroom::load_chats(user_id)
}

#[tauri::command]
fn create_chat_room(name: String) -> Result<String, String> {
    // auth::chatroom::load_chats(&user_id)
    let new_name = name; 
    match auth::chatroom::create_chat_room(new_name) {
        Ok(string) => Ok(string), // Convert the Option<chats> to Vec<User>
        // Ok(None) => Err("No chats found".to_string()),
        Err(e) => Err(e),
    }

}
#[tauri::command]
fn join_chat_room(user_id: i32, access_code: String) -> Result<Vec<Chatrooms>, String> {
    // auth::chatroom::load_chats(&user_id)
    // let mut new_name = name; 
    match auth::chatroom::join_chat_room(user_id, access_code) {
        Ok(Some(chats)) => Ok(vec![chats]), // Convert the Option<chats> to Vec<User>
        Ok(None) => Err("No chats found".to_string()),
        Err(e) => Err(e),
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login, load_chats,create_chat_room, join_chat_room])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}