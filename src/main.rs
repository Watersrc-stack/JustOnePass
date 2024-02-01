mod auth;

fn main() {
    match auth::auth_manager(){
        true => println!("true"),
        false => println!("false")
    }
    return ;
}
