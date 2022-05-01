#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod assets;
mod game;
mod menu;

const WINDOW_SIZE: (f32, f32) = (800.0, 600.0);
const ALLOW_EXIT: bool = cfg!(not(target_arch = "wasm32"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

fn main() {
    // When building for WASM, print panics to the browser console.
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
            title: "Pong!".into(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_loopless_state(AppState::MainMenu)
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin);

    if ALLOW_EXIT {
        app.add_system(bevy::input::system::exit_on_esc_system);
    }

    app.run();
}
