use log::{error, warn};
use rocket::form::Form;
use rocket::http::Status;
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
struct DisplaySheet {
    id: Option<String>,
    title: Option<String>,
    tiptap: Option<String>,
}

impl DisplaySheet {
    fn from(sheet: Sheet) -> Self {
        DisplaySheet {
            id: Some(sheet.id.to_string()),
            title: Some(sheet.title),
            tiptap: Some(sheet.tiptap.to_string()),
        }
    }
}

#[derive(Serialize)]
struct SheetContext {
    edit: bool,
    sheet: DisplaySheet,
}

#[derive(FromForm)]
pub struct NewSheetForm {
    title: String,
}

#[derive(Serialize)]
struct SheetManagementContext {
    sheets: Vec<DisplaySheet>,
}

pub const MOUNT: &str = "/sheets";

#[get("/")]
pub async fn sheets(db: Db) -> Result<Template, Status> {
    match get_all_sheets(db).await {
        Ok(sheets) => Ok(Template::render(
            "docmgmt",
            &SheetManagementContext {
                sheets: sheets
                    .into_iter()
                    .map(|(id, title)| DisplaySheet {
                        id: Some(id.to_string()),
                        title: Some(title),
                        tiptap: None,
                    })
                    .collect(),
            },
        )),
        Err(e) => {
            error!("Error reading from database: {}", e);
            Err(Status::InternalServerError)
        }
    }
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
        Ok(Some(sheet)) => Ok(Template::render(
            "sheet",
            &SheetContext {
                edit: false,
                sheet: DisplaySheet::from(sheet),
            },
        )),
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
        Ok(Some(sheet)) => Ok(Template::render(
            "sheet",
            &SheetContext {
                edit: true,
                sheet: DisplaySheet::from(sheet),
            },
        )),
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

async fn get_all_sheets(db: Db) -> Result<Vec<(Id, String)>, Error> {
    let rows = db
        .run(move |c| {
            c.query(
                "select id, title
            from sheets",
                &[],
            )
        })
        .await?;
    Ok(rows.iter().map(|r| (r.get("id"), r.get("title"))).collect())
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
