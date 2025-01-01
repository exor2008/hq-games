use rocket::serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Default, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Games(pub Vec<Game>);

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub name: String,
    pub address: IpAddr,
    pub port: u16,
}
