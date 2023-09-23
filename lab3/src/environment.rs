use bevy::prelude::*;

#[derive(Component)]
pub struct Platform {}

pub const PLATFORM_WIDTH: i32 = 1640;
pub const NUM_PLATFORMS: i32 = 100;

pub fn spawn_platforms(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a platform as an example. You can add more.
    for i in 0..NUM_PLATFORMS {
        commands
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(650.0 + (PLATFORM_WIDTH * i) as f32, 100.0, 0.0), // Adjust position as needed.
                texture: asset_server.load("sprites/platform.png"), // Make sure to have a platform sprite.
                ..Default::default()
            })
            .insert(Platform {});
    }
}
