use log::error;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

#[database("hci_bildung")]
pub struct Db(diesel::PgConnection);

embed_migrations!();

pub async fn migrate(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    if let Some(db) = Db::get_one(&rocket).await {
        db.run(|c| match embedded_migrations::run(c) {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Database migration failed: {}", e);
                Err(rocket)
            }
        })
        .await
    } else {
        Err(rocket)
    }
}
