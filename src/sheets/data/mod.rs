use chrono::{DateTime, Utc};
use rocket_sync_db_pools::diesel;

use crate::login::transport::UserTransport;
use crate::Db;

use super::logic::{Id, Sheet, SheetMetadata};

use self::diesel::prelude::*;
use crate::db::model::{SheetDiesel, SheetMetadataDiesel, UserTransportDiesel};
use crate::db::schema::{sheets, users};

pub type Error = diesel::result::Error;

impl From<(SheetDiesel, UserTransportDiesel)> for Sheet {
    fn from(t: (SheetDiesel, UserTransportDiesel)) -> Sheet {
        let (s, u) = t;
        Sheet {
            metadata: SheetMetadata {
                id: s.id,
                title: s.title,
                owner: u.into(),
                created: s.created,
                changed: s.changed,
                trashed: s.trashed,
            },
            tiptap: s.tiptap,
        }
    }
}

impl From<UserTransportDiesel> for UserTransport {
    fn from(u: UserTransportDiesel) -> UserTransport {
        UserTransport {
            id: u.id,
            username: u.username,
        }
    }
}

impl From<(SheetMetadataDiesel, UserTransportDiesel)> for SheetMetadata {
    fn from(t: (SheetMetadataDiesel, UserTransportDiesel)) -> SheetMetadata {
        let (s, u) = t;
        SheetMetadata {
            id: s.id,
            title: s.title,
            owner: u.into(),
            created: s.created,
            changed: s.changed,
            trashed: s.trashed,
        }
    }
}

pub async fn get_all_sheets(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>, Error> {
    let sheets: Vec<(SheetMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            sheets::table
                .inner_join(users::table)
                .select((
                    SheetMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(sheets::owner_id.eq(user_id))
                .filter(sheets::trashed.is_null())
                .order(sheets::title.asc())
                .load(c)
        })
        .await?;
    Ok(sheets.into_iter().map(|s| s.into()).collect())
}

pub async fn get_trash(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>, Error> {
    let sheets: Vec<(SheetMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            sheets::table
                .inner_join(users::table)
                .select((
                    SheetMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(sheets::owner_id.eq(user_id))
                .filter(sheets::trashed.is_not_null())
                .order(sheets::trashed.desc())
                .load(c)
        })
        .await?;
    Ok(sheets.into_iter().map(|s| s.into()).collect())
}

pub async fn get_recent(db: &Db, user_id: i32) -> Result<Vec<SheetMetadata>, Error> {
    let sheets: Vec<(SheetMetadataDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            sheets::table
                .inner_join(users::table)
                .select((
                    SheetMetadataDiesel::columns(),
                    UserTransportDiesel::columns(),
                ))
                .filter(sheets::owner_id.eq(user_id))
                .filter(sheets::trashed.is_null())
                .order(sheets::changed.desc())
                .load(c)
        })
        .await?;
    Ok(sheets.into_iter().map(|s| s.into()).collect())
}

pub async fn get_sheet_by_id(db: &Db, id: Id) -> Result<Option<Sheet>, Error> {
    let sheet: Option<(SheetDiesel, UserTransportDiesel)> = db
        .run(move |c| {
            sheets::table
                .inner_join(users::table)
                .select((sheets::all_columns, UserTransportDiesel::columns()))
                .filter(sheets::id.eq(id))
                .first(c)
                .optional()
        })
        .await?;
    Ok(sheet.map(|s| s.into()))
}

pub async fn create_sheet(
    db: &Db,
    title: String,
    tiptap: serde_json::Value,
    owner_id: i32,
    created: DateTime<Utc>,
    changed: DateTime<Utc>,
    trashed: Option<DateTime<Utc>>,
) -> Result<Id, Error> {
    let sheet: SheetDiesel = db
        .run(move |c| {
            diesel::insert_into(sheets::table)
                .values(&(
                    sheets::title.eq(title),
                    sheets::owner_id.eq(owner_id),
                    sheets::created.eq(created),
                    sheets::changed.eq(changed),
                    sheets::tiptap.eq(tiptap),
                    sheets::trashed.eq(trashed),
                ))
                .get_result(c)
        })
        .await?;
    Ok(sheet.id)
}

pub async fn update_sheet(
    db: &Db,
    id: Id,
    title: String,
    tiptap: serde_json::Value,
) -> Result<(), Error> {
    db.run(move |c| {
        diesel::update(sheets::table.find(id))
            .set((sheets::title.eq(title), sheets::tiptap.eq(tiptap)))
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn delete_sheet(db: &Db, id: Id) -> Result<(), Error> {
    db.run(move |c| diesel::delete(sheets::table.find(id)).execute(c))
        .await?;
    Ok(())
}

pub async fn move_sheet_to_trash(db: &Db, id: Id) -> Result<(), Error> {
    db.run(move |c| {
        diesel::update(sheets::table.find(id))
            .set(sheets::trashed.eq(Some(Utc::now())))
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn restore_sheet(db: &Db, id: Id) -> Result<(), Error> {
    db.run(move |c| {
        diesel::update(sheets::table.find(id))
            .set(sheets::trashed.eq(None::<DateTime<Utc>>))
            .execute(c)
    })
    .await?;
    Ok(())
}
