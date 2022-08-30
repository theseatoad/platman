use std::fs;

use bevy::prelude::*;

use crate::types::GameState;

use super::{player::OnlyInGame, wall::WallBundle, player::Scoreboard};

pub struct MapPlugin;

#[derive(PartialEq)]
pub enum TileType {
    Wall,
    Teleporter,
    Coin
}
#[derive(Component)]
pub struct Tile(pub TileType);

pub const MAPWIDTH: f32 = 41.;
pub const TILESIZE: f32 = 25.;
pub const XOFFSET: f32 = -500.;
pub const YOFFSET: f32 = -362.;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_map));
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {

        // Scoreboard
        commands.spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: asset_server.load("alagard.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("alagard.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(1.0, 0.5, 0.5),
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }),
        ).insert(Scoreboard{score: 0});


    let map_string = fs::read_to_string("assets/maps/map1.txt").expect("Could not read map");

    let mut x = 0.;
    let mut y = 0.;
    for mut tile in map_string.split(",") {
        tile = tile.trim_matches('\n');
        if !tile.is_empty() {
            match tile {
                "1" => {
                    commands
                        .spawn_bundle(WallBundle::new(
                            Vec3 {
                                x: (x * TILESIZE) + XOFFSET,
                                y: (y * TILESIZE) + YOFFSET,
                                z: 0.1,
                            },
                            Color::rgb(115. / 255., 23. / 255., 45. / 255.),
                            Vec3 { x: 25., y: 25., z: 1. }
                        ))
                        .insert(OnlyInGame)
                        .insert(Tile(TileType::Wall));
                }
                "2" => {
                    commands
                        .spawn_bundle(WallBundle::new(
                            Vec3 {
                                x: (x * TILESIZE) + XOFFSET,
                                y: (y * TILESIZE) + YOFFSET,
                                z: 0.1,
                            },
                            Color::rgb(0. / 255., 0. / 255., 139. / 255.),
                            Vec3 { x: 25., y: 25., z: 1. }
                        ))
                        .insert(Tile(TileType::Teleporter))
                        .insert(OnlyInGame);
                }
                "3" => {
                    commands
                        .spawn_bundle(WallBundle::new(
                            Vec3 {
                                x: (x * TILESIZE) + XOFFSET,
                                y: (y * TILESIZE) + YOFFSET,
                                z: 0.1,
                            },
                            Color::rgb(255. / 255., 238. / 255., 0. / 255.),
                            Vec3 { x: 10., y: 10., z: 1. }
                        ))
                        .insert(Tile(TileType::Coin))
                        .insert(OnlyInGame);
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
