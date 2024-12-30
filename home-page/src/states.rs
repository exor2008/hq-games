use rocket::tokio::sync::Mutex;

#[derive(Default)]
pub struct ServersState {
    pub servers: Mutex<Games>,
}

#[derive(Default, Clone)]
pub struct Games {
    pub games: Vec<Game>,
}

#[derive(Default, Clone)]
pub struct Game {
    pub name: String,
}
