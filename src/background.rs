use bevy::prelude::*;
use crate::gravity::Velocity;
use crate::state::GameState;

pub struct BackgroundPlugin;

#[derive(Component)]
struct Background;

const BACKGROUND_SPEED: f32 = -80.0;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::GameOver),
                         (
                             despawn_background,
                             setup
                         )
                             .chain(),
            );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -1.0)
                .with_scale(Vec3::new(1.0, 1.7, 1.0)),
            texture: asset_server.load("background.png"),
            ..default()
        },
        Velocity { velocity: Vec2::from((BACKGROUND_SPEED, 0.0)) }
    ));
}

fn despawn_background(
    mut commands: Commands,
    query: Query<Entity, With<Background>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}