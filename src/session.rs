use std::collections::HashMap;
use std::io;

use base64;
use chrono::NaiveDateTime;
use crypto::scrypt;
use log::error;
use rand::rngs::OsRng;
use rand::RngCore;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::postgres::{self, Row};

use crate::sheets;
use crate::Db;

type Error = postgres::Error;

#[derive(Debug)]
pub struct User {
    id: i32,
    username: String,
    password_hash: String,
    session: Option<Session>,
}

impl User {
    fn from(row: &Row, session: Option<Session>) -> Self {
        User {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            session,
        }
    }
    fn check_password(&self, password: &str) -> bool {
        check_password(password, &self.password_hash)
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
                    get_user_by_session_id(&db, session_id)
                        .await
                        .map_err(|e| error!("Error reading from database: {}", e))
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

#[derive(Debug)]
pub struct Session {
    id: String,
    expires: NaiveDateTime,
}

impl Session {
    fn from(row: &Row) -> Self {
        Session {
            id: row.get("session_id"),
            expires: row.get("expires"),
        }
    }
}

#[derive(FromForm)]
pub struct LoginForm {
    username: String,
    password: String,
}

const EXPIRY_DAYS: i64 = 5;
const SESSION_ID_COOKIE_NAME: &str = "session_id";

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
    match get_user_by_name(&db, form.username).await {
        Ok(Some(user)) => match login_user(&db, &user, &form.password).await {
            Ok(Some(session_id)) => {
                cookies.add_private(Cookie::new(SESSION_ID_COOKIE_NAME, session_id));
                Ok(Redirect::to(format!(
                    "{}{}",
                    sheets::MOUNT,
                    uri!(sheets::sheets)
                )))
            }
            Ok(None) => Ok(Redirect::to(uri!(login_form))),
            Err(e) => {
                error!("Error writing to database: {}", e);
                Err(Status::InternalServerError)
            }
        },
        Ok(None) => Ok(Redirect::to(uri!(login_form))),
        Err(e) => {
            error!("Error writing to database: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/logout")]
pub async fn logout(db: Db, user: &User, cookies: &CookieJar<'_>) -> Result<Redirect, Status> {
    cookies.remove_private(Cookie::named(SESSION_ID_COOKIE_NAME));
    delete_session(&db, user.id)
        .await
        .map(|_| Redirect::to(uri!(login_form)))
        .map_err(|e| {
            error!("Error writing to database: {}", e);
            Status::InternalServerError
        })
}

async fn login_user(
    db: &Db,
    user: &User,
    provided_password: &str,
) -> Result<Option<String>, Error> {
    if user.check_password(provided_password) {
        let session_id = if let Some(session) = &user.session {
            renew_session(&db, session.id.clone()).await?;
            session.id.clone()
        } else {
            create_session(&db, user.id).await?
        };
        Ok(Some(session_id))
    } else {
        Ok(None)
    }
}

fn register(form: LoginForm) {
    // Create new user from form and store in db
}

fn hash_password(password: &str) -> io::Result<String> {
    scrypt::scrypt_simple(password, &scrypt::ScryptParams::new(14, 16, 1))
}

fn check_password(password: &str, hash: &str) -> bool {
    scrypt::scrypt_check(password, hash).unwrap_or_else(|e| {
        error!("Error checking password: {}", e);
        false
    })
}

fn generate_session_id() -> String {
    let mut rng = OsRng {};
    let mut bytes: [u8; 96] = [0; 96];
    rng.fill_bytes(&mut bytes);
    base64::encode(bytes)
}

async fn get_user_by_name(db: &Db, name: String) -> Result<Option<User>, Error> {
    let row = db
        .run(move |c| {
            c.query_opt(
                "select u.id, u.username, u.password_hash, s.session_id, s.expires
                from users u left join sessions s on u.id = s.user_id
                where u.username = $1",
                &[&name],
            )
        })
        .await?;
    Ok(row.map(|r| {
        let session_id: Option<String> = r.get("session_id");
        let session = session_id.map(|_| Session::from(&r));
        User::from(&r, session)
    }))
}

async fn get_user_by_session_id(db: &Db, session_id: String) -> Result<Option<User>, Error> {
    let row = db
        .run(move |c| {
            c.query_opt(
                "select u.id, u.username, u.password_hash, s.session_id, s.expires
                from users u join sessions s on u.id = s.user_id
                where s.session_id = $1",
                &[&session_id],
            )
        })
        .await?;
    Ok(row.map(|r| User::from(&r, Some(Session::from(&r)))))
}

async fn create_session(db: &Db, for_user: i32) -> Result<String, Error> {
    let session_id = generate_session_id();
    let expires = chrono::Utc::now().naive_local() + chrono::Duration::days(EXPIRY_DAYS);
    let row = db
        .run(move |c| {
            c.query_one(
                "insert into sessions(session_id, user_id, expires)
                values ($1, $2, $3)
                returning session_id",
                &[&session_id, &for_user, &expires],
            )
        })
        .await?;
    Ok(row.get("session_id"))
}

async fn renew_session(db: &Db, session_id: String) -> Result<(), Error> {
    let expires = chrono::Utc::now().naive_local() + chrono::Duration::days(EXPIRY_DAYS);
    db.run(move |c| {
        c.execute(
            "update sessions
            set expires = $2
            where session_id = $1",
            &[&session_id, &expires],
        )
    })
    .await?;
    Ok(())
}

async fn delete_session(db: &Db, user_id: i32) -> Result<(), Error> {
    db.run(move |c| {
        c.execute(
            "delete from sessions
            where user_id = $1",
            &[&user_id],
        )
    })
    .await?;
    Ok(())
}
