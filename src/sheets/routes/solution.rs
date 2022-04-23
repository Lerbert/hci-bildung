use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::flash::{FlashContext, FlashRedirect};
use crate::login::guards::{AuthenticatedUser, Student, Teacher};
use crate::status::ToStatus;
use crate::Db;

use super::logic;
use super::logic::solution::{Solution, SolutionMetadata};
use super::logic::Id;
use super::transport::SolutionTransport;
use super::{handle_insufficient_permissions, sheets_uri};

#[derive(Serialize)]
struct SolutionContext<'a> {
    solution: Solution,
    user: &'a AuthenticatedUser,
}

#[derive(Serialize)]
struct SolutionManagementContext<'a> {
    flash: Option<FlashContext>,
    sheet_title: Option<String>,
    solutions: Vec<SolutionMetadata>,
    user: &'a AuthenticatedUser,
}

#[get("/solutions")]
pub async fn solution_overview(db: Db, teacher: Teacher<'_>) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::solution::get_solutions_teacher(&db, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|solutions| {
            Template::render(
                "management/solution/my_solutions_teacher",
                &SolutionManagementContext {
                    flash: None,
                    sheet_title: None,
                    solutions,
                    user,
                },
            )
        })
}

#[get("/solutions", rank = 2)]
pub fn login_solution_overview(user: Option<&AuthenticatedUser>) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/solutions/my")]
pub async fn my_solution_overview(db: Db, student: Student<'_>) -> Result<Template, Status> {
    let user = student.into_inner();
    logic::solution::get_solutions_student(&db, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|solutions| {
            Template::render(
                "management/solution/my_solutions_student",
                &SolutionManagementContext {
                    flash: None,
                    sheet_title: None,
                    solutions,
                    user,
                },
            )
        })
}

#[get("/solutions/my", rank = 2)]
pub fn login_my_solution_overview(
    user: Option<&AuthenticatedUser>,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/solutions/my/trash")]
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
                    user,
                },
            )
        })
}

#[get("/solutions/my/trash", rank = 2)]
pub fn login_trashed_solutions(user: Option<&AuthenticatedUser>) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/solutions/my/recent")]
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
                    user,
                },
            )
        })
}

#[get("/solutions/my/recent", rank = 2)]
pub fn login_recent_solutions(user: Option<&AuthenticatedUser>) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/<sheet_id>/solutions")]
pub async fn sheet_solutions(
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
                    user,
                },
            )
        })
}

#[get("/<_id>/solutions", rank = 2)]
pub fn login_sheet_solutions(
    user: Option<&AuthenticatedUser>,
    _id: Id,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[post("/<sheet_id>/solve")]
pub async fn start_solve(db: Db, student: Student<'_>, sheet_id: Id) -> Result<Redirect, Status> {
    let user = student.into_inner();
    logic::solution::start_solve(&db, sheet_id, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(sheets_uri(uri!(latest_solution(sheet_id)))))
}

#[get("/<sheet_id>/solutions/my")]
pub async fn my_sheet_solutions(
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
                    user,
                },
            )
        })
}

#[get("/<_id>/solutions/my", rank = 2)]
pub fn login_my_sheet_solutions(
    user: Option<&AuthenticatedUser>,
    _id: Id,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/<sheet_id>/solutions/my/latest")]
pub async fn latest_solution(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
) -> Result<Template, Status> {
    let user = student.into_inner();
    logic::solution::get_latest_solution(&db, sheet_id, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|solution| {
            Template::render(
                "sheet/solution/my_solution",
                &SolutionContext { solution, user },
            )
        })
}

#[get("/<_sheet_id>/solutions/my/latest", rank = 2)]
pub fn login_latest_solution(
    user: Option<&AuthenticatedUser>,
    _sheet_id: Id,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/<sheet_id>/solutions/my/<solution_id>", rank = 3)]
pub async fn my_solution(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
    solution_id: i32,
) -> Result<Template, Status> {
    let user = student.into_inner();
    logic::solution::get_my_solution(&db, user.user_info.id, sheet_id, solution_id)
        .await
        .map_err(|e| e.to_status())
        .map(|solution| {
            Template::render(
                "sheet/solution/my_solution",
                &SolutionContext { solution, user },
            )
        })
}

#[get("/<_sheet_id>/solutions/my/<_solution_id>", rank = 4)]
pub fn login_my_solution(
    user: Option<&AuthenticatedUser>,
    _sheet_id: Id,
    _solution_id: i32,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[put(
    "/<sheet_id>/solutions/my/<solution_id>",
    format = "json",
    data = "<solution>"
)]
pub async fn save_solution(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
    solution_id: i32,
    solution: Json<SolutionTransport>,
) -> Result<(), Status> {
    let user = student.into_inner();
    let solution = solution.into_inner();
    logic::solution::update_solution(
        &db,
        user.user_info.id,
        sheet_id,
        solution_id,
        solution.content,
    )
    .await
    .map_err(|e| e.to_status())
}

#[delete("/<sheet_id>/solutions/my/<solution_id>")]
pub async fn delete_solution(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
    solution_id: i32,
) -> Result<Redirect, Status> {
    let user = student.into_inner();
    logic::solution::delete_solution(&db, user.user_info.id, sheet_id, solution_id)
        .await
        .map_err(|e| e.to_status())
        .map(|outcome| match outcome {
            logic::DeleteOutcome::Trashed => Redirect::to(sheets_uri(uri!(my_solution_overview))),
            logic::DeleteOutcome::Deleted => Redirect::to(sheets_uri(uri!(trashed_solutions))),
        })
}

#[post("/<sheet_id>/solutions/my/<solution_id>/restore")]
pub async fn restore_solution(
    db: Db,
    student: Student<'_>,
    sheet_id: Id,
    solution_id: i32,
) -> Result<Redirect, Status> {
    let user = student.into_inner();
    logic::solution::restore_solution(&db, user.user_info.id, sheet_id, solution_id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(sheets_uri(uri!(my_solution_overview))))
}

#[get("/<sheet_id>/solutions/<student_id>/latest", rank = 5)]
pub async fn latest_student_solution(
    db: Db,
    teacher: Teacher<'_>,
    sheet_id: Id,
    student_id: i32,
) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::solution::get_latest_solution_for_teacher(&db, user.user_info.id, sheet_id, student_id)
        .await
        .map_err(|e| e.to_status())
        .map(|solution| {
            Template::render(
                "sheet/solution/student_solution",
                &SolutionContext { solution, user },
            )
        })
}

#[get("/<_sheet_id>/solutions/<_student_id>/latest", rank = 6)]
pub fn login_latest_student_solution(
    user: Option<&AuthenticatedUser>,
    _sheet_id: Id,
    _student_id: i32,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/<sheet_id>/solutions/<student_id>/<solution_id>", rank = 7)]
pub async fn student_solution(
    db: Db,
    teacher: Teacher<'_>,
    sheet_id: Id,
    student_id: i32,
    solution_id: i32,
) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::solution::get_solution_for_teacher(
        &db,
        user.user_info.id,
        sheet_id,
        student_id,
        solution_id,
    )
    .await
    .map_err(|e| e.to_status())
    .map(|solution| {
        Template::render(
            "sheet/solution/student_solution",
            &SolutionContext { solution, user },
        )
    })
}

#[get("/<_sheet_id>/solutions/<_student_id>/<_solution_id>", rank = 8)]
pub fn login_student_solution(
    user: Option<&AuthenticatedUser>,
    _sheet_id: Id,
    _student_id: i32,
    _solution_id: i32,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}
