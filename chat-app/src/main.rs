// use std::io;``
// use mysql::*;
// use mysql::prelude::*;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find url");

    print!("URL: {db_url}")
    // // let user = String::new();
    // let user = "Adrian";

    // println!("________");

    // let mut msg = String::new();
    // io::stdin()
    //     .read_line(&mut msg)
    //     .expect("Failed to read message");

    // println!("{user}: {msg}");
}
