use rocket::serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Register {
    pub name: String,
    pub address: Ipv4Addr,
}
