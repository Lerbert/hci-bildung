use rocket::serde::{Deserialize, Serialize};

use super::logic::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id: user.id,
            username: user.username,
        }
    }
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        UserInfo {
            id: user.id,
            username: user.username.clone(),
        }
    }
}

#[derive(FromForm)]
pub struct LoginForm {
    #[field(validate = neq(""))]
    pub username: String,
    #[field(validate = neq(""))]
    pub password: String,
}
