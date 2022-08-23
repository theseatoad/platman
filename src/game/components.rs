use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Component, Default, Clone)]
pub struct Wall;

#[derive(Component, Default, Clone)]
pub struct Position(pub Vec2);

#[derive(Component, Default, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct EntityName(pub String);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct Jump(pub f32);

#[derive(Component)]
pub struct PlayerInput(pub Vec2);