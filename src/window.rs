use bevy::prelude::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(bevy::prelude::WindowPlugin {
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
        );
    }
}