use rocket::http::Status;
use rocket::response::Redirect;

use crate::login::guards::Student;
use crate::status::ToStatus;
use crate::Db;

use super::logic;
use super::logic::Id;

#[get("/solutions")]
pub fn solution_overview() {}

#[get("/<id>/solutions")]
pub fn sheet_solutions(id: Id) {}

#[post("/<id>/solve")]
pub async fn start_solve(db: Db, student: Student<'_>, id: Id) -> Result<Redirect, Status> {
    let user = student.into_inner();
    logic::solution::start_solve(&db, id, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(uri!(my_solution(id))))
}

#[get("/<id>/solutions/my")]
pub fn my_solution(id: Id) {}

#[get("/<id>/solutions/<user_id>", rank = 2)]
pub fn student_solution(id: Id, user_id: i32) {}
