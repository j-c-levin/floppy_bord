mod camera;
mod quit;
mod bird;
mod gravity;
mod input;
mod state;
mod window;
mod despawn;
mod rock;
mod debug;

use bevy::prelude::*;
use crate::bird::BirdPlugin;
use crate::camera::CameraPlugin;
use crate::gravity::GravityPlugin;
use crate::input::InputPlugin;
use crate::quit::QuitPlugin;
use crate::rock::RockPlugin;
use crate::state::StatePlugin;
use crate::window::WindowPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
use crate::debug::DebugPlugin;

fn main() {
    App::new()
        .add_plugins(WindowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(QuitPlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(GravityPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(RockPlugin)
        // debug plugins
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(DebugPlugin)
        .run();
}
