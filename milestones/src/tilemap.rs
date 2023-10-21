// tilemap.rs
use bevy::prelude::*;
use bevy::reflect::Map;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_ecs_ldtk::*;
use bevy_ecs_tilemap::prelude::*;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tilemap);
    }
}

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn(Camera2dBundle::default());

    let handle = asset_server.load("ldtk/AdvancedAutoLayers.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: handle,
        transform: Transform::from_xyz(400.0, 0.0, 0.0), // Adjust position as needed.
        visibility: Visibility::Visible,
        ..Default::default()
    });
}
