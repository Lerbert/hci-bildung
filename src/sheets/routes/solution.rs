use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::flash::{FlashContext, FlashRedirect};
use crate::login::guards::{Student, Teacher};
use crate::login::transport::UserTransport;
use crate::status::ToStatus;
use crate::Db;

use super::logic;
use super::logic::solution::{Solution, SolutionMetadata};
use super::logic::Id;
use super::transport::SolutionTransport;
use super::{redirect_to_login, sheets_uri};

#[derive(Serialize)]
struct SolutionContext<'a> {
    solution: Solution,
    user: &'a UserTransport,
}

#[derive(Serialize)]
struct SolutionManagementContext<'a> {
    flash: Option<FlashContext>,
    sheet_title: Option<String>,
    solutions: Vec<SolutionMetadata>,
    user: &'a UserTransport,
}

#[get("/solutions")]
pub fn solution_overview() {}

#[get("/solutions", rank = 2)]
pub fn login_solution_overview() -> FlashRedirect {
    redirect_to_login()
}

#[get("/solutions/trash")]
pub async fn trashed_solutions(db: Db, student: Student<'_>) -> Result<Template, Status> {
    let user = student.into_inner();
    logic::solution::get_trash(&db, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|solutions| {
            Template::render(
                "management/solution/trash",
                &SolutionManagementContext {
                    flash: None,
                    sheet_title: None,
                    solutions,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/solutions/trash", rank = 2)]
pub fn login_trashed_solutions() -> FlashRedirect {
    redirect_to_login()
}

#[get("/solutions/recent")]
pub async fn recent_solutions(db: Db, student: Student<'_>) -> Result<Template, Status> {
    let user = student.into_inner();
    logic::solution::get_recent(&db, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|solutions| {
            Template::render(
                "management/solution/recent_solutions",
                &SolutionManagementContext {
                    flash: None,
                    sheet_title: None,
                    solutions,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/solutions/recent", rank = 2)]
pub fn login_recent_solutions() -> FlashRedirect {
    redirect_to_login()
}

#[get("/<sheet_id>/solutions")]
pub async fn sheet_solutions_teacher(
    db: Db,
    teacher: Teacher<'_>,
    flash: Option<FlashMessage<'_>>,
    sheet_id: Id,
) -> Result<Template, Status> {
    let user = teacher.into_inner();
    let sheet_title = logic::sheet::get_sheet_title(&db, sheet_id)
        .await
        .map_err(|e| e.to_status())?;
    logic::solution::get_sheet_solutions_teacher(&db, user.user_info.id, sheet_id)
        .await
        .map_err(|e| e.to_status())
        .map(|solutions| {
            Template::render(
                "management/solution/sheet_solutions_teacher",
                &SolutionManagementContext {
                    flash: flash.map(|f| f.into()),
                    sheet_title: Some(sheet_title),
                    solutions,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/<sheet_id>/solutions", rank = 2)]
pub async fn sheet_solutions_student(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
) -> Result<Template, Status> {
    let user = student.into_inner();
    let sheet_title = logic::sheet::get_sheet_title(&db, sheet_id)
        .await
        .map_err(|e| e.to_status())?;
    logic::solution::get_sheet_solutions_student(&db, user.user_info.id, sheet_id)
        .await
        .map_err(|e| e.to_status())
        .map(|solutions| {
            Template::render(
                "management/solution/sheet_solutions_student",
                &SolutionManagementContext {
                    flash: None,
                    sheet_title: Some(sheet_title),
                    solutions,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/<_id>/solutions", rank = 3)]
pub fn login_sheet_solutions(_id: Id) -> FlashRedirect {
    redirect_to_login()
}

#[post("/<sheet_id>/solve")]
pub async fn start_solve(db: Db, student: Student<'_>, sheet_id: Id) -> Result<Redirect, Status> {
    let user = student.into_inner();
    logic::solution::start_solve(&db, sheet_id, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(sheets_uri(uri!(my_solution(sheet_id)))))
}

#[get("/<sheet_id>/solutions/my")]
pub async fn my_solution(db: Db, student: Student<'_>, sheet_id: Id) -> Result<Template, Status> {
    let user = student.into_inner();
    logic::solution::get_latest_solution(&db, sheet_id, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|solution| {
            Template::render(
                "sheet/solution/my_solution",
                &SolutionContext {
                    solution,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/<_id>/solutions/my", rank = 3)]
pub fn login_my_solution(_id: Id) -> FlashRedirect {
    redirect_to_login()
}

#[put("/<sheet_id>/solutions/my", format = "json", data = "<solution>")]
pub async fn save_solution(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
    solution: Json<SolutionTransport>,
) -> Result<(), Status> {
    let user = student.into_inner();
    let solution = solution.into_inner();
    logic::solution::update_solution(&db, sheet_id, user.user_info.id, solution.content)
        .await
        .map_err(|e| e.to_status())
}

#[get("/<sheet_id>/solutions/<student_id>", rank = 2)]
pub async fn student_solution(
    db: Db,
    teacher: Teacher<'_>,
    sheet_id: Id,
    student_id: i32,
) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::solution::get_solution_for_teacher(&db, sheet_id, user.user_info.id, student_id)
        .await
        .map_err(|e| e.to_status())
        .map(|solution| {
            Template::render(
                "sheet/solution/student_solution",
                &SolutionContext {
                    solution,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/<_sheet_id>/solutions/<_student_id>", rank = 4)]
pub fn login_student_solution(_sheet_id: Id, _student_id: i32) -> FlashRedirect {
    redirect_to_login()
}

#[delete("/<sheet_id>/solutions/<student_id>/<solution_id>")]
pub async fn delete_solution(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
    student_id: i32,
    solution_id: i32,
) -> Result<Redirect, Status> {
    let user = student.into_inner();
    logic::solution::delete_solution(&db, user.user_info.id, sheet_id, student_id, solution_id)
        .await
        .map_err(|e| e.to_status())
        .map(|outcome| match outcome {
            logic::DeleteOutcome::Trashed => Redirect::to(sheets_uri(uri!(solution_overview))),
            logic::DeleteOutcome::Deleted => Redirect::to(sheets_uri(uri!(trashed_solutions))),
        })
}

#[post("/<sheet_id>/solutions/<student_id>/<solution_id>/restore")]
pub async fn restore_solution(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
    student_id: i32,
    solution_id: i32,
) -> Result<Redirect, Status> {
    let user = student.into_inner();
    logic::solution::restore_solution(&db, user.user_info.id, sheet_id, student_id, solution_id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(sheets_uri(uri!(solution_overview))))
}
