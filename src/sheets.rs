use std::convert::From;

use chrono::{DateTime, Local};
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

pub struct Sheet {
    metadata: SheetMetadata,
    tiptap: serde_json::Value,
}

impl Sheet {
    fn from(row: &Row) -> Self {
        Sheet {
            metadata: SheetMetadata::from(row),
            tiptap: row.get("tiptap"),
        }
    }
}

pub struct SheetMetadata {
    id: Id,
    title: String,
    created: DateTime<Local>,
    changed: DateTime<Local>,
}

impl SheetMetadata {
    fn from(row: &Row) -> Self {
        SheetMetadata {
            id: row.get("id"),
            title: row.get("title"),
            created: row.get("created"),
            changed: row.get("changed"),
        }
    }
}

// Transport

#[derive(Debug, Deserialize, Serialize)]
pub struct SheetTransport {
    id: Option<Id>,
    title: String,
    tiptap: serde_json::Value,
}

impl SheetTransport {
    fn validate(&self) -> bool {
        self.title != "" && self.id.is_none()
    }
}

impl From<Sheet> for SheetTransport {
    fn from(sheet: Sheet) -> Self {
        SheetTransport {
            id: Some(sheet.metadata.id),
            title: sheet.metadata.title,
            tiptap: sheet.tiptap,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SheetOverviewTransport {
    id: String,
    title: String,
    created: DateTime<Local>,
    changed: DateTime<Local>,
}

impl From<Sheet> for SheetOverviewTransport {
    fn from(sheet: Sheet) -> Self {
        SheetOverviewTransport::from(sheet.metadata)
    }
}

impl From<SheetMetadata> for SheetOverviewTransport {
    fn from(metadata: SheetMetadata) -> Self {
        SheetOverviewTransport {
            id: metadata.id.to_string(),
            title: metadata.title,
            created: metadata.created,
            changed: metadata.changed,
        }
    }
}

// Forms

#[derive(FromForm)]
pub struct NewSheetForm {
    #[field(validate = neq(""))]
    title: String,
}

// Contexts

#[derive(Serialize)]
struct SheetContext {
    edit: bool,
    sheet: SheetTransport,
}

#[derive(Serialize)]
struct SheetManagementContext {
    sheets: Vec<SheetOverviewTransport>,
}

pub const MOUNT: &str = "/sheets";

#[get("/")]
pub async fn sheets(db: Db) -> Result<Template, Status> {
    match get_all_sheets(db).await {
        Ok(sheets) => Ok(Template::render(
            "docmgmt",
            &SheetManagementContext {
                sheets: sheets.into_iter().map(|metadata| metadata.into()).collect(),
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
    match create_sheet(db, form.title).await {
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
                sheet: sheet.into(),
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
                sheet: sheet.into(),
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
pub async fn save_sheet(db: Db, id: Id, sheet: Json<SheetTransport>) -> Result<(), Status> {
    let sheet = sheet.into_inner();
    if sheet.validate() {
        match update_sheet(db, id, sheet.title, sheet.tiptap).await {
            Ok(()) => Ok(()),
            Err(e) => {
                error!("Error writing to database: {}", e);
                Err(Status::InternalServerError)
            }
        }
    } else {
        warn!("Sheet validation failed for {:?}", sheet);
        return Err(Status::BadRequest);
    }
}

async fn get_all_sheets(db: Db) -> Result<Vec<SheetMetadata>, Error> {
    let rows = db
        .run(move |c| {
            c.query(
                "select id, title, created, changed
                from sheets",
                &[],
            )
        })
        .await?;
    Ok(rows.iter().map(|r| SheetMetadata::from(r)).collect())
}

async fn get_sheet_by_id(db: Db, id: Id) -> Result<Option<Sheet>, Error> {
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
    Ok(row.map(|r| Sheet::from(&r)))
}

async fn create_sheet(db: Db, title: String) -> Result<Id, Error> {
    let row = db
        .run(move |c| {
            c.query_one(
                "insert into sheets(title, tiptap)
                values ($1, '{\"type\": \"doc\", \"content\": [{\"type\": \"paragraph\"}]}'::json)
                returning id",
                &[&title],
            )
        })
        .await?;
    Ok(row.get("id"))
}

async fn update_sheet(
    db: Db,
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
