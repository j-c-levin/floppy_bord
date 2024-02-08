use bevy::prelude::*;
use crate::gravity::Velocity;
use crate::state::GameState;

pub struct BackgroundPlugin;

#[derive(Component)]
struct Background;

const BACKGROUND_SPEED: f32 = -60.0;
const TILE_SPAWN_X: f32 = 800.0;

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
            )
            .add_systems(Update, tile_background.run_if(in_state(GameState::InGame)));
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
        Velocity { velocity: Vec2::from((BACKGROUND_SPEED, 0.0)) },
        Background,
        Name::new("background")
    ));
}

fn despawn_background(
    mut commands: Commands,
    query: Query<Entity, With<Background>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn tile_background(
    mut commands: Commands,
    query: Query<&GlobalTransform, With<Background>>,
    asset_server: Res<AssetServer>
) {
    let should_tile = query.iter().all(|&transform| {
        return transform.translation().x < 0.0
    });

    if should_tile {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(TILE_SPAWN_X, 0.0, -1.0)
                    .with_scale(Vec3::new(1.0, 1.7, 1.0)),
                texture: asset_server.load("background.png"),
                ..default()
            },
            Velocity { velocity: Vec2::from((BACKGROUND_SPEED, 0.0)) },
            Background,
            Name::new("background")
        ));
    }
}