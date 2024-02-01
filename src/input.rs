use bevy::prelude::*;
use crate::gravity::{Gravity, Velocity};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                         (
                             (jump_space,jump_tap),
                             apply_jump
                         ).chain(),
            )
            .add_event::<JumpEvent>();
    }
}

#[derive(Event)]
struct JumpEvent;

#[derive(Component)]
pub struct Jump {
    speed: f32,
}

impl Jump {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

fn jump_space(
    keyboard_input: Res<Input<KeyCode>>,
    mut jump_event_writer: EventWriter<JumpEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        jump_event_writer.send(JumpEvent);
    }
}

fn jump_tap(
    touches: Res<Touches>,
    mut jump_event_writer: EventWriter<JumpEvent>,
) {
    if touches.any_just_pressed() {
        jump_event_writer.send(JumpEvent);
    }
}

fn apply_jump(
    mut jump_event_reader: EventReader<JumpEvent>,
    mut bird: Query<(&mut Velocity, &Jump), With<Gravity>>,
) {
    let Ok((mut velocity, jump)) = bird.get_single_mut() else {
        println!("jump: could not find bird");
        return;
    };

    for _ in jump_event_reader.read() {
        velocity.velocity.y = jump.speed;
    }
}