use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;
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

use crate::cam::*;
use crate::enemy::*;
use crate::environment::*;
use crate::player::*;
use crate::prelude::*;
use crate::ui::*;

use seldom_pixel::PxPlugin;

struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gravity>()
            .add_systems(Update, physics_system);
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Running,
    Paused,
}

// Example asset collection for an image asset
#[derive(AssetCollection, Resource)]
pub struct PlayerAnimation {
    #[asset(texture_atlas(tile_size_x = 24.0, tile_size_y = 24.0, columns = 24, rows = 1))]
    #[asset(path = "sprites/sheets/DinoSprites - doux.png")]
    player: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins((
            CameraPlugin,
            PlayerPlugin,
            PhysicsPlugin,
            UIPlugin,
            EnemyPlugin,
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_systems(Startup, spawn_platforms)
        // Add a loading state for assets
        .add_loading_state(
            LoadingState::new(GameState::Paused).continue_to_state(GameState::Running),
        )
        // Initialize your asset collection
        .init_collection::<PlayerAnimation>()
        .run();
}
