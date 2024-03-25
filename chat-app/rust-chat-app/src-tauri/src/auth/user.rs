
use dotenv::dotenv;
use crate::auth::models::Chatrooms;
use crate::auth::models::RoomMsgs;
use mysql::{prelude::Queryable, *};
use rand::seq::SliceRandom;
use std::env;

pub fn delete_account(user_id: i32) -> Result<String, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error");

    let delete_account: String = conn.exec_drop(
        "DELETE FROM users WHERE id = :user_id",
        params!{
            "user_id" => &user_id,
        }
    ).expect("Could not delete user".to_string());

    Ok("Delete Successful!".to_string())
}

pub fn load_msgs(chat_id: i32) -> Result<Vec<RoomMsgs>, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error");

    let load_chat_msgs: Vec<RoomMsgs> = conn
        .exec_map(
            "SELECT msg.message, u.username FROM messages msg
                    LEFT JOIN users u on msg.user_id=u.id WHERE (msg.group_chat_id = :chat_id)",
            params! {
                "chat_id" => &chat_id,
            },
            |(message, username)| RoomMsgs { message, username },
        )
        .expect("Could not find chatroom");

    if load_chat_msgs.is_empty() {
        Err("No messages found".to_string())
    } else {
        Ok(load_chat_msgs) // Return the first user found
    }
}

pub fn join_chat_room(user_id: i32, access_code: String) -> Result<Option<Chatrooms>, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error connectiong to DB");

    let find_chat_room: Vec<Chatrooms> = conn
        .exec_map(
            "SELECT id, name FROM chat_rooms WHERE access_code = :access_code",
            params! {
                "access_code" => &access_code,
            },
            |(id, name)| Chatrooms { id, name },
        )
        .expect("Error finding chat room");

    // if let Some(chat_room) = find_chat_room.get(0) {
    //     let chat_room_id = chat_room.id;
    println!("{}", user_id);
    //     Ok(find_chat_room.to_string())
    //     // conn.exec_drop(
    //     //             "INSERT INTO chat_table_permissions (chat_room_id, user_id) VALUES (:chat_room_id,:user_id)",
    //     //             params! {
    //     //                 "chat_room_id" => &chat_room_id,
    //     //                 "user_id" => &user_id
    //     //             },
    //     //         ).expect("Failed to join room");
    //     // Ok(true)
    // } else {
    //     Err("No chats found".to_string())
    // }

    if let Some(chat_room) = find_chat_room.get(0) {
        let chat_room_id = chat_room.id;
        conn.exec_drop(
                            "INSERT INTO chat_table_permissions (chat_room_id, user_id) VALUES (:chat_room_id,:user_id)",
                            params! {
                                "chat_room_id" => &chat_room_id,
                                "user_id" => &user_id
                            },
                        ).expect("Failed to join room");
        Ok(find_chat_room.into_iter().next())
    } else {
        Err("No user found".to_string())
    }
}
pub fn create_chat_room(name: String, user_id: i32) -> Result<String, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error");

    let letters = vec![
        'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
        'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r',
        'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
    ];
    let numbs = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let letter_len = 5;
    let num_len = 3;

    let full_code_length = letter_len + num_len;

    let mut letter_output: Vec<_> = letters
        .choose_multiple(&mut rand::thread_rng(), letter_len.try_into().unwrap())
        .collect();

    let mut numbs_output: Vec<_> = numbs
        .choose_multiple(&mut rand::thread_rng(), num_len.try_into().unwrap())
        .collect();

    let mut code_vec = vec![];

    //combine the responses into 1 vec
    code_vec.append(&mut letter_output);
    code_vec.append(&mut numbs_output);

    code_vec.shuffle(&mut rand::thread_rng());

    let code: String = code_vec
        .into_iter()
        .take(full_code_length as usize)
        .collect();

    // println!("{}",user_id);
    conn.exec_drop(
        "INSERT INTO chat_rooms (access_code,name) VALUES (:code,:name)",
        params! {
            "name" => name.clone(),
            "code" => code.clone()
        },
    )
    .expect("Failed to create room");
    // println!("Success!");
    join_chat_room(user_id, code.clone());
    Ok(code)
}

