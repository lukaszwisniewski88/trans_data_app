use diesel::PgConnection;
use rocket_sync_db_pools::database;

pub mod session;
pub mod users;

#[database("trans_api")]
pub struct Postgres(PgConnection);
