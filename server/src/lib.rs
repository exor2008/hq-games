use std::collections::HashMap;

use reqwest::Client;
use rocket::{
    fairing::{Fairing, Info, Kind},
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

        let mut payload = HashMap::new();
        payload.insert("address", rocket.config().address.to_string());
        payload.insert("name", "Vasya".to_string());

        rocket::info!("address: {}", payload["address"]);

        let res = client
            .post("http://127.0.0.1:8000/register")
            .json(&payload)
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
