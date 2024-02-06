use bevy::prelude::*;
use crate::gravity::Velocity;
use crate::state::GameState;
use rand::Rng;

pub struct RockPlugin;

#[derive(Component)]
pub struct Rock;

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}
const SPAWN_TIME_SECONDS: f32 = 3.0;
const ROCK_X: f32 = 300.0;
const ROCK_Y_HIGH: f32 = -94.0;
const ROCK_Y_LOW: f32 = -650.0;
const ROCK_SPEED: f32 = -120.0;
const ROCK_Y_SCALE: f32 = 2.6;
const ROCK_GAP: f32 = 770.0;

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

fn spawn_rocks_on_timer(
    mut command: Commands,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    let handle = asset_server.load("rockGrass.png");

    let mut rng = rand::thread_rng();
    let random_y = rng.gen_range(ROCK_Y_LOW..ROCK_Y_HIGH);

    // spawn bottom rock
    command.spawn(rock_bundle(true, handle.clone(), random_y));

    // spawn top rock
    command.spawn(rock_bundle(false, handle.clone(), random_y));
}

fn rock_bundle(bottom: bool, texture: Handle<Image>, random_y: f32) -> (Rock, Velocity, SpriteBundle, Name) {
    let position = if bottom { random_y } else { random_y + ROCK_GAP };
    let name = if bottom { "bottom_rock" } else { "top_rock" };

    (
        Rock,
        Velocity { velocity: Vec2::from((ROCK_SPEED, 0.0)) },
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(ROCK_X, position, 0.0),
                scale: Vec3::new(1.0, ROCK_Y_SCALE, 1.0),
                ..default()
            },
            texture,
            sprite: Sprite {
                flip_y: !bottom,
                ..default()
            },
            ..default()
        },
        Name::new(name)
    )
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