use bevy::prelude::*;
use game::{player::PlayerPlugin, map::MapPlugin};
use bevy_inspector_egui::{WorldInspectorPlugin};

mod game;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Platman".to_string(),
            width: 960.,
            height: 768.,
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        })
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands){
    commands.spawn_bundle(Camera2dBundle::default());
}