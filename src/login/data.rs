use chrono::NaiveDateTime;
use rocket_sync_db_pools::postgres::{self, Row};

use crate::Db;

use super::logic::{Session, User};

pub type Error = postgres::Error;

pub async fn get_user_by_name(db: &Db, name: String) -> Result<Option<User>, Error> {
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
        let session = session_id.map(|_| session_from_row(&r));
        user_from_row(&r, session)
    }))
}

pub async fn get_user_by_session_id(db: &Db, session_id: String) -> Result<Option<User>, Error> {
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
    Ok(row.map(|r| user_from_row(&r, Some(session_from_row(&r)))))
}

pub async fn create_session(
    db: &Db,
    user_id: i32,
    session_id: String,
    expires: NaiveDateTime,
) -> Result<String, Error> {
    let row = db
        .run(move |c| {
            c.query_one(
                "insert into sessions(session_id, user_id, expires)
                values ($1, $2, $3)
                returning session_id",
                &[&session_id, &user_id, &expires],
            )
        })
        .await?;
    Ok(row.get("session_id"))
}

pub async fn renew_session(
    db: &Db,
    session_id: String,
    expires: NaiveDateTime,
) -> Result<(), Error> {
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

pub async fn delete_session(db: &Db, user_id: i32) -> Result<(), Error> {
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

fn user_from_row(row: &Row, session: Option<Session>) -> User {
    User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        session,
    }
}

fn session_from_row(row: &Row) -> Session {
    Session {
        id: row.get("session_id"),
        expires: row.get("expires"),
    }
}
