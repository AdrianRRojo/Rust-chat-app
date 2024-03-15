// use std::io;
use dotenv::dotenv;
// use mysql::prelude::*;
use crate::auth::models::User;
use mysql::{prelude::Queryable, *};
use std::env;

pub fn register_user(username: &str, password: &str, email: &str) -> Result<Option<User>, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error");;

    // let mut new_user = String::new();
    // println!("Username: ");

    // io::stdin()
    //     .read_line(&mut new_user)
    //     .expect("Failed to register username");

    // let mut new_user_password = String::new();
    // println!("Password: ");

    // io::stdin()
    //     .read_line(&mut new_user_password)
    //     .expect("Failed to register user");

    // //TODO I should probably encrypt / decrypt the password
    // let new_user = new_user.trim();
    // let new_user_password = new_user_password.trim();

    conn.exec_drop(
        "INSERT INTO users (username, password, email) VALUES (:username, :password, :email)",
        params! {
            "username" => username,
            "password" => password,
            "email" => email
        },
    )
    .expect("Failed to insert new user");
    let find_user: Vec<User> = conn.exec_map(
    "SELECT id, username, password FROM users WHERE username = :username AND password = :password",
    params! {
        "username" => &username,
        "password" => &password,
    },
    // The closure now correctly matches the structure of the `User` struct
    |(id, username, password)| User { id, username, password },
).expect("Failed to find user");

    if find_user.is_empty() {
        Err("No user found".to_string())
    } else {
        Ok(find_user.into_iter().next()) // Return the first user found
    }
}
