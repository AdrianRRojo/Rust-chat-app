// use core::num;
// mod msg;
use dotenv::dotenv;
// use std::io;
// use mysql::prelude::*;
use crate::auth::models::Chatrooms;
use mysql::{prelude::Queryable, *};
use rand::seq::SliceRandom;
use std::env;

// #[derive(Debug)]
// struct Chatrooms {
//     id: i32,
//     name: String,
// }

#[derive(Debug)]
struct RoomMsgs {
    message: String,
    username: String,
}
#[derive(Debug)]
struct ChatroomAccessCode {
    access_code: String,
}

pub fn load_chats(user_id: i32) -> Result<Vec<Chatrooms>, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error");

    let users_chat_rooms: Vec<Chatrooms> = conn
        .exec_map(
            "SELECT ctp.chat_room_id, cr.name FROM chat_table_permissions ctp
        LEFT JOIN chat_rooms cr ON cr.id = ctp.chat_room_id WHERE ctp.user_id = :user_id",
            params! {
                "user_id" => user_id,
            },
            |(chat_room_id, name)| Chatrooms {
                id: chat_room_id,
                name,
            },
        )
        .expect("Failed to find chat rooms");
    if users_chat_rooms.is_empty() {
        Err("No user found".to_string())
    } else {
        Ok(users_chat_rooms) // Return the first user found
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
pub fn create_chat_room(name: String) -> Result<String, String> {
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
    // join_chats(1, code.clone());
    Ok(code)
}

// pub fn chatrooms(user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().ok();
//     let db_url = env::var("DB_URL").expect("Failed to find DB url");
//     let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
//     let pool = Pool::new(opts).expect("Failed to create pool");

//     let mut conn = pool.get_conn()?;

//     println!("Chat rooms \n----------- \n");
//     let mut find_a_new_room = String::new();

//     println!("Do you want to:
//         \n 1. find a new chat room
//         \n 2. create a room
//         \n 3. join your a current room? \n Enter 1 or 2 or 3.");

//     io::stdin()
//         .read_line(&mut find_a_new_room)
//         .expect("Error. Input not taken");

//     let find_a_new_room = find_a_new_room.trim();

//     if find_a_new_room == "1"{
//         let mut code = String::new();
//         println!("Enter Access code");

//         io::stdin()
//             .read_line(&mut code)
//             .expect("Error, code was not taken");

//         let code = code.trim();

//         let find_chat_room: Vec<Chatrooms> = conn.exec_map(
//             "SELECT id, name FROM chat_rooms WHERE access_code = :code",
//             params! {
//                 "code" => &code,
//             }, |(id, name) | Chatrooms {id,  name},

//         ).expect("Error finding chat room");

//         for room in find_chat_room {
//             println!("{:?}", room);
//         };
//     }else if find_a_new_room == "2"{
//         let letters = vec!['A','a','B','b','C','c','D','d','E','e','F','f','G','g','H','h','I','i','J','j','K','k','L','l','M','m','N','n','O','o','P','p','Q','q','R','r','S','s','T','t','U','u','V','v','W','w','X','x','Y','y','Z','z'];
//         let numbs = vec!['0','1','2','3','4','5','6','7','8','9'];

//         let letter_len = 5;
//         let num_len = 3;

//         let full_password_length = letter_len + num_len;

//         let mut letter_output: Vec<_> = letters
//         .choose_multiple(&mut rand::thread_rng(), letter_len.try_into().unwrap())
//         .collect();

//         let mut numbs_output: Vec<_> = numbs
//         .choose_multiple(&mut rand::thread_rng(), num_len.try_into().unwrap())
//         .collect();

//         let mut code_vec = vec![];

//         //combine the responses into 1 vec
//         pass_vec.append(&mut letter_output);
//         pass_vec.append(&mut numbs_output);

//         pass_vec.shuffle(&mut rand::thread_rng());

//         let password: String = pass_vec.into_iter().take(full_password_length as usize).collect();

//         println!("{}", password);

//         let mut new_name = String::new();
//         println!("Chat room name: ");

//         io::stdin()
//             .read_line(&mut new_name)
//             .expect("Error reading name");

//         conn.exec_drop(
//             "INSERT INTO chat_rooms (access_code, name) VALUES (:password, :new_name)",
//             params! {
//                 "new_name" => new_name,
//                 "password" => password
//             },
//         ).expect("Failed to create room");
//         println!("Success!");
//     }else if find_a_new_room == "3"{

//         let users_chat_rooms: Vec<Chatrooms> = conn.exec_map(
//             "SELECT ctp.chat_room_id, cr.name FROM chat_table_permissions ctp
//             LEFT JOIN chat_rooms cr on cr.id=ctp.chat_room_id WHERE (ctp.user_id = :user_id AND valid_access = TRUE)",
//             params! {
//                 "user_id" => &user_id,
//             }, |(id, name)| Chatrooms { id, name},

//         ).expect("Failed to find valid chat rooms");

//         for room in users_chat_rooms {
//             println!("{:?}", room);
//         }

//         let mut group_chat_id = String::new();

//         println!("Select a chat room ID");
//         io::stdin()
//             .read_line(&mut group_chat_id)
//             .expect("Error: Enter a valid ID");

//         let group_chat_id = group_chat_id.trim();

//         let enter_room: Vec<RoomMsgs> = conn.exec_map(
//             "SELECT msg.message, u.username FROM messages msg
//             LEFT JOIN users u on msg.user_id=u.id WHERE (msg.group_chat_id = :group_chat_id)",
//             params! {
//                 "group_chat_id" => &group_chat_id,
//             }, |(message,username) | RoomMsgs {message, username}).expect("Could not find chatroom");

//         for msgs in enter_room {
//             println!("{}: {} ", msgs.username, msgs.message);
//         };

//         let mut chat_room_choice = String::new();
//         println!("Do you want to  \n 1. Enter a message \n 2. Get the access code?");

//         io::stdin()
//             .read_line(&mut chat_room_choice)
//             .expect("Error reading choice");

//         let chat_room_choice = chat_room_choice.trim();

//         if chat_room_choice == "1"{
//             let get_access_code: Vec<ChatroomAccessCode> = conn.exec_map(
//                 "SELECT access_code FROM chat_rooms WHERE id = :group_chat_id",
//                 params! {
//                     "group_chat_id" => group_chat_id,
//                 },
//                 |(access_code)| ChatroomAccessCode { access_code },
//             ).expect("Error finding group chat");

//             for access_code in get_access_code {
//                 println!("Access code: {:?}", access_code.access_code);
//             }
//         }else{
//             super::msg::msg(group_chat_id);
//         }
//     }
//     Ok(())
// }
