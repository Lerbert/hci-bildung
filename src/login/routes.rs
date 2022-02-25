use log::error;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{FlashMessage, FromRequest, Outcome, Request};
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::flash::{FlashContext, FlashRedirect};
use crate::sheet;
use crate::status::ToStatus;
use crate::Db;

use super::logic::{self, User};
use super::transport::LoginForm;

const SESSION_ID_COOKIE_NAME: &str = "session_id";

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

#[derive(Serialize)]
struct LoginContext {
    flash: Option<FlashContext>,
}

#[get("/login")]
pub fn login_form(flash: Option<FlashMessage>) -> Template {
    println!("{:?}", flash);
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
                cookies.add_private(Cookie::new(SESSION_ID_COOKIE_NAME, session_id));
                Ok(FlashRedirect::no_flash(format!(
                    "{}{}",
                    sheet::routes::MOUNT,
                    uri!(sheet::routes::sheets)
                )))
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
pub async fn logout(db: Db, user: &User, cookies: &CookieJar<'_>) -> Result<Redirect, Status> {
    cookies.remove_private(Cookie::named(SESSION_ID_COOKIE_NAME));
    logic::logout(&db, user)
        .await
        .map_err(|e| e.to_status())
        .map(|_| Redirect::to(uri!(login_form)))
}
