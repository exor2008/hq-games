use crate::model::{GamesState, User};
use rocket::request::FlashMessage;
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};

pub mod admin;
pub mod login;

#[get("/")]
pub async fn index(
    user: User,
    games_state: &State<GamesState>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let games = &(*games_state.0.lock().await);

    Template::render(
        "index",
        context! {
            user: user,
            message: flash,
            games: &games.0,
        },
    )
}

#[get("/", rank = 2)]
pub async fn no_auth_index(
    games_state: &State<GamesState>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let games = &(*games_state.0.lock().await);

    Template::render(
        "index",
        context! {
            message: flash,
            games: &games.0
        },
    )
}

pub fn routes() -> Vec<Route> {
    routes![index, no_auth_index]
}
