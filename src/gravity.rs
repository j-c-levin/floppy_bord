use bevy::prelude::*;
pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
                        (
                            apply_gravity, apply_velocity
                        ).chain(),
        );
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
    time: Res<Time>,
) {
    let Ok((gravity, mut velocity)) = bird.get_single_mut() else {
        println!("apply_gravity: couldn't find bird");
        return;
    };

    velocity.velocity += gravity.acceleration * time.delta_seconds();
}

fn apply_velocity(
    mut query: Query<(&Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.velocity.x * time.delta_seconds();
        transform.translation.y += velocity.velocity.y * time.delta_seconds();
    }
}