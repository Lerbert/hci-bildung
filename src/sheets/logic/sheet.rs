use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

use crate::login::transport::UserInfo;
use crate::Db;

use super::{data, DeleteOutcome, Error, Id, Result};

#[derive(Debug, Serialize)]
pub struct Sheet {
    pub metadata: SheetMetadata,
    pub content: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SheetMetadata {
    pub id: Id,
    pub title: String,
    pub owner: UserInfo,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub trashed: Option<DateTime<Utc>>,
}

pub async fn get_all_sheets(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>> {
    Ok(data::sheet::get_all_sheets(db, user_id).await?)
}

pub async fn get_trash(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>> {
    Ok(data::sheet::get_trash(db, user_id).await?)
}

pub async fn get_recent(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>> {
    Ok(data::sheet::get_recent(db, user_id).await?)
}

pub async fn get_updated(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>> {
    Ok(data::sheet::get_updated(db, user_id).await?)
}

pub async fn create_empty_sheet(db: &Db, user_id: i32, title: String) -> Result<Id> {
    let content =
        serde_json::from_str("{\"type\": \"doc\", \"content\": [{\"type\": \"paragraph\", \"content\": [], \"marks\": []}], \"marks\": []}")
            .expect("malformed JSON");
    create_sheet(db, user_id, title, content).await
}

pub async fn create_sheet(
    db: &Db,
    user_id: i32,
    title: String,
    content: serde_json::Value,
) -> Result<Id> {
    let now = chrono::Utc::now();
    Ok(data::sheet::create_sheet(db, title, content, user_id, now, now, None).await?)
}

pub async fn get_sheet_title(db: &Db, sheet_id: Id) -> Result<String> {
    Ok(data::sheet::get_sheet_title(db, sheet_id).await?)
}

pub async fn get_sheet(db: &Db, id: Id) -> Result<Sheet> {
    data::sheet::get_sheet_by_id(db, id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("sheet {}", id)))
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

pub async fn check_sheet_ownership(db: &Db, user_id: i32, id: Id) -> Result<()> {
    get_sheet_owned_by_user(db, user_id, id).await?; // We don't care about the sheet here, we just need to check ownership
    Ok(())
}

pub async fn get_sheet_for_edit(db: &Db, user_id: i32, id: Id) -> Result<Sheet> {
    get_sheet_owned_by_user(db, user_id, id).await
}

pub async fn update_sheet(
    db: &Db,
    user_id: i32,
    id: Id,
    title: String,
    content: serde_json::Value,
) -> Result<()> {
    check_sheet_ownership(db, user_id, id).await?;
    let now = chrono::Utc::now();
    Ok(data::sheet::update_sheet(db, id, title, content, now).await?)
}

pub async fn delete_sheet(db: &Db, user_id: i32, id: Id) -> Result<DeleteOutcome> {
    let sheet = get_sheet_owned_by_user(db, user_id, id).await?;
    if sheet.metadata.trashed.is_some() {
        data::sheet::delete_sheet(db, id).await?;
        Ok(DeleteOutcome::Deleted)
    } else {
        data::sheet::move_sheet_to_trash(db, id).await?;
        Ok(DeleteOutcome::Trashed)
    }
}

pub async fn restore_sheet(db: &Db, user_id: i32, id: Id) -> Result<()> {
    check_sheet_ownership(db, user_id, id).await?;
    data::sheet::restore_sheet(db, id).await?;
    Ok(())
}
