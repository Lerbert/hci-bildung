use chrono::{DateTime, Utc};
use rocket_sync_db_pools::diesel;

use crate::db::model::{SolutionDiesel, SolutionMetadataDiesel, UserTransportDiesel};
use crate::db::schema::{sheets, solutions, users};
use crate::Db;

use super::logic::solution::{FreshSolution, Solution, SolutionMetadata};
use super::logic::Id;
use super::Error;

use self::diesel::prelude::*;

impl From<(SolutionDiesel, UserTransportDiesel)> for Solution {
    fn from(t: (SolutionDiesel, UserTransportDiesel)) -> Solution {
        let (s, u) = t;
        Solution {
            metadata: SolutionMetadata {
                id: s.id,
                title: s.title,
                sheet_id: s.sheet_id,
                sheet_version: s.sheet_version,
                owner: u.into(),
                created: s.created,
                changed: s.changed,
                trashed: s.trashed,
            },
            content: s.content,
        }
    }
}

impl From<(SolutionMetadataDiesel, UserTransportDiesel)> for SolutionMetadata {
    fn from(t: (SolutionMetadataDiesel, UserTransportDiesel)) -> SolutionMetadata {
        let (s, u) = t;
        SolutionMetadata {
            id: s.id,
            title: s.title,
            sheet_id: s.sheet_id,
            sheet_version: s.sheet_version,
            owner: u.into(),
            created: s.created,
            changed: s.changed,
            trashed: s.trashed,
        }
    }
}

