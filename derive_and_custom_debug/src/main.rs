use std::fmt::Debug;

fn main() {
    let user = User {
        name: "clippy".to_string(),
        email: "clippy@rust.org".to_string(),
        password_hash: "hash_password123".to_string(),
    };

    println!("{:?}", user);
}

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    email: String,
    password_hash: String,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let password_hash = "*".repeat(self.password_hash.len());
        f.debug_struct("User")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("password_hash", &password_hash)
            .finish()
    }
}