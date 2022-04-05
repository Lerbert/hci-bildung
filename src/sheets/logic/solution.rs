use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

use crate::login::transport::UserTransport;
use crate::Db;

use super::{data, Error, Id, Result};

#[derive(Debug, Serialize)]
pub struct Solution {
    pub metadata: SolutionMetadata,
    pub solution: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SolutionMetadata {
    pub id: i32,
    pub title: String,
    pub sheet_id: Option<Id>,
    pub sheet_version: DateTime<Utc>,
    pub owner: UserTransport,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub trashed: Option<DateTime<Utc>>,
}
