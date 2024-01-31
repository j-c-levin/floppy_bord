use bevy::prelude::*;
use crate::{AnimationIndices, AnimationTimer};
use crate::gravity::{Gravity, Velocity};
use crate::input::Jump;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, animate_sprite);
    }
}

const BIRD_SIZE: f32 = 16.0;
const BIRD_ATLAS_COLUMNS: usize = 4;
const BIRD_ATLAS_ROWS: usize = 1;
const BIRD_ANIMATION_SPEED: f32 = 0.1;
const BIRD_GRAVITY: Vec2 = Vec2::new(0.0, -3000.0);

const JUMP_SPEED: f32 = 800.0;

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
        Gravity::new(BIRD_GRAVITY),
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