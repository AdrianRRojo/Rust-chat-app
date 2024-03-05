mod auth;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut response = String::new();

    println!("Login or register? ");

    io::stdin()
        .read_line(&mut response)
        .expect("Please enter login or register");

    let response = response.trim().to_lowercase();
    
    if response == "login"{
        auth::login::login_user();
    }else if response == "register"{
        auth::register::register_user();
    }else {
        print!("Error, please enter Login or Register {}", response);
    }

    Ok(())
}
