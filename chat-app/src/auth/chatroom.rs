// mod msg;
use std::io;
use dotenv::dotenv;
// use mysql::prelude::*;
use mysql::{prelude::Queryable, *};
use std::{env};

#[derive(Debug)]
struct Chatrooms {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct RoomMsgs{
    message: String,
    username: String,
}

pub fn chatrooms(user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn()?;


    println!("Chat rooms \n----------- \n");
    let mut find_a_new_room = String::new();

    println!("Do you want to:
        \n 1. find a new chat room 
        \n 2. create a room
        \n 3. join your a current room? \n Enter 1 or 2 or 3.");

    io::stdin()
        .read_line(&mut find_a_new_room)
        .expect("Error. Input not taken");

    let find_a_new_room = find_a_new_room.trim();

    if find_a_new_room == "1"{
        let mut code = String::new();
        println!("Enter Access code");

        io::stdin()
            .read_line(&mut code)
            .expect("Error, code was not taken");

        let code = code.trim();

        let find_chat_room: Vec<Chatrooms> = conn.exec_map(
            "SELECT id, name FROM chat_rooms WHERE access_code = :code", 
            params! {
                "code" => &code,
            }, |(id, name) | Chatrooms {id,  name},
        
        ).expect("Error finding chat room");

        for room in find_chat_room {
            println!("{:?}", room);
        };
    }else if find_a_new_room == "3"{

        let users_chat_rooms: Vec<Chatrooms> = conn.exec_map(
            "SELECT ctp.chat_room_id, cr.name FROM chat_table_permissions ctp 
            LEFT JOIN chat_rooms cr on cr.id=ctp.chat_room_id WHERE (ctp.user_id = :user_id AND valid_access = TRUE)", 
            params! {
                "user_id" => &user_id,
            }, |(id, name)| Chatrooms { id, name},
    
        ).expect("Failed to find valid chat rooms");
    
        for room in users_chat_rooms {
            println!("{:?}", room);
        }

        let mut group_chat_id = String::new();

        println!("Select a chat room ID");
        io::stdin()
            .read_line(&mut group_chat_id)
            .expect("Error: Enter a valid ID");

        let group_chat_id = group_chat_id.trim();

        let enter_room: Vec<RoomMsgs> = conn.exec_map(
            "SELECT msg.message, u.username FROM messages msg 
            LEFT JOIN users u on msg.user_id=u.id WHERE (msg.group_chat_id = :group_chat_id)", 
            params! {
                "group_chat_id" => &group_chat_id,
            }, |(message,username) | RoomMsgs {message, username}).expect("Could not find chatroom");
    
        for msgs in enter_room {
            println!("{}: {} ", msgs.username, msgs.message);
        };
        super::msg::msg(group_chat_id);
    }
    Ok(())
}
