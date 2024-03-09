// use std::io;
use dotenv::dotenv;
// use mysql::prelude::*;
use mysql::{prelude::Queryable, *};
use std::env;

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
}
pub fn login_user(username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn()?;

    let find_user: Vec<User> = conn.exec_map(
        "SELECT id, username, password FROM users WHERE username = :username AND password = :password",
        params! {
            "username" => &username,
            "password" => &password,
        },
        // The closure now correctly matches the structure of the `User` struct
        |(id, username, password)| User { id, username, password },
    ).expect("Failed to find user");
    
    if !find_user.is_empty(){
        for user in find_user {
            println!("{:?}", user);
            // super::chatroom::chatrooms(user.id);
           return Ok(());
        }
        Ok(())
    }else {
        Err("No user found".into())
        // login_user();
    }
}