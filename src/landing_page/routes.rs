use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::login::guards::AuthenticatedUser;

#[derive(Serialize)]
struct LandingPageContext<'a> {
    user: Option<&'a AuthenticatedUser>,
}

#[get("/")]
pub fn landing_page(user: Option<&AuthenticatedUser>) -> Template {
    Template::render("landing_page", &LandingPageContext { user })
}

#[get("/demo")]
pub fn demo(user: Option<&AuthenticatedUser>) -> Template {
    Template::render("demo", &LandingPageContext { user })
}
