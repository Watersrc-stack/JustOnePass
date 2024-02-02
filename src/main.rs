use std::io::Write;
use crate::auth::{create_account, login};

mod auth;

fn readline(mut line: String) -> (String, bool) {
    let valid: bool;
    line.clear();

    match std::io::stdin().read_line(&mut line) {
        Ok(_nice) => {valid = true},
        Err(e) => { println!("An error occurred: {}", e); valid = false }
    }
    return (line, valid);
}

fn main() {
    let mut input: String = String::new();
    let mut valid: bool;

    loop {
        print!("--> ");
        std::io::stdout().flush().unwrap();
        (input, valid) = readline(input);
        if !valid || input == "exit\n" {break}
        else if input == "create\n" {valid = create_account()}
        else if input == "login\n" {valid = login()}
        else {println!("Command not found")};
        if !valid {break};
    }
    return ;
}
