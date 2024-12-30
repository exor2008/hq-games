#[macro_use]
extern crate rocket;

use contexts::IndexContext;
use rocket::State;
use rocket_dyn_templates::Template;
use states::ServersState;

pub mod contexts;
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
