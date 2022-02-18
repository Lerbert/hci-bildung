use std::collections::HashMap;

use log::{error, warn};
use rocket::http::Status;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::postgres::{self, Row};

use crate::{Db, Id};

type Error = postgres::Error;

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

#[derive(FromForm)]
pub struct NewSheetForm {
    title: String,
}

pub const MOUNT: &str = "/sheets";

#[get("/")]
pub fn sheets() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("docmgmt", &context)
}

#[post("/", data = "<form>")]
pub async fn new_sheet(db: Db, form: Form<NewSheetForm>) -> Result<Redirect, Status> {
    let form = form.into_inner();
    match create_sheet(db, Some(form.title)).await {
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
        Ok(Some(sheet)) => Ok(Template::render("sheet", &SheetContext::from(sheet, false))),
        Ok(None) => Err(Status::NotFound),
        Err(e) => {
            error!("Error reading from database: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<id>?edit")]
pub async fn edit_sheet(db: Db, id: Id) -> Result<Template, Status> {
    match get_sheet_by_id(db, id).await {
        Ok(Some(sheet)) => Ok(Template::render("sheet", &SheetContext::from(sheet, true))),
        Ok(None) => Err(Status::NotFound),
        Err(e) => {
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

async fn get_sheet_by_id(db: Db, id: Id) -> Result<Option<Sheet>, Error> {
    let row = db
        .run(move |c| {
            c.query_opt(
                "select id, title, tiptap
                from sheets
                where id = $1",
                &[&id],
            )
        })
        .await?;
    Ok(row.map(|r| Sheet::from(&r)))
}

async fn create_sheet(db: Db, title: Option<String>) -> Result<Id, Error> {
    let row = db
        .run(move |c| {
            c.query_one(
                "insert into sheets(title, tiptap)
                values ($1, '{\"type\": \"doc\", \"content\": [{\"type\": \"paragraph\"}]}'::json)
                returning id",
                &[&title.unwrap_or("".to_owned())],
            )
        })
        .await?;
    Ok(row.get("id"))
}

async fn update_sheet(db: Db, sheet: Sheet) -> Result<(), Error> {
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
