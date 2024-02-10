use bevy::prelude::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(bevy::prelude::WindowPlugin {
                    primary_window: Some(Window {
                        title: "Floppy Bord".into(),
                        resolution: (500., 600.).into(),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        );
    }
}