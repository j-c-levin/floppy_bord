use bevy::prelude::*;
use crate::gravity::{Gravity, Velocity};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, jump);
    }
}

#[derive(Component)]
pub struct Jump {
    speed: f32,
}

impl Jump {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut bird: Query<(&mut Velocity, &Jump), With<Gravity>>,
) {
    let Ok((mut velocity, jump)) = bird.get_single_mut() else {
        println!("jump: could not find bird");
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.velocity.y = jump.speed;
    }
}