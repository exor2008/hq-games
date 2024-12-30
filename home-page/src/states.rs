use std::net::Ipv4Addr;

use rocket::tokio::sync::Mutex;

#[derive(Default)]
pub struct ServersState {
    pub servers: Mutex<Games>,
}

#[derive(Default, Clone)]
pub struct Games {
    pub games: Vec<Game>,
}

#[derive(Clone)]
pub struct Game {
    pub name: String,
    pub address: Ipv4Addr,
}
