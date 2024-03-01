// use std::io;``
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::{prelude::Queryable, *};
use std::{env, io};

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    created_at: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn()?;

    let mut new_user = String::new();
    println!("Username \n");

    io::stdin()
        .read_line(&mut new_user)
        .expect("Failed to register username");

    let mut new_user_password = String::new();
    println!("Username \n");

    io::stdin()
        .read_line(&mut new_user_password)
        .expect("Failed to register user");

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

    for user in users {
        println!("{:?}", user);
    }
    Ok(())
    // // let user = String::new();
    // let user = "Adrian";

    // println!("________");

    // let mut msg = String::new();
    // io::stdin()
    //     .read_line(&mut msg)
    //     .expect("Failed to read message");

    // println!("{user}: {msg}");
}
