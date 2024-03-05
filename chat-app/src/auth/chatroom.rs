// mod msg;
use std::io;
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::{prelude::Queryable, *};
use std::{env};


pub fn chatrooms() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn()?;

    let mut response = String::new();

    println!("Chat rooms \n----------- \n");
    

    

    Ok(())
}
