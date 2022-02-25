use std::fmt::{self, Display};

use log::error;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::login::User;
use crate::status::ToStatus;
use crate::validation::Validate;
use crate::Db;

use super::logic;
use super::transport::{Id, NewSheetForm, SheetOverviewTransport, SheetTransport};

impl ToStatus for logic::Error {
    fn to_status(self) -> Status {
        error!("{}", self);
        Status::InternalServerError
    }
}

#[derive(Serialize)]
struct SheetContext {
    edit: bool,
    sheet: SheetTransport,
}

#[derive(Serialize)]
struct SheetManagementContext {
    sheets: Vec<SheetOverviewTransport>,
}

#[derive(Debug)]
pub enum SheetTransportValidationError {
    TitleEmpty,
    IdSet,
}

impl Display for SheetTransportValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TitleEmpty => write!(f, "Title cannot be empty"),
            Self::IdSet => write!(f, "Id cannot be set"),
        }
    }
}

impl ToStatus for SheetTransportValidationError {
    fn to_status(self) -> Status {
        error!("Sheet validation failed: {}", self);
        Status::BadRequest
    }
}

impl Validate for SheetTransport {
    type ValidationError = SheetTransportValidationError;

    fn validate(&self) -> Result<(), Self::ValidationError> {
        if self.title.is_empty() {
            return Err(Self::ValidationError::TitleEmpty);
        }
        if self.id.is_some() {
            return Err(Self::ValidationError::IdSet);
        }
        Ok(())
    }
}

pub const MOUNT: &str = "/sheets";

#[get("/")]
pub async fn sheets(db: Db, _user: &User) -> Result<Template, Status> {
    logic::get_all_sheets(&db)
        .await
        .map_err(|e| e.to_status())
        .map(|sheets| {
            Template::render(
                "docmgmt",
                &SheetManagementContext {
                    sheets: sheets.into_iter().map(|metadata| metadata.into()).collect(),
                },
            )
        })
}

#[post("/", data = "<form>")]
pub async fn new_sheet(db: Db, _user: &User, form: Form<NewSheetForm>) -> Result<Redirect, Status> {
    let form = form.into_inner();
    logic::create_sheet(&db, form.title)
        .await
        .map_err(|e| e.to_status())
        .map(|id| Redirect::to(format!("{}{}", MOUNT, uri!(edit_sheet(id)))))
}

#[get("/<id>")]
pub async fn view_sheet(db: Db, id: Id) -> Result<Template, Status> {
    logic::get_sheet_by_id(&db, id)
        .await
        .map_err(|e| e.to_status())
        .and_then(|s| {
            s.map(|sheet| {
                Ok(Template::render(
                    "sheet.html",
                    &SheetContext {
                        edit: false,
                        sheet: sheet.into(),
                    },
                ))
            })
            .unwrap_or_else(|| Err(Status::NotFound))
        })
}

#[get("/<id>?edit")]
pub async fn edit_sheet(db: Db, _user: &User, id: Id) -> Result<Template, Status> {
    logic::get_sheet_by_id(&db, id)
        .await
        .map_err(|e| e.to_status())
        .and_then(|s| {
            s.map(|sheet| {
                Ok(Template::render(
                    "sheet.html",
                    &SheetContext {
                        edit: true,
                        sheet: sheet.into(),
                    },
                ))
            })
            .unwrap_or_else(|| Err(Status::NotFound))
        })
}

#[put("/<id>", format = "json", data = "<sheet>")]
pub async fn save_sheet(
    db: Db,
    _user: &User,
    id: Id,
    sheet: Json<SheetTransport>,
) -> Result<(), Status> {
    let sheet = sheet.into_inner();
    if let Err(e) = sheet.validate() {
        Err(e.to_status())
    } else {
        logic::update_sheet(&db, id, sheet.title, sheet.tiptap)
            .await
            .map_err(|e| e.to_status())
    }
}