use chrono::NaiveDateTime;
use rocket_sync_db_pools::diesel;

use crate::Db;

use super::logic::{Role, Session, User};

use self::diesel::prelude::*;
use crate::db::schema::{roles, sessions, users};
use crate::db::sql_types;

pub type Error = diesel::result::Error;

#[derive(Debug, Identifiable, PartialEq, Queryable)]
#[table_name = "users"]
struct UserDiesel {
    id: i32,
    username: String,
    password_hash: String,
}

#[derive(Associations, Debug, Identifiable, PartialEq, Queryable)]
#[belongs_to(UserDiesel, foreign_key = "user_id")]
#[primary_key(user_id, role)]
#[table_name = "roles"]
struct RoleDiesel {
    user_id: i32,
    role: sql_types::RoleDb,
}

impl From<sql_types::RoleDb> for Role {
    fn from(r: sql_types::RoleDb) -> Role {
        match r {
            sql_types::RoleDb::Teacher => Self::Teacher,
            sql_types::RoleDb::Student => Self::Student,
        }
    }
}

#[derive(Associations, Debug, Identifiable, Insertable, PartialEq, Queryable)]
#[belongs_to(UserDiesel, foreign_key = "user_id")]
#[primary_key(session_id)]
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

impl From<(UserDiesel, Option<SessionDiesel>, Vec<RoleDiesel>)> for User {
    fn from(t: (UserDiesel, Option<SessionDiesel>, Vec<RoleDiesel>)) -> User {
        let (u, s, r) = t;
        User {
            id: u.id,
            username: u.username,
            password_hash: u.password_hash,
            session: s.map(|s| s.into()),
            roles: r.into_iter().map(|r| r.role.into()).collect(),
        }
    }
}

impl From<(UserDiesel, SessionDiesel, Vec<RoleDiesel>)> for User {
    fn from(t: (UserDiesel, SessionDiesel, Vec<RoleDiesel>)) -> User {
        User::from((t.0, Some(t.1), t.2))
    }
}

pub async fn get_user_by_name(db: &Db, name: String) -> Result<Option<User>, Error> {
    let user: Option<UserDiesel> = db
        .run(move |c| {
            users::table
                .filter(users::username.eq(name))
                .first(c)
                .optional()
        })
        .await?;
    match user {
        Some(user) => Ok(Some(complete_user(db, user).await?)),
        None => Ok(None),
    }
}

async fn complete_user(db: &Db, user: UserDiesel) -> Result<User, Error> {
    db.run(move |c| {
        let session = SessionDiesel::belonging_to(&user).first(c).optional()?;
        let roles = RoleDiesel::belonging_to(&user).load(c)?;
        Ok(User::from((user, session, roles)))
    })
    .await
}

pub async fn get_user_by_session_id(db: &Db, session_id: String) -> Result<Option<User>, Error> {
    let user_session: Option<(UserDiesel, SessionDiesel)> = db
        .run(move |c| {
            users::table
                .inner_join(sessions::table)
                .filter(sessions::session_id.eq(session_id))
                .first(c)
                .optional()
        })
        .await?;
    match user_session {
        Some((user, session)) => Ok(Some(complete_user_session(db, user, session).await?)),
        None => Ok(None),
    }
}

async fn complete_user_session(
    db: &Db,
    user: UserDiesel,
    session: SessionDiesel,
) -> Result<User, Error> {
    db.run(move |c| {
        let roles = RoleDiesel::belonging_to(&user).load(c)?;
        Ok(User::from((user, session, roles)))
    })
    .await
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
