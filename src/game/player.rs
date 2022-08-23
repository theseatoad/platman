use super::components::{
    Collider, EntityName, Grounded, Player, PlayerInput, Velocity, Wall,
};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision}, time::Stopwatch,
};

pub const PLAYERSPEED: f32 = 250.;
pub const PLAYERJUMPSPEED: f32 = 20000.;
pub const PLAYERHELDSPEED : f32 = 100.;
pub const PLAYERFALLSPEED: f32 = 2500.;
pub const PLAYERMAXJUMPTIME : f32 = 0.175;
#[derive(Component)]
struct JumpHeld {
    time: Stopwatch,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(player_input)
            .add_system(
                check_collisions
                    .after(player_input))
            .add_system(
                move_player
                    .after(check_collisions));
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
    jump_held : JumpHeld
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
            jump_held : JumpHeld { time: bevy::time::Stopwatch::new() }
        }
    }
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(PlayerBundle::new(Vec2 { x: 10., y: 10. }))
        .insert(EntityName("Player".to_string()));
}

fn move_player(
    time: Res<Time>,
    mut player_query: Query<(&mut JumpHeld, &PlayerInput, &Grounded, &mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut player_jump_held, player_input, player_grounded, mut player_transform, mut player_velocity) =
        player_query.single_mut();

    player_velocity.0.x = player_input.0.x * PLAYERSPEED;

    println!("{:?}", player_jump_held.time.elapsed().as_secs_f32());
    if player_grounded.0 {
        if player_input.0.y > 0. {
            player_velocity.0.y = PLAYERJUMPSPEED * time.delta_seconds();
            player_jump_held.time.tick(time.delta());
        } else {
            player_velocity.0.y = 0.;
        }
    } else {
        if player_jump_held.time.elapsed().as_secs_f32() > 0. && player_jump_held.time.elapsed().as_secs_f32() < PLAYERMAXJUMPTIME {
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
    let mut player_input  = player_query.single_mut();
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

fn check_collisions(
    mut player_query: Query<(&mut PlayerInput, &mut Grounded, &Transform), With<Player>>,
    collider_query: Query<(Entity, &Transform, Option<&Wall>, Without<Player>), With<Collider>>,
) {
    let (mut player_input, mut player_grounded, player_transform) = player_query.single_mut();

    let mut collision_on_bottom = false;
    //Check collisions
    for (_, transform, _, _) in &collider_query {
        let collision = collide(
            transform.translation,
            transform.scale.truncate(),
            player_transform.translation,
            player_transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            match collision {
                Collision::Left => {
                    if player_input.0.x < 0. {
                        player_input.0.x = 0.;
                    }
                }
                Collision::Right => {
                    if player_input.0.x > 0. {
                        player_input.0.x = 0.;
                    }
                }
                Collision::Top => println!("Collision Top"),
                Collision::Bottom => {
                    collision_on_bottom = true;
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
