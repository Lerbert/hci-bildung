use rocket::serde::{Deserialize, Serialize};

use super::logic::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserTransport {
    username: String,
}

impl From<User> for UserTransport {
    fn from(user: User) -> Self {
        UserTransport {
            username: user.username,
        }
    }
}

impl From<&User> for UserTransport {
    fn from(user: &User) -> Self {
        UserTransport {
            username: user.username.clone(),
        }
    }
}

#[derive(FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}
