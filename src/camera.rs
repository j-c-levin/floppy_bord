use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::WHITE));

    commands.spawn((
        MainCamera,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        }
    ));
}