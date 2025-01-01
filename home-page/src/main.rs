use home_page::{db::Db, index, register, states::ServersState, users};
use rocket::fs::FileServer;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index, register, users])
        .manage(ServersState::default())
        .attach(Template::fairing())
        .attach(Db::init())
}
