use super::logic::Id;

#[get("/solutions")]
pub fn solution_overview() {}

#[get("/<id>/solutions")]
pub fn sheet_solutions(id: Id) {}

#[post("/<id>/solve")]
pub fn start_solve(id: Id) {}

#[get("/<id>/solutions/my")]
pub fn my_solution(id: Id) {}

#[get("/<id>/solutions/<user_id>", rank = 2)]
pub fn student_solution(id: Id, user_id: i32) {}
