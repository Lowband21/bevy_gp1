use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorOptions;

use bevy::diagnostic::DiagnosticsStore;

use bevy::prelude::*;

// bevy_asset_loader imports
use bevy_asset_loader::prelude::*;

mod prelude;

mod cam;

mod player;

mod environment;

mod ui;

mod enemy;

mod stars;

mod tilemap;

mod helpers;

use crate::cam::*;
use crate::enemy::*;
use crate::environment::*;
use crate::player::*;
use crate::prelude::*;
use crate::stars::*;
use crate::tilemap::TilemapPlugin;
use crate::ui::*;

use seldom_pixel::PxPlugin;

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gravity>()
            .add_systems(Update, physics_system.run_if(in_state(GameState::Running)));
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Running,
    Loading,
    Paused,
}

// Example asset collection for an image asset
#[derive(AssetCollection, Resource)]
pub struct PlayerAnimation {
    #[asset(texture_atlas(tile_size_x = 24.0, tile_size_y = 24.0, columns = 24, rows = 1))]
    #[asset(path = "sprites/sheets/DinoSprites - doux.png")]
    player: Handle<TextureAtlas>,
}

/// System set to allow ordering of `PanCamPlugin`
#[derive(Debug, Clone, Copy, SystemSet, PartialEq, Eq, Hash)]
pub struct GameSystemSet;

fn main() {
    App::new()
        .add_event::<GameOver>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<GameState>()
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_plugins((
            CameraPlugin,
            PlayerPlugin,
            PhysicsPlugin,
            UIPlugin,
            EnemyPlugin,
            StarPlugin,
            TilemapPlugin,
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        //.add_systems(Startup, spawn_platforms)
        // Add a loading state for assets
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Paused),
        )
        // Initialize your asset collection
        .init_collection::<PlayerAnimation>()
        .run();
}
