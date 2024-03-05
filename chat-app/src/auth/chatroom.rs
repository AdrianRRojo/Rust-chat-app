// mod msg;
use std::io;
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::{prelude::Queryable, *};
use std::{env};

#[derive(Debug)]
struct Chatrooms {
    id: i32,
    name: String,
}


pub fn chatrooms(user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn()?;

    let mut response = String::new();

    println!("Chat rooms \n----------- \n");
    
    let find_chat_rooms: Vec<Chatrooms> = conn.exec_map(
        "SELECT ctp.chat_room_id, cr.name FROM chat_table_permissions ctp 
        LEFT JOIN chat_rooms cr on cr.id=ctp.chat_room_id WHERE (ctp.user_id = :user_id AND valid_access = TRUE)", 
        params! {
            "user_id" => &user_id,
        }, |(id, name)| Chatrooms { id, name},
    
    ).expect("Failed to find valid chat rooms");
    
    for room in find_chat_rooms {
        println!("{:?}", room);
    }

    Ok(())
}
