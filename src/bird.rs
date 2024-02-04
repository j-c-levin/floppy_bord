use std::usize;
use bevy::prelude::*;
use crate::gravity::{Gravity, Velocity};
use crate::input::Jump;
use crate::state::GameState;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::GameOver), setup)
            .add_systems(
                Update,
                (
                    lost_bird,
                    (animate_sprite, rotate_bird)
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct AnimationTimer(Timer);

const BIRD_SIZE: f32 = 16.0;
const BIRD_ATLAS_COLUMNS: usize = 4;
const BIRD_ATLAS_ROWS: usize = 1;
const BIRD_ANIMATION_SPEED: f32 = 0.1;
const BIRD_GRAVITY: f32 = -3000.0;
const JUMP_SPEED: f32 = 800.0;
const ROTATE_UP_ANGLE: f32 = 30.0;
const ROTATE_DOWN_ANGLE: f32 = -90.0;
const ROTATE_DOWN_THRESHOLD: f32 = -500.0;
const ROTATE_UP_SPEED: f32 = 500.0;
const ROTATE_DOWN_SPEED: f32 = -400.0;

const BIRD_DESPAWN_DISTANCE: f32 = 850.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bird.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::splat(BIRD_SIZE),
        BIRD_ATLAS_COLUMNS,
        BIRD_ATLAS_ROWS,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(animation_indices.first),
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(BIRD_ANIMATION_SPEED, TimerMode::Repeating)),
        Gravity::new(Vec2::new(0.0, BIRD_GRAVITY)),
        Velocity::new(Vec2::ZERO),
        Jump::new(JUMP_SPEED)
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn rotate_bird(
    mut bird: Query<(&mut Transform, &Velocity), With<Gravity>>,
    time: Res<Time>,
) {
    let Ok((mut transform, velocity)) = bird.get_single_mut() else {
        println!("rotate_bird: couldn't find bird");
        return;
    };

    let bird_angle = transform.rotation.to_euler(EulerRot::XYZ).2.to_degrees();
    let is_rising = velocity.velocity.y > ROTATE_DOWN_THRESHOLD;
    let (mut rotation_amount, rotation_max) = if is_rising {
        (f32::to_radians(ROTATE_UP_SPEED), ROTATE_UP_ANGLE)
    } else {
        (f32::to_radians(ROTATE_DOWN_SPEED), ROTATE_DOWN_ANGLE)
    };

    rotation_amount *= time.delta_seconds();

    let smoothly_rotate = if is_rising {
        bird_angle < rotation_max
    } else {
        bird_angle > rotation_max
    };

    // smoothly rotate or snap to max
    if smoothly_rotate {
        transform.rotate_z(rotation_amount);
    } else {
        transform.rotation = Quat::from_rotation_z(f32::to_radians(rotation_max));
    }
}

fn lost_bird(
    bird: Query<&GlobalTransform, With<Gravity>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(transform) = bird.get_single() else {
        println!("lost_bird: could not find bird!");
        return;
    };
    let distance = transform.translation().distance(Vec3::ZERO);

    if distance > BIRD_DESPAWN_DISTANCE {
        next_state.set(GameState::GameOver);
    }
}