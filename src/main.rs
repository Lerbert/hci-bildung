use std::collections::HashMap;

#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::route::RouteUri;
use rocket::serde::uuid::Uuid;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database, postgres};
use tera::{self, from_value, to_value, Function};

mod session;
mod sheets;

type Id = Uuid;

#[database("hci_bildung")]
pub struct Db(postgres::Client);

fn make_url_for(urls: HashMap<String, RouteUri<'static>>) -> impl Function {
    println!("{:?}", urls.keys());
    move |args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
        match args.get("endpoint") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => instantiate_uri(
                    urls.get(&v)
                        .ok_or::<tera::Error>(format!("Endpoint {} not found", v).into())?,
                    &args,
                ),
                Err(e) => Err(format!("Error parsing JSON: {}", e).into()),
            },
            None => Err("No endpoint argument specified".into()),
        }
    }
}

// Cannot use uri! Macro because we have to compute this dynamically
fn instantiate_uri(
    uri: &RouteUri,
    args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let path = uri
        .origin
        .path()
        .segments()
        .map(|s| {
            if s.starts_with('<') && s.ends_with('>') {
                let mut name = &s[1..(s.len() - 1)];

                if name.ends_with("..") {
                    name = &name[..(name.len() - 2)];
                }

                args.get(name)
                    .and_then(|val| from_value::<String>(val.clone()).ok())
                    .unwrap_or(s.into())
            } else {
                s.into()
            }
        })
        .collect::<Vec<String>>()
        .join("/");

    let query = uri.origin.query().map(|q| {
        {
            q.segments().map(|(k, v)| {
                let (k, v) = if k.starts_with('<') && k.ends_with('>') {
                    let mut name = &k[1..(k.len() - 1)];

                    if name.ends_with("..") {
                        name = &name[..(name.len() - 2)];
                    }

                    let v = args
                        .get(name)
                        .and_then(|val| from_value::<String>(val.clone()).ok())
                        .unwrap_or(v.into());
                    (name, v)
                } else {
                    (k, v.into())
                };
                if v == "" {
                    format!("{}", k)
                } else {
                    format!("{}={}", k, v)
                }
            })
        }
        .collect::<Vec<String>>()
        .join("&")
    });
    if let Some(query) = query {
        Ok(to_value(format!("/{}?{}", path, query)).unwrap())
    } else {
        Ok(to_value(format!("/{}", path)).unwrap())
    }
}

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
            sheets::MOUNT,
            routes![
                sheets::sheets,
                sheets::new_sheet,
                sheets::view_sheet,
                sheets::edit_sheet,
                sheets::save_sheet
            ],
        )
        .mount(
            "/",
            routes![session::login_form, session::login, session::logout],
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
            .add_template_file("vue_dist/sheet.html.tera", Some("sheet.html"))
            .ok();
        engines
            .tera
            .register_function("url_for", make_url_for(map.clone()));
    }))
    .attach(Db::fairing())
}
