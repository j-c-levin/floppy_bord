mod camera;
mod quit;
mod bird;
mod gravity;
mod input;
mod state;
mod window;
mod despawn;
mod rock;

use bevy::prelude::*;
use crate::bird::BirdPlugin;
use crate::camera::CameraPlugin;
use crate::despawn::DespawnPlugin;
use crate::gravity::GravityPlugin;
use crate::input::InputPlugin;
use crate::quit::QuitPlugin;
use crate::rock::RockPlugin;
use crate::state::StatePlugin;
use crate::window::WindowPlugin;

fn main() {
    App::new()
        .add_plugins(WindowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(QuitPlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(GravityPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(RockPlugin)
        .run();
}
