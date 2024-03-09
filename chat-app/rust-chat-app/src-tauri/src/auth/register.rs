use std::io;
use dotenv::dotenv;
// use mysql::prelude::*;
use mysql::{prelude::Queryable, *};
use std::{env};

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    created_at: String,
}
pub fn register_user() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn()?;
    
    let mut new_user = String::new();
    println!("Username: ");

    io::stdin()
        .read_line(&mut new_user)
        .expect("Failed to register username");

    let mut new_user_password = String::new();
    println!("Password: ");

    io::stdin()
        .read_line(&mut new_user_password)
        .expect("Failed to register user");

    //TODO I should probably encrypt / decrypt the password
    let new_user = new_user.trim();
    let new_user_password = new_user_password.trim();
        
    conn.exec_drop(
        "INSERT INTO users (username, password) VALUES (:username, :password)",
        params! {
            "username" => new_user,
            "password" => new_user_password,
        },
    )
    .expect("Failed to insert new user");

    let users: Vec<User> = conn.query_map(
        "SELECT id, username, password, created_at FROM users",
        |(id, username, password, created_at)| User {
            id,
            username,
            password,
            created_at,
        },
    )?;

    // for user in users {
    //     // println!("{:?}", user);
    //     super::chatroom::chatrooms(user.id);
    // }
    Ok(())
}