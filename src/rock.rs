use std::time::Duration;
use bevy::prelude::*;
use bevy_xpbd_2d::math::Vector;
use crate::gravity::Velocity;
use crate::state::GameState;
use rand::Rng;
use bevy_xpbd_2d::prelude::*;
use crate::collision_layers::Layer;

pub struct RockPlugin;

#[derive(Component)]
pub struct Rock;

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Score;

const SPAWN_TIME_SECONDS: f32 = 2.8;
const ROCK_X: f32 = 280.0;
const ROCK_Y_HIGH: f32 = -160.0;
const ROCK_Y_LOW: f32 = -566.0;
const ROCK_SPEED: f32 = -130.0;
const ROCK_Y_SCALE: f32 = 2.6;
const ROCK_GAP: f32 = 770.0;
const ROCK_DESPAWN_X: f32 = -400.0;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnTimer {
                timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating)
            })
            .add_systems(OnEnter(GameState::InGame), start_spawn)
            .add_systems(Update, (spawn_rocks_on_timer, lost_rock).run_if(in_state(GameState::InGame)))
            .add_systems(OnEnter(GameState::GameOver), (despawn_rocks, despawn_score_colliders, reset_timer));
    }
}

fn start_spawn(
    mut timer: ResMut<SpawnTimer>
) {
    timer.timer.tick(Duration::from_secs((SPAWN_TIME_SECONDS - 0.1) as u64));
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

    // spawn score collider
    command.spawn((
        Collider::cuboid(2.0, 800.0),
        Velocity {
            velocity: Vec2::from((ROCK_SPEED, 0.0))
        },
        Transform {
            translation: Vec3::new(ROCK_X + 12.0, 0.0, 0.0),
            ..default()
        },
        GlobalTransform::default(),
        Score,
        CollisionLayers::new([Layer::Score], [Layer::Bird]),
    ));
}

fn rock_bundle(bottom: bool, texture: Handle<Image>, random_y: f32) -> (Rock, Velocity, SpriteBundle, Name, Collider, CollisionLayers) {
    let position = if bottom { random_y } else { random_y + ROCK_GAP };
    let name = if bottom { "bottom_rock" } else { "top_rock" };
    let sign = if bottom { 1.0 } else { -1.0 };

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
        Name::new(name),
        Collider::triangle(Vector::new(-55.0 * sign, -118.0 * sign), Vector::new(55.0 * sign, -118.0 * sign), Vector::new(13.0, 120.0 * sign)),
        CollisionLayers::new([Layer::Rock], [Layer::Bird]),
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

fn despawn_score_colliders(
    mut command: Commands,
    rocks: Query<Entity, With<Score>>,
) {
    for entity in rocks.iter() {
        command.entity(entity).despawn_recursive();
    }
}

fn lost_rock(
    mut commands: Commands,
    rocks: Query<(&GlobalTransform, Entity), With<Rock>>,
) {
    for (transform, entity) in rocks.iter() {
        if transform.translation().x < ROCK_DESPAWN_X {
            commands.entity(entity).despawn_recursive();
        }
    }
}
