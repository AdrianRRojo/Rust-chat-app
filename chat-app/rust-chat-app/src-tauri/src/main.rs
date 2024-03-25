// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use serde::Serialize;
use crate::auth::models::User;
use crate::auth::models::Chatrooms;
use crate::auth::models::RoomMsgs;
mod auth;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn login(username: &str, password: &str) -> Result<Vec<User>, String> {
    match auth::login::login_user(&username, &password) {
        Ok(Some(user)) => Ok(vec![user]), // Convert the Option<User> to Vec<User>
        Ok(None) => Err("No user found".to_string()),
        Err(e) => Err(e),
    }
}
#[tauri::command]
fn register(username: &str, password: &str, email: &str) -> Result<Vec<User>, String> {
    match auth::register::register_user(&username, &password, &email) {
        Ok(Some(user)) => Ok(vec![user]), // Convert the Option<User> to Vec<User>
        Ok(None) => Err("No user found".to_string()),
        Err(e) => Err(e),
    }
}
#[tauri::command]
fn load_chats(user_id: i32) -> Result<Vec<Chatrooms>, String> {        
    auth::chatroom::load_chats(user_id)
}
#[tauri::command]
fn create_chat_room(name: String, user_id: String) -> Result<String, String> {
    // auth::chatroom::load_chats(&user_id)
    let new_name = name; 

    // value comes in as a string, covert this to i32
    let user_id_to_int = user_id.parse::<i32>().unwrap();
    match auth::chatroom::create_chat_room(new_name,user_id_to_int) {
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
#[tauri::command]
fn load_msgs(chat_id: i32) -> Result<Vec<RoomMsgs>, String>{
    auth::chatroom::load_msgs(chat_id)      
}
#[tauri::command]
fn send_msg(chat_id: String, user_id: String, user_msg: String) -> Result<String, String>{
    let user_id_to_int = user_id.parse::<i32>().unwrap();
    let chat_id_to_int = chat_id.parse::<i32>().unwrap();
    let user_msg1 = user_msg;
    match auth::msg::send_msg(chat_id_to_int,user_id_to_int,user_msg1){
        Ok(string) => Ok(string),
        Err(e) => Err(e)
    }
}

#[tauri::command]
fn delete_account(userId: i32) -> Result<String, String>{

    auth::user::delete_account(userId)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login,register, load_chats,create_chat_room, join_chat_room, load_msgs,send_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}