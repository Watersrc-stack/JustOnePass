pub struct ConnHandler {
    pub valid: bool,
    pub logged: bool,
    pub user: String,
    pub key: String
}

impl ConnHandler {
    pub fn new() -> ConnHandler {
        ConnHandler {
            valid: false,
            logged: false,
            user: String::new(),
            key: String::new()
        }
    }
}
