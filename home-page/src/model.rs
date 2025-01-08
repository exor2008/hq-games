use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::Request;
use sqlx::types::chrono::{DateTime, Local};
use sqlx::types::Uuid;
use sqlx::FromRow;
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

#[derive(Default)]
pub struct GamesState(pub Mutex<Games>);

#[derive(Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct DbUser {
    pub id: Uuid,
    pub name: String,
    pub hashed_password: String,
    pub role: String,
    pub created: DateTime<Local>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User(pub Uuid);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| Uuid::parse_str(cookie.value()).ok())
            .map(User)
            .or_forward(Status::Unauthorized)
    }
}

#[derive(FromForm)]
pub struct Login<'r> {
    pub password: &'r str,
    pub name: &'r str,
}
