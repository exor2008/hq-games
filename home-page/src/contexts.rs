use std::net::Ipv4Addr;

use crate::states::{Game, Games};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IndexContext {
    pub servers: GamesContext,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GamesContext {
    games: Vec<GameContext>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GameContext {
    name: String,
    address: Ipv4Addr,
}

impl From<Games> for GamesContext {
    fn from(value: Games) -> Self {
        Self {
            games: value.games.into_iter().map(|game| game.into()).collect(),
        }
    }
}

impl From<Game> for GameContext {
    fn from(value: Game) -> Self {
        Self {
            name: value.name,
            address: value.address,
        }
    }
}
