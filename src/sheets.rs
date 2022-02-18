use std::collections::HashMap;
use std::convert::From;

use log::warn;

use rocket::http::Status;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::postgres::{self, Row};

use crate::Db;

enum Error {
    RowCount,
    Other(postgres::Error),
}

impl From<postgres::Error> for Error {
    fn from(e: postgres::Error) -> Self {
        Self::Other(e)
    }
}

#[derive(Serialize)]
struct Sheet {
    id: i32,
    title: Option<String>,
    tiptap: Option<String>,
}

impl Sheet {
    fn from(row: &Row) -> Self {
        Sheet {
            id: row.get("id"),
            title: Some(row.get("title")),
            tiptap: Some(row.get("tiptap")),
        }
    }
}

#[derive(Serialize)]
struct SheetContext {
    edit: bool,
    sheet: Sheet,
}

#[get("/")]
pub fn sheets() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("docmgmt", &context)
}

#[get("/<id>")]
pub async fn view_sheet(db: Db, id: i32) -> Result<Template, Status> {
    match get_sheet_by_id(db, id).await {
        Ok(sheet) => Ok(Template::render(
            "sheet",
            &SheetContext { edit: false, sheet },
        )),
        Err(Error::RowCount) => Err(Status::NotFound),
        Err(Error::Other(e)) => {
            warn!("{}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<id>?edit")]
pub async fn edit_sheet(db: Db, id: i32) -> Result<Template, Status> {
    match get_sheet_by_id(db, id).await {
        Ok(sheet) => Ok(Template::render(
            "sheet",
            &SheetContext { edit: true, sheet },
        )),
        Err(Error::RowCount) => Err(Status::NotFound),
        Err(Error::Other(e)) => {
            warn!("{}", e);
            Err(Status::InternalServerError)
        }
    }
}

async fn get_sheet_by_id(db: Db, id: i32) -> Result<Sheet, Error> {
    let rows = db
        .run(move |c| {
            c.query(
                "
            select id, title, cast(tiptap as text)
            from sheets where id = $1
            ",
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
