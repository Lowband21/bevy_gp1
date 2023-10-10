use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::Player;

use crate::prelude::*;

pub struct StarPlugin;

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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
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

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, (With<PrimaryWindow>, Without<Player>)>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, (With<Player>, Without<PrimaryWindow>)>,
) {
    let window = window_query.get_single().unwrap();
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return; // If player doesn't exist, exit the function early.
    };
    let player_position = player_transform.translation;

    // Use an offset to spawn stars to the right of the player
    const OFFSET: f32 = 100.0;

    for _ in 0..NUMBER_OF_STARS {
        let random_x = player_position.x + OFFSET + (random::<f32>() * window.width());
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, (With<PrimaryWindow>, Without<Player>)>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
    player_query: Query<&Transform, (With<Player>, Without<PrimaryWindow>)>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let player_transform = if let Ok(transform) = player_query.get_single() {
            transform
        } else {
            return; // If player doesn't exist, exit the function early.
        };
        let player_position = player_transform.translation;

        // Use an offset to spawn stars to the right of the player
        const OFFSET: f32 = 100.0;

        let random_x = player_position.x + OFFSET + (random::<f32>() * window.width());
        let random_y = ((random::<f32>() * window.height()) / 5.0) + 40.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                println!("Player hit star!");
                score.value += 1;
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");

                // Play the sound effect.
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                });

                commands.entity(star_entity).despawn();
            }
        }
    }
}
