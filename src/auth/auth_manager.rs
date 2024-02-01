use std::string::String;
use sha3::{Digest, Sha3_256};
use hex;


fn fetch_username() -> String {
    let mut line: String = String::new();
    let mut hasher: Sha3_256 = Sha3_256::new();

    println!("Enter username:");
    match std::io::stdin().read_line(&mut line) {
        Ok(_nice) => {},
        Err(e) => {
            println!("An error as occurred: {}", e);
            line.clear();
            line.push_str("Invalid username\n");
        }
    }
    Digest::update(&mut hasher, line.as_bytes());
    return hex::encode(hasher.finalize().as_slice());
}

pub fn auth_manager() -> bool {
    let username: String = fetch_username();

    if username == "99ca69e39c3a9f719671bfb07c6101c856bceda26d377e4a28f5620564896046" {
        return false;
    }
    println!("{}", username);
    return true;
}
