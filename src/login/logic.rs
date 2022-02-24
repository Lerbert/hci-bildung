use std::convert::From;
use std::fmt::{self, Display};
use std::io;

use chrono::NaiveDateTime;
use crypto::scrypt;
use log::error;
use rand::rngs::OsRng;
use rand::RngCore;

use crate::Db;

use super::data;
use super::transport::LoginForm;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DbError(data::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbError(e) => write!(f, "Error interacting with database: {}", e),
        }
    }
}

impl From<data::Error> for Error {
    fn from(e: data::Error) -> Self {
        Self::DbError(e)
    }
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub session: Option<Session>,
}

impl User {
    fn check_password(&self, password: &str) -> bool {
        check_password(password, &self.password_hash)
    }
}

#[derive(Debug)]
pub struct Session {
    pub id: String,
    pub expires: NaiveDateTime,
}

const EXPIRY_DAYS: i64 = 5;

pub async fn login(db: &Db, login_data: LoginForm) -> Result<Option<String>> {
    if let Some(user) = data::get_user_by_name(db, login_data.username).await? {
        login_user(db, &user, &login_data.password).await
    } else {
        Ok(None)
    }
}

pub async fn get_session_user(db: &Db, session_id: String) -> Result<Option<User>> {
    Ok(data::get_user_by_session_id(db, session_id).await?)
}

pub async fn logout(db: &Db, user: &User) -> Result<()> {
    Ok(data::delete_session(db, user.id).await?)
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

fn _hash_password(password: &str) -> io::Result<String> {
    scrypt::scrypt_simple(password, &scrypt::ScryptParams::new(14, 16, 1))
}

fn check_password(password: &str, hash: &str) -> bool {
    scrypt::scrypt_check(password, hash).unwrap_or_else(|e| {
        error!("Error checking password: {}", e);
        false
    })
}
