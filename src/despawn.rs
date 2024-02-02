use bevy::prelude::*;
use crate::gravity::Gravity;
use crate::state::GameState;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                despawn_far_away_entities.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                despawn_all_entities,
            );
    }
}

const DESPAWN_DISTANCE: f32 = 850.0;

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<Gravity>>,
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(
    mut commands: Commands,
    query: Query<Entity, With<Gravity>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}