use rocket::outcome::{try_outcome, IntoOutcome};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

use crate::Db;

use super::logic::{self, Role, User};
use super::transport::UserInfo;

pub const SESSION_ID_COOKIE_NAME: &str = "session_id";

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticatedUser {
    pub user_info: UserInfo,
    pub roles: Vec<Role>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r AuthenticatedUser {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request
            .local_cache_async(async {
                let db = request.guard::<Db>().await.succeeded()?;
                if let Some(session_id) = request
                    .cookies()
                    .get_private(SESSION_ID_COOKIE_NAME)
                    .map(|cookie| cookie.value().to_owned())
                {
                    logic::validate_session(&db, session_id)
                        .await
                        .map_err(|e| error!("{}", e))
                        .ok()
                        .flatten()
                        .map(|user| user.into())
                } else {
                    None
                }
            })
            .await;
        user.as_ref().or_forward(())
    }
}

impl From<User> for AuthenticatedUser {
    fn from(user: User) -> Self {
        AuthenticatedUser {
            user_info: UserInfo {
                id: user.id,
                username: user.username,
            },
            roles: user.roles,
        }
    }
}

impl From<&User> for AuthenticatedUser {
    fn from(user: &User) -> Self {
        AuthenticatedUser {
            user_info: user.into(),
            roles: user.roles.clone(),
        }
    }
}

pub struct Teacher<'a>(&'a AuthenticatedUser);

impl<'a> Teacher<'a> {
    pub fn into_inner(self) -> &'a AuthenticatedUser {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Teacher<'r> {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<&'r AuthenticatedUser>().await);
        if user.roles.contains(&Role::Teacher) {
            Outcome::Success(Teacher(user))
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct Student<'a>(&'a AuthenticatedUser);

impl<'a> Student<'a> {
    pub fn into_inner(self) -> &'a AuthenticatedUser {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Student<'r> {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<&'r AuthenticatedUser>().await);
        if user.roles.contains(&Role::Student) {
            Outcome::Success(Student(user))
        } else {
            Outcome::Forward(())
        }
    }
}
