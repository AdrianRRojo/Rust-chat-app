use std::io;
fn main() {
    // let user = String::new();
    let user = "Adrian";

    println!("________");

    let mut msg = String::new();
    io::stdin()
        .read_line(&mut msg)
        .expect("Failed to read message");

    println!("{user}: {msg}");
}
