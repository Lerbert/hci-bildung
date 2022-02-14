use std::collections::HashMap;

use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct SheetContext {
    edit: bool,
}

#[get("/")]
pub fn sheets() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("docmgmt", &context)
}

#[get("/<id>")]
pub fn view_sheet(id: u32) -> Template {
    Template::render("sheet", &SheetContext { edit: false })
}

#[get("/<id>?edit")]
pub fn edit_sheet(id: u32) -> Template {
    Template::render("sheet", &SheetContext { edit: true })
}
