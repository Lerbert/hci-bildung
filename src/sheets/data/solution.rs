use chrono::{DateTime, Utc};
use rocket_sync_db_pools::diesel;

use crate::db::model::{SolutionDiesel, SolutionMetadataDiesel, UserTransportDiesel};
use crate::db::schema::{solutions, users};
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

pub async fn get_solution_by_sheet_and_user_id(
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
