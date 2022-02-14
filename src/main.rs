use std::collections::HashMap;

#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("vue", &context)
}

#[get("/documents")]
fn documents() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("docmgmt", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, documents])
    .mount("/vue", FileServer::from(relative!("vue_dist/vue")))
    .mount("/assets", FileServer::from(relative!("assets")))
    .attach(Template::custom(|engines| {
        engines.tera.add_template_file("vue_dist/index.html", Some("vue_index")).ok();
    }))
}
