use std::fs;

use bevy::prelude::*;

use crate::types::GameState;

use super::{wall::WallBundle, player::OnlyInGame};

pub struct MapPlugin;

pub const MAPWIDTH: f32 = 35.;
pub const TILESIZE: f32 = 25.;
pub const XOFFSET: f32 = -430.;
pub const YOFFSET: f32 = -362.;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_map));
    }
}

fn setup_map(mut commands: Commands) {
    let map_string = fs::read_to_string("assets/maps/map1.txt").expect("Could not read map");

    let mut x = 0.;
    let mut y = 0.;
    for mut tile in map_string.split(",") {
        tile = tile.trim_matches('\n');
        if !tile.is_empty() {
            match tile {
                "1" => {
                    commands.spawn_bundle(WallBundle::new(Vec3 {
                        x: (x * TILESIZE) + XOFFSET,
                        y: (y * TILESIZE) + YOFFSET,
                        z: 0.1,
                    })).insert(OnlyInGame);
                }
                _ => { /* Nothing */ }
            }
            if x == MAPWIDTH - 1. {
                x = 0.;
                y += 1.;
            } else {
                x += 1.
            }
        }
    }
}
