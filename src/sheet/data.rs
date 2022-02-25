use chrono::{DateTime, Local};
use rocket_sync_db_pools::postgres::{self, Row};

use crate::Db;

use super::logic::{Sheet, SheetMetadata};
use super::transport::Id;

pub type Error = postgres::Error;

pub async fn get_all_sheets(db: &Db) -> Result<Vec<SheetMetadata>, Error> {
    let rows = db
        .run(move |c| {
            c.query(
                "select id, title, created, changed
                from sheets",
                &[],
            )
        })
        .await?;
    Ok(rows.iter().map(sheet_metadata_from_row).collect())
}

pub async fn get_sheet_by_id(db: &Db, id: Id) -> Result<Option<Sheet>, Error> {
    let row = db
        .run(move |c| {
            c.query_opt(
                "select id, title, created, changed, tiptap
                from sheets
                where id = $1",
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
    created: DateTime<Local>,
    changed: DateTime<Local>,
) -> Result<Id, Error> {
    let row = db
        .run(move |c| {
            c.query_one(
                "insert into sheets(title, tiptap, created, changed)
                values ($1, $2, $3, $4)
                returning id",
                &[&title, &tiptap, &created, &changed],
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
        id: row.get("id"),
        title: row.get("title"),
        created: row.get("created"),
        changed: row.get("changed"),
    }
}
