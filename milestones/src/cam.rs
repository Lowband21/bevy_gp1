use crate::player::Player;
use bevy::app::App;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

//use crate::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow_system)
            .add_systems(Update, camera_zoom_system);
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, window.height() / 2.0, 1000.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn camera_follow_system(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            // Follow on X-axis but don't move left past the start point (0.0)
            camera_transform.translation.x = camera_transform
                .translation
                .x
                .max(player_transform.translation.x);

            // Simple lerp on the Y-axis for smoother vertical movement
            let lerp_factor = 0.05; // Adjust this value for faster/slower vertical tracking
            camera_transform.translation.y +=
                (player_transform.translation.y - camera_transform.translation.y) * lerp_factor;
        }
    }
}

pub fn camera_zoom_system(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut event_reader: EventReader<MouseWheel>, // Changed this line
) {
    let mut zoom = 1.0;
    for event in event_reader.iter() {
        zoom += event.y * 0.00001; // Adjust the multiplier for faster/slower zoom
    }

    for mut transform in camera_query.iter_mut() {
        transform.scale *= zoom;
    }
}
