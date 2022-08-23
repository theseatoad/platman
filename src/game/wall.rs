use bevy::prelude::*;
use super::components::Collider;

#[derive(Bundle)]
pub struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    pub fn new(location: Vec3, scale: Vec2) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: location.x,
                        y: location.y,
                        z: location.z,
                    },
                    scale: Vec3 {
                        x: scale.x,
                        y: scale.y,
                        z: 1.,
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.2),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}