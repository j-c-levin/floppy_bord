use bevy::prelude::*;
use crate::input::JumpEvent;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Setup,
    InGame,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(Update, (
                transition_to_in_game.run_if(in_state(GameState::Setup)),
                transition_to_setup.run_if(in_state(GameState::GameOver))
            ));
    }
}

fn transition_to_in_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut jump_event: EventReader<JumpEvent>
) {
    for _ in jump_event.read() {
        next_state.set(GameState::InGame);
    }
}

fn transition_to_setup(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::Setup);
}