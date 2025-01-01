#[macro_use]
extern crate rocket;

use db::{Db, User};
use model::Game;
use rocket::{serde::json::Json, State};
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use states::GamesState;

pub mod db;
pub mod model;
pub mod states;

#[get("/")]
pub async fn index(games_state: &State<GamesState>) -> Template {
    let games = &(*games_state.0.lock().await);

    Template::render(
        "index",
        context! {
            games: &games.0,
        },
    )
}

#[post("/register", data = "<game_request>", format = "json")]
pub async fn register(game_request: Json<Game>, games_state: &State<GamesState>) {
    let games = &mut *games_state.0.lock().await;

    games.0.push(game_request.0);
}

#[get("/users")]
pub async fn users(mut db: Connection<Db>) -> Template {
    let users: Vec<User> = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&mut **db)
        .await
        .unwrap();

    Template::render("users", context! {users: users})
}