pub async fn get_solutions_by_sheet_owner(
    db: &Db,
    user_id: i32,
) -> Result<Vec<SolutionMetadata>, Error> {
    let solutions: Vec<(SolutionMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            solutions::table
                .inner_join(users::table)
                .inner_join(sheets::table)
                .select((
                    SolutionMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(sheets::owner_id.eq(user_id))
                .filter(sheets::trashed.is_null())
                .filter(solutions::trashed.is_null())
                .order((solutions::changed.desc(), users::username.asc()))
                .load(c)
        })
        .await?;
    Ok(solutions.into_iter().map(|s| s.into()).collect())
}

pub async fn get_solutions_by_owner(db: &Db, user_id: i32) -> Result<Vec<SolutionMetadata>, Error> {
    let solutions: Vec<(SolutionMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            solutions::table
                .inner_join(users::table)
                .select((
                    SolutionMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(solutions::owner_id.eq(user_id))
                .filter(solutions::trashed.is_null())
                .order(solutions::sheet_version.desc())
                .load(c)
        })
        .await?;
    Ok(solutions.into_iter().map(|s| s.into()).collect())
}

pub async fn get_trash(db: &Db, user_id: i32) -> Result<Vec<SolutionMetadata>, Error> {
    let solutions: Vec<(SolutionMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            solutions::table
                .inner_join(users::table)
                .select((
                    SolutionMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(solutions::owner_id.eq(user_id))
                .filter(solutions::trashed.is_not_null())
                .order(solutions::trashed.desc())
                .load(c)
        })
        .await?;
    Ok(solutions.into_iter().map(|s| s.into()).collect())
}

pub async fn get_recent(db: &Db, user_id: i32) -> Result<Vec<SolutionMetadata>, Error> {
    let solutions: Vec<(SolutionMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            solutions::table
                .inner_join(users::table)
                .select((
                    SolutionMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(solutions::owner_id.eq(user_id))
                .filter(solutions::trashed.is_null())
                .order(solutions::changed.desc())
                .load(c)
        })
        .await?;
    Ok(solutions.into_iter().map(|s| s.into()).collect())
}

pub async fn get_all_sheet_solutions(
    db: &Db,
    sheet_id: Id,
) -> Result<Vec<SolutionMetadata>, Error> {
    let solutions: Vec<(SolutionMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            solutions::table
                .inner_join(users::table)
                .select((
                    SolutionMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(solutions::sheet_id.eq(sheet_id))
                .filter(solutions::trashed.is_null())
                .order((users::username.asc(), solutions::sheet_version.desc()))
                .load(c)
        })
        .await?;
    Ok(solutions.into_iter().map(|s| s.into()).collect())
}

pub async fn get_sheet_solutions_by_sheet_and_user_id(
    db: &Db,
    sheet_id: Id,
    user_id: i32,
) -> Result<Vec<SolutionMetadata>, Error> {
    let solutions: Vec<(SolutionMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            solutions::table
                .inner_join(users::table)
                .select((
                    SolutionMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(solutions::sheet_id.eq(sheet_id))
                .filter(solutions::owner_id.eq(user_id))
                .filter(solutions::trashed.is_null())
                .order(solutions::sheet_version.desc())
                .load(c)
        })
        .await?;
    Ok(solutions.into_iter().map(|s| s.into()).collect())
}

pub async fn create_solution(db: &Db, fresh_solution: FreshSolution) -> Result<i32, Error> {
    let solution: SolutionDiesel = db
        .run(move |c| {
            diesel::insert_into(solutions::table)
                .values(&(
                    solutions::title.eq(fresh_solution.title),
                    solutions::owner_id.eq(fresh_solution.owner_id),
                    solutions::sheet_id.eq(fresh_solution.sheet_id),
                    solutions::sheet_version.eq(fresh_solution.sheet_version),
                    solutions::created.eq(fresh_solution.created),
                    solutions::changed.eq(fresh_solution.changed),
                    solutions::trashed.eq(fresh_solution.trashed),
                    solutions::content.eq(fresh_solution.content),
                ))
                .get_result(c)
        })
        .await?;
    Ok(solution.id)
}

pub async fn get_solution_by_id(db: &Db, id: i32) -> Result<Option<Solution>, Error> {
    let solution: Option<(SolutionDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            solutions::table
                .inner_join(users::table)
                .select((solutions::all_columns, UserTransportDiesel::columns()))
                .filter(solutions::id.eq(id))
                .first(c)
                .optional()
        })
        .await?;
    Ok(solution.map(|s| s.into()))
}

pub async fn get_latest_solution_by_sheet_and_user_id(
    db: &Db,
    sheet_id: Id,
    user_id: i32,
) -> Result<Option<Solution>, Error> {
    let solution: Option<(SolutionDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            solutions::table
                .inner_join(users::table)
                .select((solutions::all_columns, UserTransportDiesel::columns()))
                .filter(solutions::sheet_id.eq(sheet_id))
                .filter(solutions::owner_id.eq(user_id))
                .order(solutions::sheet_version.desc())
                .first(c)
                .optional()
        })
        .await?;
    Ok(solution.map(|s| s.into()))
}

pub async fn update_solution(
    db: &Db,
    sheet_id: Id,
    user_id: i32,
    content: serde_json::Value,
    changed: DateTime<Utc>,
) -> Result<(), Error> {
    if let Some(latest_solution) =
        get_latest_solution_by_sheet_and_user_id(db, sheet_id, user_id).await?
    {
        db.run(move |c| {
            diesel::update(solutions::table.find(latest_solution.metadata.id))
                .set((
                    solutions::content.eq(content),
                    solutions::changed.eq(changed),
                ))
                .execute(c)
        })
        .await?;
    }
    Ok(())
}

pub async fn delete_solution(db: &Db, id: i32) -> Result<(), Error> {
    db.run(move |c| diesel::delete(solutions::table.find(id)).execute(c))
        .await?;
    Ok(())
}

pub async fn move_solution_to_trash(db: &Db, id: i32) -> Result<(), Error> {
    db.run(move |c| {
        diesel::update(solutions::table.find(id))
            .set(solutions::trashed.eq(Some(Utc::now())))
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn restore_solution(db: &Db, id: i32) -> Result<(), Error> {
    db.run(move |c| {
        diesel::update(solutions::table.find(id))
            .set(solutions::trashed.eq(None::<DateTime<Utc>>))
            .execute(c)
    })
    .await?;
    Ok(())
}
