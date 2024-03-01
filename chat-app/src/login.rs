use std::io;
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
pub fn login_user() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn()?;
    
    let mut user = String::new();
    println!("Username: ");

    io::stdin()
        .read_line(&mut user)
        .expect("Failed to accept username");

    let mut user_password = String::new();
    println!("Password: ");

    io::stdin()
        .read_line(&mut user_password)
        .expect("Failed accept password");

    let user = user.trim();
    let user_password = user_password.trim();
        
    // let find_user: Vec<User> = conn.exec_drop(
    //     "SELECT * FROM users WHERE username = :username AND password = :password",
    //     params! {
    //         "username" => user,
    //         "password" => user_password,
    //     },
    // )
    // .expect("Failed to find user");

    let find_user: Vec<User> = conn.exec_map(
        "SELECT id, username, password FROM users WHERE username = :username AND password = :password",
        params! {
            "username" => &user,
            "password" => &user_password,
        },
        // The closure now correctly matches the structure of the `User` struct
        |(id, username, password)| User { id, username, password },
    ).expect("Failed to find user");
    
    for user in find_user {
        println!("{:?}", user);
    }
    Ok(())
}