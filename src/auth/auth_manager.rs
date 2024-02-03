use std::string::String;
use sha3::{Digest, Sha3_256};
use rpassword;
use hex;
use super::super::structures::*;

fn fetch_username(prompt: &str) -> (String, bool) {
    let mut line: String = String::new();
    let mut hasher: Sha3_256 = Sha3_256::new();
    let valid: bool;

    println!("{}", prompt);
    (line, valid) = super::super::readline(line);
    Digest::update(&mut hasher, line.as_bytes());
    return (hex::encode(hasher.finalize().as_slice()), valid);
}

fn fetch_password(prompt: &str) -> (String, bool) {
    let line: String;
    let mut hasher: Sha3_256 = Sha3_256::new();
    let mut valid: bool = true;

    println!("{}", prompt);
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
    let (username, user_valid) = fetch_username("Enter username:");
    let (password, pass_valid) = fetch_password("Enter password:");

    if user_valid == false || pass_valid == false {
        println!("Read error");
        if user_valid == false {
            println!("username not valid");
        } else if pass_valid == false {
            println!("password not valid");
        } else {
            println!("Wat da hail");
        }
        return false;
    }
    println!("{}", username);
    println!("{}", password);
    return true;
}

pub fn create_account() -> ConnHandler {
    let (username, user_valid) = fetch_username("Enter username:");
    let (password, pass_valid) = fetch_password("Enter password:");
    let (password2, pass_valid2) = fetch_password("Confirm password");
    let mut conn: ConnHandler = ConnHandler::new();
    let mut name: String;

    println!("Returns check");
    if user_valid == false || pass_valid == false || pass_valid2 == false {
        println!("Read error");
        if user_valid == false {
            println!("username not valid");
        } else if pass_valid == false || pass_valid2 == false {
            println!("password not valid");
        } else {
            println!("Wat da hail");
        }
        return conn;
    }
    if password != password2 {
        println!("Passwords do not match.");
        return conn;
    }
    name = format!("./users/{}", username);
    println!("directory creation\n{}", name.clone());
    match std::fs::create_dir(name.clone()) {
        Ok(_nice) => {
            conn.valid = true;
        }
        Err(e) => {
            println!("Error: \n{}", e);
            return conn
        }
    }
    name = format!("./users/{}/{}{}", username, username, password);
    println!("Final step");
    return match std::fs::File::create(name) {
        Ok(_nice) => {
            conn.valid = true;
            conn.logged = true;
            conn.user = username;
            conn
        }
        Err(e) => {
            println!("Error: \n{}", e);
            conn
        }
    }
}
