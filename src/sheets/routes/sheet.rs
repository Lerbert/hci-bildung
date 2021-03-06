use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::flash::{FlashContext, FlashRedirect};
use crate::login::guards::{AuthenticatedUser, Student, Teacher};
use crate::status::ToStatus;
use crate::validation::Validate;
use crate::Db;

use super::logic;
use super::logic::sheet::{Sheet, SheetMetadata};
use super::logic::solution::SolutionMetadata;
use super::logic::Id;
use super::sheet_tree;
use super::transport::{ImportSheetForm, NewSheetForm, SheetTransport};
use super::{handle_insufficient_permissions, sheets_uri};

#[derive(Serialize)]
struct SheetContext<'a> {
    sheet: Sheet,
    user: Option<&'a AuthenticatedUser>,
}

#[derive(Serialize)]
struct SheetOverviewContext<'a> {
    flash: Option<FlashContext>,
    sheets: Vec<SheetMetadata>,
    solutions: Vec<SolutionMetadata>,
    user: &'a AuthenticatedUser,
}

#[get("/")]
pub async fn sheet_overview_teacher(db: Db, teacher: Teacher<'_>) -> Result<Template, Status> {
    let user = teacher.into_inner();
    let user_id = user.user_info.id;
    let recent_sheets = logic::sheet::get_recent(&db, user_id)
        .await
        .map_err(|e| e.to_status())?;
    let recent_solutions = logic::solution::get_solutions_teacher(&db, user_id)
        .await
        .map_err(|e| e.to_status())?;
    Ok(Template::render(
        "management/overview/teacher",
        &SheetOverviewContext {
            flash: None,
            sheets: recent_sheets,
            solutions: recent_solutions,
            user,
        },
    ))
}

#[get("/", rank = 2)]
pub async fn sheet_overview_student(db: Db, student: Student<'_>) -> Result<Template, Status> {
    let user = student.into_inner();
    let user_id = user.user_info.id;
    let updated_sheets = logic::sheet::get_updated(&db, user_id)
        .await
        .map_err(|e| e.to_status())?;
    let recent_solutions = logic::solution::get_solutions_student(&db, user_id)
        .await
        .map_err(|e| e.to_status())?;
    Ok(Template::render(
        "management/overview/student",
        &SheetOverviewContext {
            flash: None,
            sheets: updated_sheets,
            solutions: recent_solutions,
            user,
        },
    ))
}

#[get("/", rank = 3)]
pub fn login_sheet_overview(user: Option<&AuthenticatedUser>) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[post("/", data = "<form>")]
pub async fn new_sheet(
    db: Db,
    teacher: Teacher<'_>,
    form: Form<NewSheetForm>,
) -> Result<Redirect, Status> {
    let user = teacher.into_inner();
    let form = form.into_inner();
    logic::sheet::create_empty_sheet(&db, user.user_info.id, form.title)
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
        Ok(sheet) => logic::sheet::create_sheet(&db, user.user_info.id, sheet.title, sheet.content)
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
    logic::sheet::get_sheet(&db, id)
        .await
        .map_err(|e| e.to_status())
        .map(|sheet| Template::render("sheet/view_sheet", &SheetContext { sheet, user }))
}

#[get("/<id>/edit")]
pub async fn edit_sheet(db: Db, teacher: Teacher<'_>, id: Id) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::sheet::get_sheet_for_edit(&db, user.user_info.id, id)
        .await
        .map_err(|e| e.to_status())
        .map(|sheet| {
            Template::render(
                "sheet/edit_sheet",
                &SheetContext {
                    sheet,
                    user: Some(user),
                },
            )
        })
}

#[get("/<_id>/edit", rank = 2)]
pub fn login_edit_sheet(
    user: Option<&AuthenticatedUser>,
    _id: Id,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
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
        logic::sheet::update_sheet(&db, user.user_info.id, id, sheet.title, sheet.content)
            .await
            .map_err(|e| e.to_status())
    }
}

#[delete("/<id>")]
pub async fn delete_sheet(db: Db, teacher: Teacher<'_>, id: Id) -> Result<Redirect, Status> {
    let user = teacher.into_inner();
    logic::sheet::delete_sheet(&db, user.user_info.id, id)
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
    logic::sheet::restore_sheet(&db, user.user_info.id, id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(sheets_uri(uri!(sheet_tree::assignment_overview))))
}
