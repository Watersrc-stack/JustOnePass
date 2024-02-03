use crate::structures::ConnHandler;
use super::fetching;

pub fn login() -> ConnHandler {
    let (username, user_valid) = fetching::fetch_username("Enter username:");
    let (password, pass_valid) = fetching::fetch_password("Enter password:");
    let mut conn: ConnHandler = ConnHandler::new();
    let name: String;

    if user_valid == false || pass_valid == false {
        println!("Read error");
        if user_valid == false {
            println!("username not valid");
        } else if pass_valid == false {
            println!("password not valid");
        } else {
            println!("Wat da hail");
        }
        return conn;
    }
    name = format!("./users/{}/{}{}", username, username, password);
    if !std::path::Path::new(& name).exists(){
        println!("Invalid credentials");
        return conn;
    }
    conn = ConnHandler {valid: true, logged: true, user: username, key: password};
    println!("Connected");
    return conn;
}
