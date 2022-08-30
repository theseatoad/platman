use bevy::prelude::*;
use super::components::Collider;

#[derive(Bundle)]
pub struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    pub fn new(location: Vec3, wall_color: Color, wall_scale: Vec3) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: location.x,
                        y: location.y,
                        z: location.z,
                    },
                    scale: wall_scale,
                    ..default()
                },
                sprite: Sprite {
                    color: wall_color,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}