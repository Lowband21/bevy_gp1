use crate::Platform;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::config::*;

use crate::PlaybackMode;

use crate::Enemy;
use crate::GameOver;
use crate::Score;
use crate::Star;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Resource)]
pub struct Gravity {
    pub value: f32,
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity { value: -980.0 } // Arbitrary gravity value.
    }
}

#[derive(Resource)]
pub struct PlayerJumpState {
    pub can_jump: bool,
    pub jump_force: f32,
}

impl Default for PlayerJumpState {
    fn default() -> Self {
        PlayerJumpState {
            can_jump: true,
            jump_force: 1000.0,
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
        Velocity { value: Vec3::ZERO },
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Space) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn physics_system(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // Apply gravity to velocity.
        velocity.value.y += gravity.value * time.delta_seconds();

        // Apply velocity to transform.
        transform.translation += velocity.value * time.delta_seconds();
    }
}

pub fn player_jump_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), (With<Player>, Without<Transform>)>,
    mut jump_state: ResMut<PlayerJumpState>,
) {
    if jump_state.can_jump && keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok((mut velocity, _transform)) = query.get_single_mut() {
            velocity.value.y += jump_state.jump_force;
            jump_state.can_jump = false;
        }
    }
}

pub fn player_landing_system(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    platform_query: Query<&Transform, (With<Platform>, Without<Player>)>,
    mut jump_state: ResMut<PlayerJumpState>,
) {
    for (mut player_transform, mut player_velocity) in player_query.iter_mut() {
        for platform_transform in platform_query.iter() {
            // Assuming platforms have a defined height, for simplicity.
            let platform_height = 30.0;
            if player_transform.translation.y <= platform_transform.translation.y + platform_height
                && player_velocity.value.y < 0.0
            {
                player_velocity.value.y = 0.0;
                player_transform.translation.y = platform_transform.translation.y + platform_height;
                jump_state.can_jump = true;
            }
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = (window.width() - half_player_size) * 100.0;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");

                // Play the sound effect.
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                });

                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
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
