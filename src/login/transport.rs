use rocket::serde::{Deserialize, Serialize};

use super::logic::{Role, User};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserTransport {
    pub id: i32,
    pub username: String,
}

impl From<User> for UserTransport {
    fn from(user: User) -> Self {
        UserTransport {
            id: user.id,
            username: user.username,
        }
    }
}

impl From<&User> for UserTransport {
    fn from(user: &User) -> Self {
        UserTransport {
            id: user.id,
            username: user.username.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RoleTransport {
    Teacher,
    Student,
}

impl From<Role> for RoleTransport {
    fn from(role: Role) -> Self {
        match role {
            Role::Teacher => Self::Teacher,
            Role::Student => Self::Student,
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
