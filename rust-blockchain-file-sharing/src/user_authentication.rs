// user_authentication.rs

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

pub fn authenticate_user(username: &str, password: &str) -> bool {
    // Simulate user data retrieval from a database or other storage
    let users: Vec<User> = vec![
        User {
            username: "user1".to_string(),
            password: "password1".to_string(),
        },
        User {
            username: "user2".to_string(),
            password: "password2".to_string(),
        },
    ];

    // Check if there's a user with the provided username and password
    users.iter().any(|user| user.username == username && user.password == password)
}
