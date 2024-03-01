// use std::io;``
use mysql::{prelude::Queryable, *};
// use mysql::prelude::*;
use std::env;
use dotenv::dotenv;
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

    let users: Vec<User> = conn.query_map(
        "SELECT id, username, password, created_at FROM users",
        |(id, username, password, created_at)| User { id, username, password, created_at },
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
