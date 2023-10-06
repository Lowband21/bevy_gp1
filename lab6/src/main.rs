use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorOptions;

use bevy::diagnostic::DiagnosticsStore;

use bevy::prelude::*;

mod prelude;

mod cam;

mod player;

mod environment;

mod ui;

use crate::cam::*;
use crate::environment::*;
use crate::player::*;
use crate::prelude::*;
use crate::ui::*;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins((CameraPlugin, PlayerPlugin, PhysicsPlugin, UIPlugin).in_set())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_systems(Startup, spawn_platforms)
        .run();
}
