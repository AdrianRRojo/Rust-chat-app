mod register;
mod login;
use std::io;
// use dotenv::dotenv;
// use mysql::prelude::*;
// use mysql::{prelude::Queryable, *};
// use std::{env};

// #[derive(Debug)]
// struct User {
//     id: i32,
//     username: String,
//     password: String
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dotenv().ok();
    // let db_url = env::var("DB_URL").expect("Failed to find url");
    // let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    // let pool = Pool::new(opts).expect("Failed to create pool");

    // let mut conn = pool.get_conn()?;

    let mut response = String::new();

    println!("Login or register? ");

    io::stdin()
        .read_line(&mut response)
        .expect("Please enter login or register");

    let response = response.trim().to_lowercase();
    
    if response == "login"{
        login::login_user();
    }else if response == "register"{
        println!("Register Now");
        register::register_user();
    }else {
        print!("Error, please enter Login or Register {}", response);
    }

    Ok(())
}
