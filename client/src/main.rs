use std::sync::{Arc, LazyLock, Mutex};

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

static TEXT: LazyLock<Arc<Mutex<String>>> = LazyLock::new(|| Arc::new(Mutex::new(String::new())));

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup)
        .add_systems(Update, update_text);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(Text2d::new("Hello!"));
}

fn update_text(mut query: Query<&mut Text2d>) {
    let mut text = query.single_mut();
    text.0 = TEXT.lock().and_then(|t| Ok((*t).clone())).unwrap();
}

#[wasm_bindgen]
pub fn send(user: &str, _address: &str, _port: &str) {
    TEXT.lock()
        .and_then(|mut text| Ok(*text = user.to_string()))
        .unwrap();
    web_sys::console::log_1(&format!("Name is: {user}").into());
}
