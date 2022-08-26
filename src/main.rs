use bevy::prelude::*;
use game::{player::PlayerPlugin, map::MapPlugin};
use gameover::GameOverPlugin;
use mainmenu::MainMenuPlugin;
use types::GameState;
mod game;
mod mainmenu;
mod types;
mod gameover;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Platman".to_string(),
            width: 960.,
            height: 768.,
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_state(GameState::MainMenu)
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands){
    commands.spawn_bundle(Camera2dBundle::default());
}