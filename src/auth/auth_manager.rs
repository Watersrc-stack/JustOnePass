use std::string::String;
use sha3::{Digest, Sha3_256};
use rpassword;
use hex;

fn fetch_username() -> (String, bool) {
    let mut line: String = String::new();
    let mut hasher: Sha3_256 = Sha3_256::new();
    let mut valid: bool = true;

    println!("Enter username:");
    match std::io::stdin().read_line(&mut line) {
        Ok(_nice) => {},
        Err(e) => {
            println!("An error as occurred: {}", e);
            valid = false;
        }
    };
    Digest::update(&mut hasher, line.as_bytes());
    return (hex::encode(hasher.finalize().as_slice()), valid);
}

fn fetch_password() -> (String, bool) {
    let line: String;
    let mut hasher: Sha3_256 = Sha3_256::new();
    let mut valid: bool = true;

    println!("Enter password:");
    line = match rpassword::read_password() {
        Ok(str) => str,
        Err(e) => {
            println!("An error as occurred: {}", e);
            valid = false;
            String::from("not valid")
        }
    };
    Digest::update(&mut hasher, line.as_bytes());
    return (hex::encode(hasher.finalize().as_slice()), valid);
}

pub fn login() -> bool {
    let (username, user_valid) = fetch_username();
    let (password, pass_valid) = fetch_password();

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
