use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Chatrooms {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct RoomMsgs {
    pub message: String,
    pub username: String,
}