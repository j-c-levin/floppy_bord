use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use crate::gravity::{Gravity, Velocity};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                         (
                             (jump_space, jump_tap),
                             apply_jump
                         ).chain(),
            )
            .add_event::<JumpEvent>();
    }
}

#[derive(Event)]
pub struct JumpEvent;

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
    mut touches: EventReader<TouchInput>,
    mut jump_event_writer: EventWriter<JumpEvent>,
) {
    for touch in touches.read() {
        match touch.phase {
            TouchPhase::Started => {
                jump_event_writer.send(JumpEvent);
            },
            _ => { /* do nothing */ }
        }
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::*;
    use bevy::input::touch::{TouchPhase, TouchInput};
    use bevy::math::Vec2;
    use crate::input::{jump_tap, JumpEvent};

    #[test]
    fn jump_on_tap() {
        // Setup app
        let mut app = App::new();
        app.add_event::<JumpEvent>();
        app.add_event::<TouchInput>();

         // Add our systems
        app.add_systems(Update, jump_tap);

        app.world
            .resource_mut::<Events<TouchInput>>()
            .send(TouchInput {
                phase: TouchPhase::Started,
                position: Vec2::ZERO,
                force: None,
                id: 4,
            });

        // Run systems
        app.update();

        let jump_event = app.world.resource::<Events<JumpEvent>>();
        let count = jump_event.get_reader().len(&jump_event);

        assert_eq!(count, 1);
    }

    #[test]
    fn ignore_drag() {
        // Setup app
        let mut app = App::new();
        app.add_event::<JumpEvent>();
        app.add_event::<TouchInput>();

        // Add our systems
        app.add_systems(Update, jump_tap);

        app.world
            .resource_mut::<Events<TouchInput>>()
            .send(TouchInput {
                phase: TouchPhase::Moved,
                position: Vec2::ZERO,
                force: None,
                id: 4,
            });

        // Run systems
        app.update();

        let jump_event = app.world.resource::<Events<JumpEvent>>();
        let count = jump_event.get_reader().len(&jump_event);

        assert_eq!(count, 0);
    }

    #[test]
    fn works_with_multiple_touches() {
        // Setup app
        let mut app = App::new();
        app.add_event::<JumpEvent>();
        app.add_event::<TouchInput>();

        // Add our systems
        app.add_systems(Update, jump_tap);

        app.world
            .resource_mut::<Events<TouchInput>>()
            .send(TouchInput {
                phase: TouchPhase::Moved,
                position: Vec2::ZERO,
                force: None,
                id: 4,
            });
        app.world
            .resource_mut::<Events<TouchInput>>()
            .send(TouchInput {
                phase: TouchPhase::Started,
                position: Vec2::ZERO,
                force: None,
                id: 5,
            });

        // Run systems
        app.update();

        let jump_event = app.world.resource::<Events<JumpEvent>>();
        let count = jump_event.get_reader().len(&jump_event);

        assert_eq!(count, 1);
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