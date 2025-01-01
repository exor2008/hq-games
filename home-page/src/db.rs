use rocket::serde::Serialize;
use rocket_db_pools::{sqlx::SqlitePool, Database};
use sqlx::types::chrono::{DateTime, Local};
use sqlx::FromRow;

#[derive(Database)]
#[database("database")]
pub struct Db(SqlitePool);

#[derive(Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    name: String,
    email: String,
    hashed_password: String,
    role: String,
    created: DateTime<Local>,
}
