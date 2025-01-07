use bevy::prelude::*;
use wasm_bindgen::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(Text2d::new("Hello!"));
}

#[wasm_bindgen]
pub fn wasm_ffi(name: &str) {
    web_sys::console::log_1(&format!("Name is: {name}").into());
}
