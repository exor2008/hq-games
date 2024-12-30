use home_page::{index, states::ServersState};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index])
        .manage(ServersState::default())
        .attach(Template::fairing())
}
