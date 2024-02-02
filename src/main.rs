mod auth;

fn main() {
    match auth::login(){
        true => println!("true"),
        false => println!("false")
    }
    return ;
}
