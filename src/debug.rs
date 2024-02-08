use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::math::Vector;
use crate::gravity::Velocity;
use crate::rock::Rock;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug);
    }
}

fn spawn_debug(
    mut command: Commands,
    asset_server: Res<AssetServer>
) {
    command.spawn((
        Rock,
        Velocity { velocity: Vec2::ZERO },
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(100.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 2.6, 1.0),
                ..default()
            },
            texture: asset_server.load("rockGrass.png"),
            sprite: Sprite {
                flip_y: true,
                ..default()
            },
            ..default()
        },
        Name::new("test_rock"),
        Collider::triangle(Vector::new(55.0, 118.0), Vector::new(-55.0, 118.0), Vector::new(13.0, -120.0))
    ));
}
// Vector::new(-55.0, -118.0), Vector::new(55.0, -118.0), Vector::new(13.0, 120.0) - bottom
// Vector::new(55.0, 118.0), Vector::new(-55.0, 118.0), Vector::new(13.0, -120.0) - top