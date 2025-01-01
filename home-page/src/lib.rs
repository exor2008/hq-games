#[macro_use]
extern crate rocket;

use contexts::IndexContext;
use db::{Db, User};
use input::Register;
use rocket::{serde::json::Json, State};
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use states::{Game, ServersState};

pub mod contexts;
pub mod db;
pub mod input;
pub mod states;

#[get("/")]
pub async fn index(servers: &State<ServersState>) -> Template {
    let games = (*servers.servers.lock().await).clone();

    Template::render(
        "index",
        IndexContext {
            servers: games.into(),
        },
    )
}

#[post("/register", data = "<reg>", format = "json")]
pub async fn register(reg: Json<Register>, servers: &State<ServersState>) {
    let mut servers = servers.servers.lock().await;
    (*servers).games.push(Game {
        name: reg.name.clone(),
        address: reg.address,
    });
}

#[get("/users")]
pub async fn users(mut db: Connection<Db>) -> Template {
    let users: Vec<User> = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&mut **db)
        .await
        .unwrap();

    Template::render("users", context! {users: users})
}
