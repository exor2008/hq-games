use std::net::IpAddr;

use reqwest::Client;
use rocket::{
    fairing::{Fairing, Info, Kind},
    serde::{Deserialize, Serialize},
    Orbit, Rocket,
};

pub struct Register;

#[rocket::async_trait]
impl Fairing for Register {
    fn info(&self) -> Info {
        Info {
            name: "Server regestration",
            kind: Kind::Liftoff,
        }
    }
    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let client = Client::new();

        let game = Game {
            name: "Game1".to_string(),
            address: rocket.config().address,
            port: rocket.config().port,
        };

        rocket::info!("address: {}", game.address);

        let res = client
            .post("http://127.0.0.1:8000/api/register")
            .json(&game)
            .send()
            .await;

        res.expect("Unable to register a server");
    }

    async fn on_shutdown(&self, _rocket: &Rocket<Orbit>) {
        let client = Client::new();

        let res = client.delete("http://10.0.0.1:8080/register").send().await;

        res.expect("Unable to register a server");
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Game {
    name: String,
    address: IpAddr,
    port: u16,
}
