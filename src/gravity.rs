use bevy::prelude::*;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}

#[derive(Component)]
pub struct Gravity {
    pub acceleration: Vec2,
}

impl Gravity {
    pub fn new(acceleration: Vec2) -> Self {
        Self { acceleration }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

impl Velocity {
    pub fn new(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

fn apply_gravity(
    mut bird: Query<(&Gravity, &mut Velocity)>,
    time: Res<Time>
) {
    let Ok((gravity, mut velocity)) = bird.get_single_mut() else {
        return;
    };

    velocity.velocity += gravity.acceleration * time.delta_seconds();
}

fn apply_