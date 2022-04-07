use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

use crate::login::transport::UserTransport;
use crate::Db;

use super::sheet::{Sheet, SheetMetadata};
use super::{data, sheet};
use super::{Error, Id, Result};

#[derive(Debug, Serialize)]
pub struct Solution {
    pub metadata: SolutionMetadata,
    pub content: serde_json::Value,
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

pub struct FreshSolution {
    pub title: String,
    pub sheet_id: Id,
    pub sheet_version: DateTime<Utc>,
    pub owner_id: i32,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub trashed: Option<DateTime<Utc>>,
    pub content: serde_json::Value,
}

impl FreshSolution {
    fn from(sheet: Sheet, user_id: i32) -> FreshSolution {
        let now = Utc::now();
        FreshSolution {
            title: sheet.metadata.title,
            sheet_id: sheet.metadata.id,
            sheet_version: sheet.metadata.changed,
            owner_id: user_id,
            created: now,
            changed: now,
            trashed: None,
            content: sheet.content,
        }
    }
}

pub async fn start_solve(db: &Db, sheet_id: Id, user_id: i32) -> Result<()> {
    let sheet = sheet::get_sheet(db, sheet_id).await?;
    data::solution::create_solution(db, FreshSolution::from(sheet, user_id)).await?;
    Ok(())
}

pub async fn get_solution(db: &Db, sheet_id: Id, user_id: i32) -> Result<Solution> {
    data::solution::get_solution_by_sheet_and_user_id(db, sheet_id, user_id)
        .await?
        .ok_or_else(|| {
            Error::NotFound(format!(
                "solution for sheet {} by user {}",
                sheet_id, user_id
            ))
        })
}

pub async fn get_solution_for_teacher(
    db: &Db,
    sheet_id: Id,
    teacher_id: i32,
    student_id: i32,
) -> Result<Solution> {
    sheet::check_sheet_ownership(db, teacher_id, sheet_id).await?;
    get_solution(db, sheet_id, student_id).await
}
