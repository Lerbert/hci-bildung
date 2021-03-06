use std::fmt::{self, Display};

use chrono::NaiveDateTime;
use rand::rngs::OsRng;
use rand::RngCore;
use rocket::serde::{Deserialize, Serialize};

use crate::crypt;
use crate::Db;

use super::data;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Db(data::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Db(e) => write!(f, "Error interacting with database: {}", e),
        }
    }
}

impl From<data::Error> for Error {
    fn from(e: data::Error) -> Self {
        Self::Db(e)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Role {
    Teacher,
    Student,
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub session: Option<Session>,
    pub roles: Vec<Role>,
}

impl User {
    fn check_password(&self, password: &str) -> bool {
        crypt::check_password(password, &self.password_hash)
    }
}

#[derive(Debug)]
pub struct Session {
    pub id: String,
    pub expires: NaiveDateTime,
}

impl Session {
    fn is_valid(&self) -> bool {
        self.expires > chrono::Utc::now().naive_local()
    }
}

const EXPIRY_DAYS: i64 = 5;

pub async fn login(db: &Db, username: String, password: String) -> Result<Option<String>> {
    if let Some(user) = data::get_user_by_name(db, username).await? {
        login_user(db, &user, &password).await
    } else {
        Ok(None)
    }
}

pub async fn validate_session(db: &Db, session_id: String) -> Result<Option<User>> {
    if let Some(user) = data::get_user_by_session_id(db, session_id).await? {
        let session = user
            .session
            .as_ref()
            .expect("user found by session_id should have session");
        if session.is_valid() {
            Ok(Some(user))
        } else {
            data::delete_session(db, user.id).await?;
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub async fn logout(db: &Db, user_id: i32) -> Result<()> {
    Ok(data::delete_session(db, user_id).await?)
}

async fn login_user(db: &Db, user: &User, provided_password: &str) -> Result<Option<String>> {
    if user.check_password(provided_password) {
        let session_id = if let Some(session) = &user.session {
            renew_session(db, &session.id).await?;
            session.id.clone()
        } else {
            create_session(db, user.id).await?
        };
        Ok(Some(session_id))
    } else {
        Ok(None)
    }
}

async fn create_session(db: &Db, user_id: i32) -> Result<String> {
    let session_id = generate_session_id();
    let expires = chrono::Utc::now().naive_local() + chrono::Duration::days(EXPIRY_DAYS);
    Ok(data::create_session(db, user_id, session_id, expires).await?)
}

async fn renew_session(db: &Db, session_id: &str) -> Result<()> {
    let expires = chrono::Utc::now().naive_local() + chrono::Duration::days(EXPIRY_DAYS);
    Ok(data::renew_session(db, session_id.to_owned(), expires).await?)
}

fn generate_session_id() -> String {
    let mut rng = OsRng {};
    let mut bytes: [u8; 96] = [0; 96];
    rng.fill_bytes(&mut bytes);
    base64::encode(bytes)
}
