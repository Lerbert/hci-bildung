use rocket_sync_db_pools::diesel;

use super::logic;

pub mod sheet;

pub type Error = diesel::result::Error;
