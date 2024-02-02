mod camera;
mod quit;
mod asset_loader;
mod bird;
mod gravity;
mod input;
mod state;
mod window;

use bevy::prelude::*;
use crate::bird::BirdPlugin;
use crate::camera::CameraPlugin;
use crate::gravity::GravityPlugin;
use crate::input::InputPlugin;
use crate::quit::QuitPlugin;
use crate::state::StatePlugin;
use crate::window::WindowPlugin;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_plugins(WindowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(QuitPlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(GravityPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(StatePlugin)
        .run();
}
