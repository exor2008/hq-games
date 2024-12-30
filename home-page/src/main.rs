use home_page::{index, register, states::ServersState};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index, register])
        .manage(ServersState::default())
        .attach(Template::fairing())
}
