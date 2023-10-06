use bevy::app::App;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::audio::PlaybackMode;
use bevy::diagnostic::LogDiagnosticsPlugin;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::mouse::MouseWheel;
use bevy::text::prelude::Text;

mod config;
mod enemy;
mod environment;
mod player;
mod stars;

use crate::enemy::*;
use crate::environment::*;
use crate::player::*;
use crate::stars::*;

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerJumpState>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, confine_player_movement)
            .add_systems(Update, player_jump_system)
            .add_systems(Update, player_landing_system);
    }
}

struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, handle_enemy_boundary)
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, tick_enemy_spawn_timer)
            .add_systems(Update, spawn_enemies_over_time);
    }
}

struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, player_hit_star)
            .add_systems(Update, update_score)
            .add_systems(Update, tick_star_spawn_timer)
            .add_systems(Update, spawn_stars_over_time);
    }
}

struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gravity>()
            .add_systems(Update, physics_system);
    }
}

struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_event::<GameOver>()
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(PhysicsPlugin)
            .add_systems(Startup, spawn_platforms)
            .add_systems(Update, handle_game_over)
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated);
    }
}

struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow_system)
            .add_systems(Update, camera_zoom_system);
    }
}

struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fps_text)
            .add_systems(Update, fps_display_system);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(GameplayPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(FpsPlugin)
        .add_systems(Update, exit_game)
        .run();
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
}

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

fn fps_display_system(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        for mut text in query.iter_mut() {
            if let Some(avg) = fps.average() {
                text.sections[0].value = format!("FPS: {}", avg);
            } else {
                text.sections[0].value = "FPS: N/A".to_string();
            }
        }
    }
}

pub fn spawn_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "FPS: 0".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            alignment: TextAlignment::Right,
            linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
        },
        style: Style {
            position_type: PositionType::Absolute,
            margin: bevy::prelude::UiRect {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
