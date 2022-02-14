use std::collections::HashMap;

#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use tera::{self, from_value, to_value, Function};

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

fn make_url_for(urls: HashMap<String, String>) -> impl Function {
    move |args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => Ok(to_value(urls.get(&v).unwrap()).unwrap()),
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    }
}

#[launch]
fn rocket() -> _ {
    let r = rocket::build()
        .mount("/", routes![index, documents])
        .mount("/vue", FileServer::from(relative!("vue_dist/vue")))
        .mount("/assets", FileServer::from(relative!("assets")));
    let map: HashMap<String, String> = r
        .routes()
        .map(|route| {
            (
                route.name.clone().unwrap().into(),
                route.uri.as_str().into(),
            )
        })
        .collect();
    r.attach(Template::custom(move |engines| {
        engines
            .tera
            .add_template_file("vue_dist/index.html", Some("vue_index"))
            .ok();
        engines
            .tera
            .register_function("url_for", make_url_for(map.clone()));
    }))
}
