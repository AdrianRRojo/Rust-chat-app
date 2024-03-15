
use dotenv::dotenv;
use crate::auth::models::User;
use mysql::{prelude::Queryable, *};
use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}
fn verify_password(password: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed)
}
pub fn register_user(username: &str, password: &str, email: &str) -> Result<Option<User>, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error");

    //TODO I should probably encrypt / decrypt the password
    let hashed_password = hash_password(password).map_err(|e| e.to_string())?;

    conn.exec_drop(
        "INSERT INTO users (username, password, email) VALUES (:username, :password, :email)",
        params! {
            "username" => username,
            "password" => &hashed_password,
            "email" => email
        },
    )
    .expect("Failed to insert new user");

    let find_user: Vec<User> = conn.exec_map(
        "SELECT id, username, password FROM users WHERE username = :username LIMIT 1 ",
        params! {
            "username" => &username,
        },
        |(id, username, password)| User { id, username, password },
    ).expect("Failed to find user");

    if let Some(user) = find_user.into_iter().next() {
        match verify_password(&password, &user.password) {
            Ok(matches) => {
                if matches {
                    Ok(Some(user)) // Passwords match, proceed with login
                } else {
                    Err("Invalid username or password".to_string())
                }
            },
            Err(_) => Err("Failed to verify password".to_string()),
        }
    } else {
        Err("No user found".to_string())
    }
}
