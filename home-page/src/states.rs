use crate::model::Games;
use rocket::tokio::sync::Mutex;

#[derive(Default)]
pub struct GamesState(pub Mutex<Games>);
