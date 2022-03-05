use chrono::NaiveDateTime;
use rocket_sync_db_pools::diesel;

use crate::Db;

use super::logic::{Session, User};

use self::diesel::prelude::*;
use crate::schema::{sessions, users};

pub type Error = diesel::result::Error;

#[derive(Debug, Queryable)]
struct UserDiesel {
    id: i32,
    username: String,
    password_hash: String,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "sessions"]
struct SessionDiesel {
    #[column_name = "session_id"]
    id: String,
    user_id: i32,
    expires: NaiveDateTime,
}

impl From<SessionDiesel> for Session {
    fn from(session: SessionDiesel) -> Session {
        Session {
            id: session.id,
            expires: session.expires,
        }
    }
}

impl From<(UserDiesel, Option<SessionDiesel>)> for User {
    fn from(t: (UserDiesel, Option<SessionDiesel>)) -> User {
        let (u, s) = t;
        User {
            id: u.id,
            username: u.username,
            password_hash: u.password_hash,
            session: s.map(|s| s.into()),
        }
    }
}

impl From<(UserDiesel, SessionDiesel)> for User {
    fn from(t: (UserDiesel, SessionDiesel)) -> User {
        User::from((t.0, Some(t.1)))
    }
}

pub async fn get_user_by_name(db: &Db, name: String) -> Result<Option<User>, Error> {
    let user: Option<(UserDiesel, Option<SessionDiesel>)> = db
        .run(move |c| {
            users::table
                .left_join(sessions::table)
                .filter(users::username.eq(name))
                .first(c)
                .optional()
        })
        .await?;
    Ok(user.map(|u| u.into()))
}

pub async fn get_user_by_session_id(db: &Db, session_id: String) -> Result<Option<User>, Error> {
    let user: Option<(UserDiesel, SessionDiesel)> = db
        .run(move |c| {
            users::table
                .inner_join(sessions::table)
                .filter(sessions::session_id.eq(session_id))
                .first(c)
                .optional()
        })
        .await?;
    Ok(user.map(|u| u.into()))
}

pub async fn create_session(
    db: &Db,
    user_id: i32,
    session_id: String,
    expires: NaiveDateTime,
) -> Result<String, Error> {
    let session = SessionDiesel {
        id: session_id,
        user_id,
        expires,
    };
    let session: SessionDiesel = db
        .run(move |c| {
            diesel::insert_into(sessions::table)
                .values(&session)
                .get_result(c)
        })
        .await?;
    Ok(session.id)
}

pub async fn renew_session(
    db: &Db,
    session_id: String,
    expires: NaiveDateTime,
) -> Result<(), Error> {
    db.run(move |c| {
        diesel::update(sessions::table.find(session_id))
            .set(sessions::expires.eq(expires))
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn delete_session(db: &Db, user_id: i32) -> Result<(), Error> {
    db.run(move |c| {
        diesel::delete(sessions::table.filter(sessions::user_id.eq(user_id))).execute(c)
    })
    .await?;
    Ok(())
}
