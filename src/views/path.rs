pub struct Path {
    pub prefix: String,
}

impl Path {
    pub fn define(&self, following_path: String) -> String {
        // format!("{}{}", self.prefix, following_path)
        self.prefix.to_owned() + &following_path
    }
}
