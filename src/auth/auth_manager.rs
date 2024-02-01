use std::string::String;
use sha3::{Digest, Sha3_256};
use rpassword;
use hex;


fn fetch_and_hash(prompt: &str) -> (String, bool) {
    let mut line: String = String::new();
    let mut hasher: Sha3_256 = Sha3_256::new();
    let mut valid: bool = true;

    println!("{}", prompt);
    match std::io::stdin().read_line(&mut line) {
        Ok(_nice) => {},
        Err(e) => {
            println!("An error as occurred: {}", e);
            valid = false;
        }
    }
    Digest::update(&mut hasher, line.as_bytes());
    return (hex::encode(hasher.finalize().as_slice()), valid);
}

pub fn auth_manager() -> bool {
    let (username, user_valid) = fetch_and_hash("Enter username:");
    let (password, pass_valid) = fetch_and_hash("Enter password:");

    if user_valid == false || pass_valid == false {
        println!("Read error");
        if user_valid == false {
            println!("username not valid");
        } else if pass_valid == false {
            println!("password not valid");
        } else {
            println!("Wat da hail");
        }
        return false
    }
    println!("{}", username);
    println!("{}", password);
    return true;
}
