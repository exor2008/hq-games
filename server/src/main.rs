use server::Register;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Register)
}
