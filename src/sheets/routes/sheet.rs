use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::flash::FlashRedirect;
use crate::login::guards::{AuthenticatedUser, Teacher};
use crate::login::transport::UserTransport;
use crate::status::ToStatus;
use crate::validation::Validate;
use crate::Db;

use super::logic::{Id, Sheet};
use super::transport::{ImportSheetForm, NewSheetForm, SheetTransport};
use super::{logic, redirect_to_login, sheet_tree, sheets_uri};

#[derive(Serialize)]
struct SheetContext<'a> {
    sheet: Sheet,
    user: Option<&'a UserTransport>,
}

#[get("/")]
pub fn sheet_overview() {}

#[post("/", data = "<form>")]
pub async fn new_sheet(
    db: Db,
    teacher: Teacher<'_>,
    form: Form<NewSheetForm>,
) -> Result<Redirect, Status> {
    let user = teacher.into_inner();
    let form = form.into_inner();
    logic::create_empty_sheet(&db, user.user_info.id, form.title)
        .await
        .map_err(|e| e.to_status())
        .map(|id| Redirect::to(sheets_uri(uri!(edit_sheet(id)))))
}

#[post("/import", data = "<form>")]
pub async fn import_sheet(
    db: Db,
    teacher: Teacher<'_>,
    form: Form<ImportSheetForm>,
) -> Result<FlashRedirect, Status> {
    let user = teacher.into_inner();
    let form = form.into_inner();
    match parse_sheet(&form.file) {
        Ok(sheet) => logic::create_sheet(&db, user.user_info.id, sheet.title, sheet.tiptap)
            .await
            .map_err(|e| e.to_status())
            .map(|id| FlashRedirect::no_flash(sheets_uri(uri!(edit_sheet(id))))),
        Err(redirect) => Ok(redirect),
    }
}

fn parse_sheet(sheet: &str) -> Result<SheetTransport, FlashRedirect> {
    let get_error_redirect = || {
        FlashRedirect::with_flash(
            sheets_uri(uri!(sheet_tree::assignment_overview)),
            "danger",
            "Invalides Dateiformat",
        )
    };
    let sheet: SheetTransport = serde_json::from_str(sheet).map_err(|e| {
        error!("JSON deserialization of sheet failed: {}", e);
        get_error_redirect()
    })?;
    sheet.validate().map_err(|e| {
        error!("Sheet validation failed: {}", e);
        get_error_redirect()
    })?;
    Ok(sheet)
}

#[get("/<id>")]
pub async fn view_sheet(
    db: Db,
    user: Option<&AuthenticatedUser>,
    id: Id,
) -> Result<Template, Status> {
    logic::get_sheet(&db, id)
        .await
        .map_err(|e| e.to_status())
        .map(|sheet| {
            Template::render(
                "sheet/view_sheet",
                &SheetContext {
                    sheet,
                    user: user.map(|u| &u.user_info),
                },
            )
        })
}

#[get("/<id>/edit")]
pub async fn edit_sheet(db: Db, teacher: Teacher<'_>, id: Id) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::get_sheet_for_edit(&db, user.user_info.id, id)
        .await
        .map_err(|e| e.to_status())
        .map(|sheet| {
            Template::render(
                "sheet/edit_sheet",
                &SheetContext {
                    sheet,
                    user: Some(&user.user_info),
                },
            )
        })
}

#[get("/<_id>/edit", rank = 2)]
pub fn login_edit_sheet(_id: Id) -> FlashRedirect {
    redirect_to_login()
}

#[put("/<id>", format = "json", data = "<sheet>")]
pub async fn save_sheet(
    db: Db,
    teacher: Teacher<'_>,
    id: Id,
    sheet: Json<SheetTransport>,
) -> Result<(), Status> {
    let user = teacher.into_inner();
    let sheet = sheet.into_inner();
    if let Err(e) = sheet.validate() {
        Err(e.to_status())
    } else {
        logic::update_sheet(&db, user.user_info.id, id, sheet.title, sheet.tiptap)
            .await
            .map_err(|e| e.to_status())
    }
}

#[delete("/<id>")]
pub async fn delete_sheet(db: Db, teacher: Teacher<'_>, id: Id) -> Result<Redirect, Status> {
    let user = teacher.into_inner();
    logic::delete_sheet(&db, user.user_info.id, id)
        .await
        .map_err(|e| e.to_status())
        .map(|outcome| match outcome {
            logic::DeleteOutcome::Trashed => {
                Redirect::to(sheets_uri(uri!(sheet_tree::assignment_overview)))
            }
            logic::DeleteOutcome::Deleted => {
                Redirect::to(sheets_uri(uri!(sheet_tree::trashed_sheets)))
            }
        })
}

#[post("/<id>/restore")]
pub async fn restore_sheet(db: Db, teacher: Teacher<'_>, id: Id) -> Result<Redirect, Status> {
    let user = teacher.into_inner();
    logic::restore_sheet(&db, user.user_info.id, id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(sheets_uri(uri!(sheet_tree::assignment_overview))))
}
