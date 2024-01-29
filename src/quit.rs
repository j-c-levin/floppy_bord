use bevy::prelude::*;

pub struct QuitPlugin;

impl Plugin for QuitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bevy::window::close_on_esc);
    }
}