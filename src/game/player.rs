use crate::types::GameState;

use super::{
    components::{Collider, EntityName, Grounded, Player, PlayerInput, Velocity},
    map::{Tile, TileType},
};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::Stopwatch,
};

pub const PLAYERSPEED: f32 = 250.;
pub const PLAYERJUMPSPEED: f32 = 20000.;
pub const PLAYERHELDSPEED: f32 = 100.;
pub const PLAYERFALLSPEED: f32 = 2500.;
pub const PLAYERMAXJUMPTIME: f32 = 0.175;
#[derive(Component)]
struct JumpHeld {
    time: Stopwatch,
}
pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct OnlyInGame;

#[derive(Component)]
pub struct Scoreboard {
    pub score: usize,
}
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_player))
            .insert_resource(Scoreboard { score: 0})
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(player_input))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(update_scoreboard))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(check_collisions_with_wall)
                    .after(player_input),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(move_player)
                    .after(check_collisions_with_wall),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(outofbounds))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup));
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    player: Player,
    grounded: Grounded,
    input: PlayerInput,
    jump_held: JumpHeld,
}

impl PlayerBundle {
    fn new(location: Vec2) -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: location.x,
                        y: location.y,
                        z: 0.,
                    },
                    scale: Vec3 {
                        x: 25.,
                        y: 25.,
                        z: 1.,
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 0.8),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity(Vec2::ZERO),
            player: Player,
            grounded: Grounded(false),
            input: PlayerInput(Vec2::ZERO),
            jump_held: JumpHeld {
                time: bevy::time::Stopwatch::new(),
            },
        }
    }
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(PlayerBundle::new(Vec2 { x: -15., y: -300. }))
        .insert(EntityName("Player".to_string()))
        .insert(OnlyInGame);
}

fn move_player(
    time: Res<Time>,
    mut player_query: Query<
        (
            &mut JumpHeld,
            &PlayerInput,
            &Grounded,
            &mut Transform,
            &mut Velocity,
        ),
        With<Player>,
    >,
) {
    let (
        mut player_jump_held,
        player_input,
        player_grounded,
        mut player_transform,
        mut player_velocity,
    ) = player_query.single_mut();

    player_velocity.0.x = player_input.0.x * PLAYERSPEED;

    if player_grounded.0 {
        if player_input.0.y > 0. {
            player_velocity.0.y = PLAYERJUMPSPEED * time.delta_seconds();
            player_jump_held.time.tick(time.delta());
        } else {
            player_velocity.0.y = 0.;
        }
    } else {
        if player_jump_held.time.elapsed().as_secs_f32() > 0.
            && player_jump_held.time.elapsed().as_secs_f32() < PLAYERMAXJUMPTIME
        {
            if player_input.0.y > 0. {
                player_velocity.0.y += PLAYERHELDSPEED * time.delta_seconds();
                player_jump_held.time.tick(time.delta());
            } else {
                player_jump_held.time.reset();
            }
        } else {
            player_velocity.0.y -= PLAYERFALLSPEED * time.delta_seconds();
            player_jump_held.time.reset();
        }
    }

    // Move player by velocity.
    player_transform.translation.x += player_velocity.0.x * time.delta_seconds();
    player_transform.translation.y += player_velocity.0.y * time.delta_seconds();
}

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut PlayerInput, With<Player>>,
) {
    let mut player_input = player_query.single_mut();
    //Start with zero
    player_input.0 = Vec2::ZERO;
    //Get input for left right
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        player_input.0.x = 1.;
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        player_input.0.x = -1.;
    }
    //Trying to jump
    if keyboard_input.pressed(KeyCode::Space) {
        player_input.0.y = 1.;
    }
}

fn check_collisions_with_wall(
    mut commands: Commands,
    mut player_query: Query<
        (
            &mut PlayerInput,
            &mut Grounded,
            &mut Transform,
            &mut Velocity,
        ),
        With<Player>,
    >,
    collider_query: Query<(Entity, &Transform, &Tile, Without<Player>), With<Collider>>,
    mut scoreboard: ResMut<Scoreboard>

) {
    let (mut player_input, mut player_grounded, mut player_transform, mut player_velocity) =
        player_query.single_mut();

    let mut collision_on_bottom = false;
    //Check collisions
    for (tile_entity, transform, tile, _) in &collider_query {
        let collision = collide(
            transform.translation,
            transform.scale.truncate(),
            player_transform.translation,
            player_transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            match collision {
                Collision::Left => {
                    if tile.0 == TileType::Wall {
                        if player_input.0.x < 0. {
                            player_input.0.x = 0.;
                        }
                    } else if tile.0 == TileType::Teleporter {
                        player_transform.translation.x = 460.;
                    } else if tile.0 == TileType::Coin {
                    }
                }
                Collision::Right => {
                    if tile.0 == TileType::Wall {
                        if player_input.0.x > 0. {
                            player_input.0.x = 0.;
                        }
                    } else if tile.0 == TileType::Teleporter {
                        player_transform.translation.x = -460.;
                    } else if tile.0 == TileType::Coin {
                    }
                }
                Collision::Top => {
                    if tile.0 == TileType::Wall {
                        player_velocity.0.y = 0.;
                        player_input.0.y = 0.;
                    } else if tile.0 == TileType::Coin {
                    }
                }
                Collision::Bottom => {
                    if tile.0 == TileType::Wall {
                        collision_on_bottom = true;
                        // Bump the player up to the top of the collider to avoid sinking into it slightly.
                        let wall_y = transform.translation.y + (transform.scale.truncate().y / 2.);
                        player_transform.translation.y =
                            wall_y + (player_transform.scale.truncate().y / 2.);
                    } else if tile.0 == TileType::Coin {
                        scoreboard.score += 1;
                        commands.entity(tile_entity).despawn_recursive();
                    }
                }
                Collision::Inside => { /* do nothing */ }
            }
        }
    }
    // Set grounded variable
    if collision_on_bottom {
        player_grounded.0 = true;
    } else {
        player_grounded.0 = false;
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>,mut game_state: ResMut<State<GameState>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
    if scoreboard.score >= 40 {
        game_state.set(GameState::GameOver).unwrap();
    }
}

fn outofbounds(query: Query<&Transform, With<Player>>, mut game_state: ResMut<State<GameState>>) {
    let player_transform = query.single();
    if player_transform.translation.y < -450. {
        game_state.set(GameState::GameOver).unwrap();
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<OnlyInGame>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
