#[macro_use]
extern crate rocket;

use contexts::IndexContext;
use input::Register;
use rocket::{serde::json::Json, State};
use rocket_dyn_templates::Template;
use states::{Game, ServersState};

pub mod contexts;
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
