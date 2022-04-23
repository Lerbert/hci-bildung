use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

use crate::login::transport::UserTransport;
use crate::Db;

use super::sheet::Sheet;
use super::{data, sheet, DeleteOutcome};
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

pub async fn get_solutions_teacher(db: &Db, user_id: i32) -> Result<Vec<SolutionMetadata>> {
    Ok(data::solution::get_solutions_by_sheet_owner(db, user_id).await?)
}

pub async fn get_solutions_student(db: &Db, user_id: i32) -> Result<Vec<SolutionMetadata>> {
    Ok(data::solution::get_solutions_by_owner(db, user_id).await?)
}

pub async fn get_trash(db: &Db, user_id: i32) -> Result<Vec<SolutionMetadata>> {
    Ok(data::solution::get_trash(db, user_id).await?)
}

pub async fn get_recent(db: &Db, user_id: i32) -> Result<Vec<SolutionMetadata>> {
    Ok(data::solution::get_recent(db, user_id).await?)
}

pub async fn get_sheet_solutions_teacher(
    db: &Db,
    user_id: i32,
    sheet_id: Id,
) -> Result<Vec<SolutionMetadata>> {
    sheet::check_sheet_ownership(db, user_id, sheet_id).await?;
    Ok(data::solution::get_all_sheet_solutions(db, sheet_id).await?)
}

pub async fn get_sheet_solutions_student(
    db: &Db,
    user_id: i32,
    sheet_id: Id,
) -> Result<Vec<SolutionMetadata>> {
    Ok(data::solution::get_sheet_solutions_by_sheet_and_user_id(db, sheet_id, user_id).await?)
}

pub async fn start_solve(db: &Db, sheet_id: Id, user_id: i32) -> Result<()> {
    let sheet = sheet::get_sheet(db, sheet_id).await?;
    let solution = get_latest_solution(db, sheet_id, user_id).await;
    match solution {
        Ok(solution) => {
            if solution.metadata.sheet_version < sheet.metadata.changed {
                create_solution(db, sheet, user_id).await
            } else {
                Ok(())
            }
        }
        Err(Error::NotFound(_)) => create_solution(db, sheet, user_id).await,
        Err(e) => Err(e),
    }
}

async fn create_solution(db: &Db, sheet: Sheet, user_id: i32) -> Result<()> {
    let fresh_solution = FreshSolution::from(sheet, user_id);
    data::solution::create_solution(db, fresh_solution).await?;
    Ok(())
}

pub async fn get_latest_solution(db: &Db, sheet_id: Id, user_id: i32) -> Result<Solution> {
    data::solution::get_latest_solution_by_sheet_and_user_id(db, sheet_id, user_id)
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
    get_latest_solution(db, sheet_id, student_id).await
}

async fn get_solution(db: &Db, solution_id: i32) -> Result<Solution> {
    data::solution::get_solution_by_id(db, solution_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("solution {}", solution_id)))
}

async fn get_solution_owned_by_user(db: &Db, user_id: i32, solution_id: i32) -> Result<Solution> {
    let solution = get_solution(db, solution_id).await?;
    if solution.metadata.owner.id == user_id {
        Ok(solution)
    } else {
        Err(Error::Forbidden(format!(
            "user {} is not the owner of solution {}",
            user_id, solution_id
        )))
    }
}

fn check_coherence(solution: &Solution, sheet_id: Id, student_id: i32) -> Result<()> {
    if solution.metadata.sheet_id != Some(sheet_id) || solution.metadata.owner.id != student_id {
        Err(Error::NotFound(format!(
            "solution {} does not match sheet {} or user {}",
            solution.metadata.id, sheet_id, student_id
        )))
    } else {
        Ok(())
    }
}

pub async fn get_my_solution(
    db: &Db,
    user_id: i32,
    sheet_id: Id,
    solution_id: i32,
) -> Result<Solution> {
    let solution = get_solution_owned_by_user(db, user_id, solution_id).await?;
    check_coherence(&solution, sheet_id, user_id)?;
    Ok(solution)
}

async fn check_solution_ownership(
    db: &Db,
    user_id: i32,
    sheet_id: Id,
    solution_id: i32,
) -> Result<()> {
    get_my_solution(db, user_id, sheet_id, solution_id).await?; // We don't care about the solution here, we just need to check ownership
    Ok(())
}

pub async fn update_solution(
    db: &Db,
    user_id: i32,
    sheet_id: Id,
    solution_id: i32,
    content: serde_json::Value,
) -> Result<()> {
    check_solution_ownership(db, user_id, sheet_id, solution_id).await?;
    let now = Utc::now();
    Ok(data::solution::update_solution(db, solution_id, content, now).await?)
}

pub async fn delete_solution(
    db: &Db,
    user_id: i32,
    sheet_id: Id,
    solution_id: i32,
) -> Result<DeleteOutcome> {
    let solution = get_my_solution(db, user_id, sheet_id, solution_id).await?;
    if solution.metadata.trashed.is_some() {
        data::solution::delete_solution(db, solution_id).await?;
        Ok(DeleteOutcome::Deleted)
    } else {
        data::solution::move_solution_to_trash(db, solution_id).await?;
        Ok(DeleteOutcome::Trashed)
    }
}

pub async fn restore_solution(db: &Db, user_id: i32, sheet_id: Id, solution_id: i32) -> Result<()> {
    check_solution_ownership(db, user_id, sheet_id, solution_id).await?;
    data::solution::restore_solution(db, solution_id).await?;
    Ok(())
}
