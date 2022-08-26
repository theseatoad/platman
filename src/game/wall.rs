use bevy::prelude::*;
use super::components::Collider;

#[derive(Bundle)]
pub struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    pub fn new(location: Vec3) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: location.x,
                        y: location.y,
                        z: location.z,
                    },
                    scale: Vec3 {
                        x: 25.,
                        y: 25.,
                        z: 1.,
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(115./255., 23./255., 45./255.),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}