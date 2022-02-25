use std::fmt::{self, Display};

use chrono::{DateTime, Local};

use crate::Db;

use super::data;

pub type Id = rocket::serde::uuid::Uuid;

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

pub struct Sheet {
    pub metadata: SheetMetadata,
    pub tiptap: serde_json::Value,
}

pub struct SheetMetadata {
    pub id: Id,
    pub title: String,
    pub created: DateTime<Local>,
    pub changed: DateTime<Local>,
}

pub async fn get_all_sheets(db: &Db) -> Result<Vec<SheetMetadata>> {
    Ok(data::get_all_sheets(db).await?)
}

pub async fn create_sheet(db: &Db, title: String) -> Result<Id> {
    let now = chrono::Local::now();
    let tiptap =
        serde_json::from_str("{\"type\": \"doc\", \"content\": [{\"type\": \"paragraph\"}]}")
            .expect("malformed JSON");
    Ok(data::create_sheet(db, title, tiptap, now, now).await?)
}

pub async fn get_sheet_by_id(db: &Db, id: Id) -> Result<Option<Sheet>> {
    Ok(data::get_sheet_by_id(db, id).await?)
}

pub async fn update_sheet(db: &Db, id: Id, title: String, tiptap: serde_json::Value) -> Result<()> {
    Ok(data::update_sheet(db, id, title, tiptap).await?)
}
