pub struct ConnHandler {
    pub valid: bool,
    pub user: String,
    pub logged: bool
}

impl ConnHandler {
    pub fn new() -> ConnHandler {
        ConnHandler {
            valid: false,
            user: String::new(),
            logged: false
        }
    }
}
