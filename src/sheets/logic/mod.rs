use std::fmt::{self, Display};

use super::data;

pub mod sheet;
pub mod solution;

pub type Id = rocket::serde::uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Db(data::Error),
    NotFound(String),
    Forbidden(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Db(e) => write!(f, "Error interacting with database: {}", e),
            Self::NotFound(msg) => write!(f, "Resource not found: {}", msg),
            Self::Forbidden(msg) => write!(f, "Forbidden resource access: {}", msg),
        }
    }
}

impl From<data::Error> for Error {
    fn from(e: data::Error) -> Self {
        Self::Db(e)
    }
}

pub enum DeleteOutcome {
    Deleted,
    Trashed,
}
