use crate::structures::ConnHandler;
use super::fetching;

pub fn create_account() -> ConnHandler {
    let (username, user_valid) = fetching::fetch_username("Enter username:");
    let (password, pass_valid) = fetching::fetch_password("Enter password:");
    let (password2, pass_valid2) = fetching::fetch_password("Confirm password");
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
    if password.len() < 8 {
        println!("Password should be at least 8 chars.");
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
            println!("Connected");
            conn.valid = true;
            conn.logged = true;
            conn.user = username;
            conn.key = password;
            conn
        }
        Err(e) => {
            println!("Error: \n{}", e);
            conn
        }
    }
}
