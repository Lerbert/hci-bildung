use log::error;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::flash::{FlashContext, FlashRedirect};
use crate::sheets;
use crate::status::ToStatus;
use crate::Db;

use super::guards::{self, AuthenticatedUser};
use super::logic;
use super::transport::LoginForm;

impl ToStatus for logic::Error {
    fn to_status(self) -> Status {
        error!("{}", self);
        Status::InternalServerError
    }
}

#[derive(Serialize)]
struct LandingPageContext<'a> {
    user: Option<&'a AuthenticatedUser>,
}

#[derive(Serialize)]
struct LoginContext {
    flash: Option<FlashContext>,
}

#[get("/")]
pub fn landing_page(user: Option<&AuthenticatedUser>) -> Template {
    Template::render("landing_page", &LandingPageContext { user })
}

#[get("/login")]
pub fn already_logged_in(_user: &AuthenticatedUser) -> Redirect {
    Redirect::to(sheets::routes::sheets_uri(uri!(
        sheets::routes::sheet::sheet_overview_teacher
    )))
}

#[get("/login", rank = 2)]
pub fn login_form(flash: Option<FlashMessage>) -> Template {
    Template::render(
        "login",
        &LoginContext {
            flash: flash.map(|f| f.into()),
        },
    )
}

#[post("/login", data = "<form>")]
pub async fn login(
    db: Db,
    cookies: &CookieJar<'_>,
    form: Form<LoginForm>,
) -> Result<FlashRedirect, Status> {
    let form = form.into_inner();
    logic::login(&db, form.username, form.password)
        .await
        .map_err(|e| e.to_status())
        .and_then(|s| {
            s.map(|session_id| {
                cookies.add_private(Cookie::new(guards::SESSION_ID_COOKIE_NAME, session_id));
                Ok(FlashRedirect::no_flash(sheets::routes::sheets_uri(uri!(
                    sheets::routes::sheet::sheet_overview_teacher
                ))))
            })
            .unwrap_or_else(|| {
                Ok(FlashRedirect::with_flash(
                    uri!(login_form),
                    "danger",
                    "Nutzername oder Passwort ungültig",
                ))
            })
        })
}

#[get("/logout")]
pub async fn logout(
    db: Db,
    user: &AuthenticatedUser,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Status> {
    cookies.remove_private(Cookie::named(guards::SESSION_ID_COOKIE_NAME));
    logic::logout(&db, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(uri!(login_form)))
}
