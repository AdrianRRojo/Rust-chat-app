// use std::io;
use dotenv::dotenv;
// use mysql::prelude::*;
use mysql::{prelude::Queryable, *};
use std::{env};

pub fn send_msg(chat_id: i32, user_id: i32, user_msg: String) -> Result<String, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error connecting to pool");

    // let mut new_message = String::new();
    // println!(": ");

    // io::stdin()
    //     .read_line(&mut new_message)
    //     .expect("Failed to send");

    // let new_message = new_message.trim();

    conn.exec_drop(
        "INSERT INTO messages (user_id, message, group_chat_id) VALUES (:user_id, :user_msg, :chat_id)",
        params! {
            "user_id" => user_id,
            "user_msg" => user_msg.clone(),
            "chat_id" => chat_id
        },
    ).expect("Failed to add message");

    Ok(user_msg)
}
