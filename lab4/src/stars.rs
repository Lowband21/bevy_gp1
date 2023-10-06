use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::Player;

use crate::config::*;

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
