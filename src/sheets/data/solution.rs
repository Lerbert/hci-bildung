use chrono::{DateTime, Utc};
use rocket_sync_db_pools::diesel;

use crate::db::model::{SolutionDiesel, SolutionMetadataDiesel, UserTransportDiesel};
use crate::db::schema::{solutions, users};
use crate::Db;

use super::logic::solution::{Solution, SolutionMetadata};
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
            solution: s.solution,
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
