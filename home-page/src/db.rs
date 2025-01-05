use rocket_db_pools::{sqlx::SqlitePool, Database};

#[derive(Database)]
#[database("database")]
pub struct Db(SqlitePool);
