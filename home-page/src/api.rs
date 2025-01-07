use crate::model::{Game, GamesState};
use rocket::{serde::json::Json, Route, State};

#[post("/register", data = "<game_request>", format = "json")]
pub async fn register(game_request: Json<Game>, games_state: &State<GamesState>) {
    let games = &mut *games_state.0.lock().await;

    games.0.push(game_request.0);
}

pub fn routes() -> Vec<Route> {
    routes![register]
}
