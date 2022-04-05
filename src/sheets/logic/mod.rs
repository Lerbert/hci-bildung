use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

use crate::login::transport::UserTransport;
use crate::Db;

use super::data;

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

#[derive(Debug, Serialize)]
pub struct Sheet {
    pub metadata: SheetMetadata,
    pub tiptap: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SheetMetadata {
    pub id: Id,
    pub title: String,
    pub owner: UserTransport,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub trashed: Option<DateTime<Utc>>,
}

pub enum DeleteOutcome {
    Deleted,
    Trashed,
}

pub async fn get_all_sheets(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>> {
    Ok(data::get_all_sheets(db, user_id).await?)
}

pub async fn get_trash(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>> {
    Ok(data::get_trash(db, user_id).await?)
}

pub async fn get_recent(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>> {
    Ok(data::get_recent(db, user_id).await?)
}

pub async fn create_empty_sheet(db: &Db, user_id: i32, title: String) -> Result<Id> {
    let tiptap =
        serde_json::from_str("{\"type\": \"doc\", \"content\": [{\"type\": \"paragraph\"}]}")
            .expect("malformed JSON");
    create_sheet(db, user_id, title, tiptap).await
}

pub async fn create_sheet(
    db: &Db,
    user_id: i32,
    title: String,
    tiptap: serde_json::Value,
) -> Result<Id> {
    let now = chrono::Utc::now();
    Ok(data::create_sheet(db, title, tiptap, user_id, now, now, None).await?)
}

pub async fn get_sheet(db: &Db, id: Id) -> Result<Sheet> {
    Ok(data::get_sheet_by_id(db, id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("sheet {}", id)))?)
}

async fn get_sheet_owned_by_user(db: &Db, user_id: i32, id: Id) -> Result<Sheet> {
    let sheet = get_sheet(db, id).await?;
    if sheet.metadata.owner.id == user_id {
        Ok(sheet)
    } else {
        Err(Error::Forbidden(format!(
            "user {} is not owner of sheet {}",
            user_id, id
        )))
    }
}

pub async fn get_sheet_for_edit(db: &Db, user_id: i32, id: Id) -> Result<Sheet> {
    get_sheet_owned_by_user(db, user_id, id).await
}

pub async fn update_sheet(
    db: &Db,
    user_id: i32,
    id: Id,
    title: String,
    tiptap: serde_json::Value,
) -> Result<()> {
    get_sheet_owned_by_user(db, user_id, id).await?; // We don't care about the sheet here, we just need to check ownership
    let now = chrono::Utc::now();
    Ok(data::update_sheet(db, id, title, tiptap, now).await?)
}

pub async fn delete_sheet(db: &Db, user_id: i32, id: Id) -> Result<DeleteOutcome> {
    let sheet = get_sheet_owned_by_user(db, user_id, id).await?;
    if sheet.metadata.trashed.is_some() {
        data::delete_sheet(db, id).await?;
        Ok(DeleteOutcome::Deleted)
    } else {
        data::move_sheet_to_trash(db, id).await?;
        Ok(DeleteOutcome::Trashed)
    }
}

pub async fn restore_sheet(db: &Db, user_id: i32, id: Id) -> Result<()> {
    get_sheet_owned_by_user(db, user_id, id).await?; // We don't care about the sheet here, we just need to check ownership
    data::restore_sheet(db, id).await?;
    Ok(())
}
