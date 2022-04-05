use chrono::{DateTime, NaiveDateTime, Utc};
use rocket::serde::uuid::Uuid;

use super::schema::{roles, sessions, sheets, solutions, users};
use super::sql_types::RoleDb;

#[derive(Debug, Identifiable, PartialEq, Queryable)]
#[table_name = "users"]
pub struct UserDiesel {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, PartialEq, Queryable)]
pub struct UserTransportDiesel {
    pub id: i32,
    pub username: String,
}

impl UserTransportDiesel {
    pub fn columns() -> (users::id, users::username) {
        (users::id, users::username)
    }
}

#[derive(Associations, Debug, Identifiable, PartialEq, Queryable)]
#[belongs_to(UserDiesel, foreign_key = "user_id")]
#[primary_key(user_id, role)]
#[table_name = "roles"]
pub struct RoleDiesel {
    pub user_id: i32,
    pub role: RoleDb,
}

#[derive(Associations, Debug, Identifiable, Insertable, PartialEq, Queryable)]
#[belongs_to(UserDiesel, foreign_key = "user_id")]
#[primary_key(session_id)]
#[table_name = "sessions"]
pub struct SessionDiesel {
    #[column_name = "session_id"]
    pub id: String,
    pub user_id: i32,
    pub expires: NaiveDateTime,
}

#[derive(Associations, Debug, Identifiable, Insertable, PartialEq, Queryable)]
#[belongs_to(UserDiesel, foreign_key = "owner_id")]
#[table_name = "sheets"]
pub struct SheetDiesel {
    pub id: Uuid,
    pub title: String,
    pub owner_id: i32,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub tiptap: serde_json::Value,
    pub trashed: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Queryable)]
pub struct SheetMetadataDiesel {
    pub id: Uuid,
    pub title: String,
    pub owner_id: i32,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub trashed: Option<DateTime<Utc>>,
}

impl SheetMetadataDiesel {
    pub fn columns() -> (
        sheets::id,
        sheets::title,
        sheets::owner_id,
        sheets::created,
        sheets::changed,
        sheets::trashed,
    ) {
        (
            sheets::id,
            sheets::title,
            sheets::owner_id,
            sheets::created,
            sheets::changed,
            sheets::trashed,
        )
    }
}

#[derive(Associations, Debug, Identifiable, Insertable, PartialEq, Queryable)]
#[belongs_to(UserDiesel, foreign_key = "owner_id")]
#[belongs_to(SheetDiesel, foreign_key = "sheet_id")]
#[table_name = "solutions"]
pub struct SolutionDiesel {
    pub id: i32,
    pub title: String,
    pub sheet_id: Option<Uuid>,
    pub sheet_version: DateTime<Utc>,
    pub owner_id: i32,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub trashed: Option<DateTime<Utc>>,
    pub solution: serde_json::Value,
}

#[derive(Debug, PartialEq, Queryable)]
pub struct SolutionMetadataDiesel {
    pub id: i32,
    pub title: String,
    pub sheet_id: Option<Uuid>,
    pub sheet_version: DateTime<Utc>,
    pub owner_id: i32,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub trashed: Option<DateTime<Utc>>,
}

impl SolutionMetadataDiesel {
    pub fn columns() -> (
        solutions::id,
        solutions::title,
        solutions::sheet_id,
        solutions::sheet_version,
        solutions::owner_id,
        solutions::created,
        solutions::changed,
        solutions::trashed,
    ) {
        (
            solutions::id,
            solutions::title,
            solutions::sheet_id,
            solutions::sheet_version,
            solutions::owner_id,
            solutions::created,
            solutions::changed,
            solutions::trashed,
        )
    }
}
