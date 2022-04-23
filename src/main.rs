use std::collections::HashMap;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket::route::RouteUri;
use rocket_dyn_templates::Template;

mod db;
mod flash;
mod landing_page;
mod login;
mod sheets;
mod status;
mod templating;
mod validation;

use db::Db;

fn setup_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    if let Err(e) = setup_logging() {
        panic!("{}", e);
    }
    let r = rocket::build()
        .mount(
            "/",
            routes![
                landing_page::routes::landing_page,
                landing_page::routes::demo
            ],
        )
        .mount(
            "/",
            routes![
                login::routes::login_form,
                login::routes::already_logged_in,
                login::routes::login,
                login::routes::logout
            ],
        )
        .mount(
            sheets::routes::MOUNT,
            routes![
                sheets::routes::sheet::sheet_overview_teacher,
                sheets::routes::sheet::sheet_overview_student,
                sheets::routes::sheet::new_sheet,
                sheets::routes::sheet::import_sheet,
                sheets::routes::sheet::view_sheet,
                sheets::routes::sheet::edit_sheet,
                sheets::routes::sheet::save_sheet,
                sheets::routes::sheet::delete_sheet,
                sheets::routes::sheet::restore_sheet,
                sheets::routes::sheet_tree::assignment_overview,
                sheets::routes::sheet_tree::trashed_sheets,
                sheets::routes::sheet_tree::recent_sheets,
                sheets::routes::solution::solution_overview,
                sheets::routes::solution::sheet_solutions,
                sheets::routes::solution::start_solve,
                sheets::routes::solution::my_solution_overview,
                sheets::routes::solution::trashed_solutions,
                sheets::routes::solution::recent_solutions,
                sheets::routes::solution::my_sheet_solutions,
                sheets::routes::solution::latest_solution,
                sheets::routes::solution::my_solution,
                sheets::routes::solution::save_solution,
                sheets::routes::solution::latest_student_solution,
                sheets::routes::solution::student_solution,
                sheets::routes::solution::delete_solution,
                sheets::routes::solution::restore_solution,
                sheets::routes::sheet::login_sheet_overview,
                sheets::routes::sheet::login_edit_sheet,
                sheets::routes::sheet_tree::login_assignment_overview,
                sheets::routes::sheet_tree::login_trashed_sheets,
                sheets::routes::sheet_tree::login_recent_sheets,
                sheets::routes::solution::login_solution_overview,
                sheets::routes::solution::login_sheet_solutions,
                sheets::routes::solution::login_my_solution_overview,
                sheets::routes::solution::login_trashed_solutions,
                sheets::routes::solution::login_recent_solutions,
                sheets::routes::solution::login_my_sheet_solutions,
                sheets::routes::solution::login_latest_solution,
                sheets::routes::solution::login_my_solution,
                sheets::routes::solution::login_latest_student_solution,
                sheets::routes::solution::login_student_solution
            ],
        )
        .mount("/vue", FileServer::from(relative!("vue_dist/vue")))
        .mount("/assets", FileServer::from(relative!("assets")));
    let map: HashMap<String, RouteUri> = r
        .routes()
        .map(|route| (route.name.clone().unwrap().into(), route.uri.clone()))
        .collect();
    r.attach(Template::custom(move |engines| {
        engines
            .tera
            .register_function("url_for", templating::make_url_for(map.clone()));
    }))
    .attach(Db::fairing())
    .attach(AdHoc::try_on_ignite(
        "Database Migrations",
        db::setup::migrate,
    ))
}
