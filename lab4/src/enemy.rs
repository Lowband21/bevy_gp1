use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::config::*;
use crate::Player;

use bevy::audio::PlaybackMode;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
) {
    let window = window_query.get_single().unwrap();

    // Check if player exists and get its transform.
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return; // If player doesn't exist, exit the function early.
    };

    let player_position = player_transform.translation;

    const SAFE_ZONE_RADIUS: f32 = PLAYER_SIZE * 6.0; // You can adjust this as needed.

    // Change the center to be based on player's X position
    let center_x = player_position.x;
    let center_y = window.height() / 2.0; // Keep vertical centering based on window

    for _ in 0..NUMBER_OF_ENEMIES {
        let mut random_x;
        let mut random_y;

        loop {
            random_x = center_x + (random::<f32>() * window.width() - window.width() / 2.0);
            random_y = random::<f32>() * window.height();
            let distance_to_center =
                ((random_x - center_x).powi(2) + (random_y - center_y).powi(2)).sqrt();

            // If it's outside the safe zone, break the loop.
            if distance_to_center > SAFE_ZONE_RADIUS {
                break;
            }
        }

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0)
                    .normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn handle_enemy_boundary(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        let mut direction_changed = false;

        // Only change direction for left boundary
        if translation.x < x_min {
            translation.x = x_min;
            enemy.direction.x = enemy.direction.x.abs(); // Ensure positive X direction
            direction_changed = true;
        }

        // Change direction for top and bottom boundaries
        if translation.y < y_min {
            translation.y = y_min;
            enemy.direction.y = enemy.direction.y.abs(); // Ensure positive Y direction
            direction_changed = true;
        } else if translation.y > y_max {
            translation.y = y_max;
            enemy.direction.y = -enemy.direction.y.abs(); // Ensure negative Y direction
            direction_changed = true;
        }

        transform.translation = translation;

        // Play SFX if direction changed (i.e., boundary was hit)
        if direction_changed {
            // Load Sound Effects
            let sound_effects = [
                asset_server.load("audio/pluck_001.ogg"),
                asset_server.load("audio/pluck_002.ogg"),
            ];

            // Randomly select a sound effect.
            let selected_effect = if random::<f32>() > 0.5 {
                sound_effects[0].clone()
            } else {
                sound_effects[1].clone()
            };

            // Play the selected sound effect.
            commands.spawn(AudioBundle {
                source: selected_effect,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Once,
                    ..default()
                },
            });
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

const SAFE_DISTANCE_FROM_PLAYER: f32 = PLAYER_SIZE * 3.0; // You can adjust this as needed.

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let (player_x, player_y) = if let Ok(player_transform) = player_query.get_single() {
            (
                player_transform.translation.x,
                player_transform.translation.y,
            )
        } else {
            // If the player doesn't exist, choose a default behavior.
            // In this example, we're setting a default position, but you might want to just return and do nothing.
            (window.width() / 2.0, window.height() / 2.0)
        };

        let mut random_x;
        let mut random_y;

        loop {
            random_x = random::<f32>() * window.width();
            random_y = random::<f32>() * window.height();

            let distance_to_player =
                ((random_x - player_x).powi(2) + (random_y - player_y).powi(2)).sqrt();

            // If it's outside the safe zone around the player or the default position, break the loop.
            if distance_to_player > SAFE_DISTANCE_FROM_PLAYER {
                break;
            }
        }

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}
