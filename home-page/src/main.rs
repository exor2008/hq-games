use home_page::{
    api,
    db::Db,
    model::GamesState,
    routes::{admin, login, routes},
};
use rocket::fs::FileServer;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes())
        .mount("/", login::routes())
        .mount("/admin", admin::routes())
        .mount("/api", api::routes())
        .manage(GamesState::default())
        .attach(Template::fairing())
        .attach(Db::init())
}
