mod camera;
mod quit;
mod asset_loader;
mod bird;
mod gravity;

use bevy::prelude::*;
use crate::bird::BirdPlugin;
use crate::camera::CameraPlugin;
use crate::gravity::GravityPlugin;
use crate::quit::QuitPlugin;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        // Bevy built-ins
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Floppy Bord".into(),
                        resolution: (500., 800.).into(),
                        position: WindowPosition::At(IVec2::from((10, 10))),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
        )
// User defined
        .add_plugins(CameraPlugin)
        .add_plugins(QuitPlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(GravityPlugin)
        .run();
}
