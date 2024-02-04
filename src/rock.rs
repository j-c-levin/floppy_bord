use bevy::prelude::*;
use crate::state::GameState;

const SPAWN_TIME_SECONDS: f32 = 2.0;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnTimer {
                timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating)
            })
            .add_systems(Update, spawn_rocks_on_timer.run_if(in_state(GameState::InGame)))
            .add_systems(OnEnter(GameState::GameOver), (despawn_rocks, reset_timer));
    }
}

#[derive(Component)]
pub struct Rock;

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}

fn spawn_rocks_on_timer(
    mut command: Commands,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    command.spawn((
        Rock,
        SpriteBundle{
            texture: asset_server.load("rockGrass.png"),
            ..default()
        }
    ));
}

fn reset_timer(
    mut timer: ResMut<SpawnTimer>
) {
    timer.timer.reset();
}

fn despawn_rocks(
    mut command: Commands,
    rocks: Query<Entity, With<Rock>>,
) {
    for entity in rocks.iter() {
        command.entity(entity).despawn_recursive();
    }
}