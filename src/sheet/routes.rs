use std::fmt::{self, Display};

use log::{error, info};
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::flash::FlashRedirect;
use crate::login::{self, User, UserTransport};
use crate::status::ToStatus;
use crate::validation::Validate;
use crate::Db;

use super::logic;
use super::transport::{Id, NewSheetForm, SheetOverviewTransport, SheetTransport};

impl ToStatus for logic::Error {
    fn to_status(self) -> Status {
        match self {
            Self::Forbidden(_) => {
                info!("{}", self);
                Status::Forbidden
            }
            _ => {
                error!("{}", self);
                Status::InternalServerError
            }
        }
    }
}

#[derive(Serialize)]
struct SheetContext {
    edit: bool,
    sheet: SheetTransport,
    user: Option<UserTransport>,
}

#[derive(Serialize)]
struct SheetManagementContext {
    sheets: Vec<SheetOverviewTransport>,
    user: UserTransport,
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
pub async fn sheets(db: Db, user: &User) -> Result<Template, Status> {
    logic::get_all_sheets(&db, user)
        .await
        .map_err(|e| e.to_status())
        .map(|sheets| {
            Template::render(
                "docmgmt",
                &SheetManagementContext {
                    sheets: sheets.into_iter().map(|metadata| metadata.into()).collect(),
                    user: user.into(),
                },
            )
        })
}

#[get("/", rank = 2)]
pub fn sheets_login_req() -> FlashRedirect {
    redirect_to_login()
}

#[post("/", data = "<form>")]
pub async fn new_sheet(db: Db, user: &User, form: Form<NewSheetForm>) -> Result<Redirect, Status> {
    let form = form.into_inner();
    logic::create_sheet(&db, user, form.title)
        .await
        .map_err(|e| e.to_status())
        .map(|id| Redirect::to(format!("{}{}", MOUNT, uri!(edit_sheet(id)))))
}

#[get("/<id>?edit")]
pub async fn edit_sheet(db: Db, user: &User, id: Id) -> Result<Template, Status> {
    logic::get_sheet_for_edit(&db, user, id)
        .await
        .map_err(|e| e.to_status())
        .and_then(|s| {
            s.map(|sheet| {
                Ok(Template::render(
                    "sheet.html",
                    &SheetContext {
                        edit: true,
                        sheet: sheet.into(),
                        user: Some(user.into()),
                    },
                ))
            })
            .unwrap_or_else(|| Err(Status::NotFound))
        })
}

#[get("/<_>?edit", rank = 2)]
pub fn edit_login_req() -> FlashRedirect {
    redirect_to_login()
}

#[get("/<id>", rank = 3)]
pub async fn view_sheet(db: Db, user: Option<&User>, id: Id) -> Result<Template, Status> {
    logic::get_sheet(&db, id)
        .await
        .map_err(|e| e.to_status())
        .and_then(|s| {
            s.map(|sheet| {
                Ok(Template::render(
                    "sheet.html",
                    &SheetContext {
                        edit: false,
                        sheet: sheet.into(),
                        user: user.map(|u| u.into()),
                    },
                ))
            })
            .unwrap_or_else(|| Err(Status::NotFound))
        })
}

#[put("/<id>", format = "json", data = "<sheet>")]
pub async fn save_sheet(
    db: Db,
    user: &User,
    id: Id,
    sheet: Json<SheetTransport>,
) -> Result<(), Status> {
    let sheet = sheet.into_inner();
    if let Err(e) = sheet.validate() {
        Err(e.to_status())
    } else {
        logic::update_sheet(&db, user, id, sheet.title, sheet.tiptap)
            .await
            .map_err(|e| e.to_status())
            .and_then(|opt| opt.ok_or(Status::NotFound))
    }
}

fn redirect_to_login() -> FlashRedirect {
    FlashRedirect::with_flash(
        uri!(login::routes::login),
        "danger",
        "Anmeldung erforderlich",
    )
}
