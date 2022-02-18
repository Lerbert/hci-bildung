use std::collections::HashMap;
use std::convert::From;

use log::{error, warn};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::postgres::{self, Row};

use crate::{Db, Id};

enum Error {
    RowCount,
    Other(postgres::Error),
}

impl From<postgres::Error> for Error {
    fn from(e: postgres::Error) -> Self {
        Self::Other(e)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Sheet {
    id: Id,
    title: String,
    tiptap: serde_json::Value,
}

impl Sheet {
    fn from(row: &Row) -> Self {
        let json: serde_json::Value = row.get("tiptap");
        Sheet {
            id: row.get("id"),
            title: row.get("title"),
            tiptap: json,
        }
    }
}

#[derive(Serialize)]
struct SheetContext {
    edit: bool,
    id: String,
    title: String,
    tiptap: String,
}

impl SheetContext {
    fn from(sheet: Sheet, edit: bool) -> Self {
        SheetContext {
            edit,
            id: sheet.id.to_string(),
            title: sheet.title,
            tiptap: sheet.tiptap.to_string(),
        }
    }
}

pub const MOUNT: &str = "/sheets";

#[get("/")]
pub fn sheets() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("docmgmt", &context)
}

#[post("/")]
pub async fn new_sheet(db: Db) -> Result<Redirect, Status> {
    match create_sheet(db).await {
        Ok(id) => Ok(Redirect::to(format!("{}{}", MOUNT, uri!(edit_sheet(id))))),
        Err(e) => {
            error!("Error writing to database: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<id>")]
pub async fn view_sheet(db: Db, id: Id) -> Result<Template, Status> {
    match get_sheet_by_id(db, id).await {
        Ok(sheet) => Ok(Template::render("sheet", &SheetContext::from(sheet, false))),
        Err(Error::RowCount) => Err(Status::NotFound),
        Err(Error::Other(e)) => {
            error!("Error reading from database: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<id>?edit")]
pub async fn edit_sheet(db: Db, id: Id) -> Result<Template, Status> {
    match get_sheet_by_id(db, id).await {
        Ok(sheet) => Ok(Template::render("sheet", &SheetContext::from(sheet, true))),
        Err(Error::RowCount) => Err(Status::NotFound),
        Err(Error::Other(e)) => {
            error!("Error reading from database: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/<id>", format = "json", data = "<sheet>")]
pub async fn save_sheet(db: Db, id: Id, sheet: Json<Sheet>) -> Result<(), Status> {
    let sheet = sheet.into_inner();
    if sheet.id != id {
        warn!("Mismatched IDs: {} and {}", id, sheet.id);
        Err(Status::BadRequest)
    } else {
        match update_sheet(db, sheet).await {
            Ok(()) => Ok(()),
            Err(e) => {
                error!("Error writing to database: {}", e);
                Err(Status::InternalServerError)
            }
        }
    }
}

async fn get_sheet_by_id(db: Db, id: Id) -> Result<Sheet, Error> {
    let rows = db
        .run(move |c| {
            c.query(
                "select id, title, tiptap
                from sheets
                where id = $1",
                &[&id],
            )
        })
        .await?;
    if rows.len() == 1 {
        Ok(Sheet::from(rows.first().unwrap()))
    } else {
        Err(Error::RowCount)
    }
}

async fn create_sheet(db: Db) -> Result<Id, postgres::Error> {
    let row = db
        .run(move |c| {
            c.query_one(
                "insert into sheets(title, tiptap)
                values ('', '{\"type\": \"doc\", \"content\": [{\"type\": \"paragraph\"}]}'::json)
                returning id",
                &[],
            )
        })
        .await?;
    Ok(row.get("id"))
}

async fn update_sheet(db: Db, sheet: Sheet) -> Result<(), postgres::Error> {
    println!("Update");
    db.run(move |c| {
        c.execute(
            "update sheets
            set title = $2, tiptap = $3
            where id = $1",
            &[&sheet.id, &sheet.title, &sheet.tiptap],
        )
    })
    .await?;
    Ok(())
}
