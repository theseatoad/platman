use bevy::prelude::*;

use super::{wall::WallBundle};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map);
    }
}


fn setup_map(mut commands: Commands){
    //Left wall
    commands.spawn_bundle(WallBundle::new(Vec3{x: -450., y: 0., z: 0.1}, Vec2 { x: 25., y: 700. }));
    //Right wall
    commands.spawn_bundle(WallBundle::new(Vec3{x: 450., y: 0., z: 0.1}, Vec2 { x: 25., y: 700. }));
    //Bottom Wall
    commands.spawn_bundle(WallBundle::new(Vec3{x: 0., y: -350., z: 0.1}, Vec2 { x: 925., y: 25. }));
    //Top Wall
    commands.spawn_bundle(WallBundle::new(Vec3{x: 0., y: 350., z: 0.1}, Vec2 { x: 925., y: 25. }));
}