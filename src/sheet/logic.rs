use std::fmt::{self, Display};

use chrono::{DateTime, Utc};

use crate::login::{User, UserTransport};
use crate::Db;

use super::data;

pub type Id = rocket::serde::uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DbError(data::Error),
    Forbidden(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbError(e) => write!(f, "Error interacting with database: {}", e),
            Self::Forbidden(msg) => write!(f, "Forbidden resource access: {}", msg),
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
    pub owner: UserTransport,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub trashed: Option<DateTime<Utc>>,
}

pub enum DeleteOutcome {
    Deleted,
    Trashed,
}

pub async fn get_all_sheets(db: &Db, user: &User) -> Result<Vec<SheetMetadata>> {
    Ok(data::get_all_sheets(db, user.id).await?)
}

pub async fn get_trash(db: &Db, user: &User) -> Result<Vec<SheetMetadata>> {
    Ok(data::get_trash(db, user.id).await?)
}

pub async fn create_empty_sheet(db: &Db, user: &User, title: String) -> Result<Id> {
    let tiptap =
        serde_json::from_str("{\"type\": \"doc\", \"content\": [{\"type\": \"paragraph\"}]}")
            .expect("malformed JSON");
    create_sheet(db, user, title, tiptap).await
}

pub async fn create_sheet(
    db: &Db,
    user: &User,
    title: String,
    tiptap: serde_json::Value,
) -> Result<Id> {
    let now = chrono::Utc::now();
    Ok(data::create_sheet(db, title, tiptap, user.id, now, now).await?)
}

pub async fn get_sheet(db: &Db, id: Id) -> Result<Option<Sheet>> {
    Ok(data::get_sheet_by_id(db, id).await?)
}

pub async fn get_sheet_for_edit(db: &Db, user: &User, id: Id) -> Result<Option<Sheet>> {
    let sheet = data::get_sheet_by_id(db, id).await?;
    sheet
        .map(|sheet| {
            if sheet.metadata.owner.id == user.id {
                Ok(sheet)
            } else {
                Err(Error::Forbidden(format!(
                    "edit sheet {} by user {}",
                    id, user.id
                )))
            }
        })
        .map_or(Ok(None), |v| v.map(Some))
}

pub async fn update_sheet(
    db: &Db,
    user: &User,
    id: Id,
    title: String,
    tiptap: serde_json::Value,
) -> Result<Option<()>> {
    let sheet = data::get_sheet_by_id(db, id).await?;
    if let Some(sheet) = sheet {
        if sheet.metadata.owner.id == user.id {
            data::update_sheet(db, id, title, tiptap).await?;
            Ok(Some(()))
        } else {
            Err(Error::Forbidden(format!(
                "edit sheet {} by user {}",
                id, user.id
            )))
        }
    } else {
        Ok(None)
    }
}

pub async fn delete_sheet(db: &Db, user: &User, id: Id) -> Result<Option<DeleteOutcome>> {
    let sheet = data::get_sheet_by_id(db, id).await?;
    if let Some(sheet) = sheet {
        if sheet.metadata.owner.id == user.id {
            if sheet.metadata.trashed.is_some() {
                data::delete_sheet(db, id).await?;
                Ok(Some(DeleteOutcome::Deleted))
            } else {
                data::move_sheet_to_trash(db, id).await?;
                Ok(Some(DeleteOutcome::Trashed))
            }
        } else {
            Err(Error::Forbidden(format!(
                "delete sheet {} by user {}",
                id, user.id
            )))
        }
    } else {
        Ok(None)
    }
}

pub async fn restore_sheet(db: &Db, user: &User, id: Id) -> Result<Option<()>> {
    let sheet = data::get_sheet_by_id(db, id).await?;
    if let Some(sheet) = sheet {
        if sheet.metadata.owner.id == user.id {
            data::restore_sheet(db, id).await?;
            Ok(Some(()))
        } else {
            Err(Error::Forbidden(format!(
                "restore sheet {} by user {}",
                id, user.id
            )))
        }
    } else {
        Ok(None)
    }
}	
