use chrono::{DateTime, Local};
use rocket_sync_db_pools::postgres::{self, Row};

use crate::login::UserTransport;
use crate::Db;

use super::logic::{Sheet, SheetMetadata};
use super::transport::Id;

pub type Error = postgres::Error;

pub async fn get_all_sheets(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>, Error> {
    let rows = db
        .run(move |c| {
            c.query(
                "select s.id as sheet_id, s.title, s.created, s.changed, u.id as user_id, u.username
                from sheets s join users u on s.owner_id = u.id
                where u.id = $1",
                &[&user_id],
            )
        })
        .await?;
    Ok(rows.iter().map(sheet_metadata_from_row).collect())
}

pub async fn get_sheet_by_id(db: &Db, id: Id) -> Result<Option<Sheet>, Error> {
    let row = db
        .run(move |c| {
            c.query_opt(
                "select s.id as sheet_id, s.title, s.created, s.changed, s.tiptap, u.id as user_id, u.username
                from sheets s join users u on s.owner_id = u.id
                where s.id = $1",
                &[&id],
            )
        })
        .await?;
    Ok(row.map(|r| sheet_from_row(&r)))
}

pub async fn create_sheet(
    db: &Db,
    title: String,
    tiptap: serde_json::Value,
    owner_id: i32,
    created: DateTime<Local>,
    changed: DateTime<Local>,
) -> Result<Id, Error> {
    let row = db
        .run(move |c| {
            c.query_one(
                "insert into sheets(title, tiptap, owner_id, created, changed)
                values ($1, $2, $3, $4, $5)
                returning id",
                &[&title, &tiptap, &owner_id, &created, &changed],
            )
        })
        .await?;
    Ok(row.get("id"))
}

pub async fn update_sheet(
    db: &Db,
    id: Id,
    title: String,
    tiptap: serde_json::Value,
) -> Result<(), Error> {
    db.run(move |c| {
        c.execute(
            "update sheets
            set title = $2, tiptap = $3
            where id = $1",
            &[&id, &title, &tiptap],
        )
    })
    .await?;
    Ok(())
}

fn sheet_from_row(row: &Row) -> Sheet {
    Sheet {
        metadata: sheet_metadata_from_row(row),
        tiptap: row.get("tiptap"),
    }
}

fn sheet_metadata_from_row(row: &Row) -> SheetMetadata {
    SheetMetadata {
        id: row.get("sheet_id"),
        title: row.get("title"),
        owner: user_transport_from_row(row),
        created: row.get("created"),
        changed: row.get("changed"),
    }
}

fn user_transport_from_row(row: &Row) -> UserTransport {
    UserTransport {
        id: row.get("user_id"),
        username: row.get("username"),
    }
}
