use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::login::guards::Student;
use crate::login::transport::UserTransport;
use crate::status::ToStatus;
use crate::Db;

use super::logic::solution::Solution;
use super::logic::Id;
use super::logic;

#[derive(Serialize)]
struct SolutionContext<'a> {
    solution: Solution,
    user: &'a UserTransport,
}

#[get("/solutions")]
pub fn solution_overview() {}

#[get("/<sheet_id>/solutions")]
pub fn sheet_solutions(sheet_id: Id) {}

#[post("/<sheet_id>/solve")]
pub async fn start_solve(db: Db, student: Student<'_>, sheet_id: Id) -> Result<Redirect, Status> {
    let user = student.into_inner();
    logic::solution::start_solve(&db, sheet_id, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(uri!(my_solution(sheet_id))))
}

#[get("/<sheet_id>/solutions/my")]
pub async fn my_solution(db: Db, student: Student<'_>, sheet_id: Id) -> Result<Template, Status> {
    let user = student.into_inner();
    logic::solution::get_solution(&db, sheet_id, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|solution| {
            Template::render(
                "sheet/solution/my_solution",
                &SolutionContext{
                    solution,
                    user: &user.user_info,
                }
            )
        })
}

#[get("/<sheet_id>/solutions/<user_id>", rank = 2)]
pub fn student_solution(sheet_id: Id, user_id: i32) {}
