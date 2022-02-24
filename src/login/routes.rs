use log::error;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};
use std::collections::HashMap;

use rocket::response::Redirect;
use rocket_dyn_templates::Template;

use crate::sheets;
use crate::Db;

use super::logic::{self, User};
use super::transport::LoginForm;

const SESSION_ID_COOKIE_NAME: &str = "session_id";

trait ToStatus {
    fn to_status(self) -> Status;
}

impl ToStatus for logic::Error {
    fn to_status(self) -> Status {
        error!("{}", self);
        Status::InternalServerError
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let user = request
            .local_cache_async(async {
                let db = request.guard::<Db>().await.succeeded()?;
                if let Some(session_id) = request
                    .cookies()
                    .get_private(SESSION_ID_COOKIE_NAME)
                    .map(|cookie| cookie.value().to_owned())
                {
                    logic::get_session_user(&db, session_id)
                        .await
                        .map_err(|e| error!("{}", e))
                        .ok()
                        .flatten()
                } else {
                    None
                }
            })
            .await;
        user.as_ref().into_outcome((Status::Unauthorized, ()))
    }
}

#[get("/login")]
pub fn login_form() -> Template {
    let c: HashMap<String, String> = HashMap::new();
    Template::render("login", &c)
}

#[post("/login", data = "<form>")]
pub async fn login(
    db: Db,
    cookies: &CookieJar<'_>,
    form: Form<LoginForm>,
) -> Result<Redirect, Status> {
    let form = form.into_inner();
    logic::login(&db, form)
        .await
        .map_err(|e| e.to_status())
        .and_then(|s| {
            s.map(|session_id| {
                cookies.add_private(Cookie::new(SESSION_ID_COOKIE_NAME, session_id));
                Ok(Redirect::to(format!(
                    "{}{}",
                    sheets::MOUNT,
                    uri!(sheets::sheets)
                )))
            })
            .unwrap_or_else(|| Ok(Redirect::to(uri!(login_form))))
        })
}

#[get("/logout")]
pub async fn logout(db: Db, user: &User, cookies: &CookieJar<'_>) -> Result<Redirect, Status> {
    cookies.remove_private(Cookie::named(SESSION_ID_COOKIE_NAME));
    logic::logout(&db, user)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(uri!(login_form)))
}
