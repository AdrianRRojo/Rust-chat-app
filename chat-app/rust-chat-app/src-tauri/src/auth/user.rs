use crate::auth::models::User;
use dotenv::dotenv;
use mysql::{prelude::Queryable, *};
// use rand::seq::SliceRandom;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;

// pub fn update_username(user_id: i32, new_username: String) -> Result<String, String>{
//     dotenv().ok();
//     let db_url = env::var("DB_URL").expect("Failed to find DB url");
//     let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
//     let pool = Pool::new(opts).expect("Failed to create pool");

//     let mut conn = pool.get_conn().expect("Error");

//     conn.exec_drop(
//         "UPDATE users SET username = :new_username WHERE id = :user_id",
//         params! {
//             "username" => new_username,
//             "user_id" => user_id,
//         },
//     ).map_err(|e| e.to_string())?;
    
//     Ok("Uodate Successful!".to_string())
// }

pub fn delete_account(user_id: i32) -> Result<String, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error");

    conn.exec_drop(
        "DELETE FROM users WHERE id = :user_id",
        params! {
            "user_id" => user_id,
        },
    )
    .map_err(|e| e.to_string())?;

    Ok("Delete Successful!".to_string())
}

fn hash_password(password: String) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}
fn verify_password(password: String, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed)
}

pub fn update_password(
    user_id: i32,
    curr_password: String,
    new_password: String,
) -> Result<String, String> {
    dotenv().ok();
    let db_url = env::var("DB_URL").expect("Failed to find DB url");
    let opts = Opts::from_url(&db_url).expect("Invalid DB Url");
    let pool = Pool::new(opts).expect("Failed to create pool");

    let mut conn = pool.get_conn().expect("Error");

    let find_user: Vec<User> = conn
        .exec_map(
            "SELECT id, username, password FROM users WHERE id = :user_id LIMIT 1 ",
            params! {
                "user_id" => &user_id,
            },
            |(id, username, password)| User {
                id,
                username,
                password,
            },
        )
        .expect("Failed to find user");

    if let Some(user) = find_user.into_iter().next() {
        match verify_password(curr_password, &user.password) {
            Ok(matches) => {
                if matches {
                    let hashed_password = hash_password(new_password).map_err(|e| e.to_string())?;
                    conn.exec_drop(
                        "UPDATE users SET password = :hashed_password WHERE id = :user_id",
                        params! {
                            "hashed_password" => hashed_password,
                            "user_id" => user_id,
                        },
                    )
                    .map_err(|e| e.to_string())?;

                    Ok("Update Successful!".to_string())
                } else {
                    Err("Invalid username or password".to_string())
                }
            }
            Err(_) => Err("Failed to update password".to_string()),
        }
    } else {
        Err("No user found".to_string())
    }

    // let hashed_password = hash_password(new_password).map_err(|e| e.to_string())?;
    // conn.exec_drop(
    //     "UPDATE users SET password = :hashed_password WHERE id = :user_id",
    //     params! {
    //         "hashed_password" => hashed_password,
    //         "user_id" => user_id,
    //     }
    // ).map_err(|e| e.to_string())?;

    // Ok("Update Successful!".to_string())
}
