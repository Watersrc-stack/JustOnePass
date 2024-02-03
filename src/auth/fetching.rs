use sha3::{Digest, Sha3_256};

pub fn fetch_username(prompt: &str) -> (String, bool) {
    let mut line: String = String::new();
    let mut hasher: Sha3_256 = Sha3_256::new();
    let valid: bool;

    println!("{}", prompt);
    (line, valid) = super::super::readline(line);
    Digest::update(&mut hasher, line.as_bytes());
    return (hex::encode(hasher.finalize().as_slice()), valid);
}

pub fn fetch_password(prompt: &str) -> (String, bool) {
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
