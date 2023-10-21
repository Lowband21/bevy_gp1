use crate::GameState;
use crate::Platform;
use crate::PlayerAnimation;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerJumpState>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Running)))
            .add_systems(
                Update,
                confine_player_movement.run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                player_landing_system.run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                player_animation_system.run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct AnimationTimer {
    pub base_duration: f32,
    pub elapsed_time: f32,
    pub frame_count: usize,
    pub playback_speed: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum PlayerActionState {
    #[default]
    Idle,
    Moving,
    Jumping,
    Running,
    RunningAndJumping,
}

#[derive(Component)]
pub struct PlayerState {
    pub action_state: PlayerActionState,
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
            jump_force: 500.0,
        }
    }
}
use seldom_pixel::prelude::*;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset: Res<PlayerAnimation>, // Assuming PlayerWalk holds the sprite sheet or its handle
) {
    // Set up the camera
    //commands.spawn(Camera2dBundle::default());
    let window = window_query.get_single().unwrap();

    // Define the sprite dimensions
    let sprite = TextureAtlasSprite {
        index: 0,
        custom_size: Some(Vec2::splat(16.0)), // 140 is the width and height of each frame
        ..Default::default()
    };

    // Spawn your player entity with the necessary components, including the idle animation attributes
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: asset.player.clone(),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            sprite,
            ..Default::default()
        },
        AnimationTimer {
            base_duration: 0.125,
            elapsed_time: 0.0,
            frame_count: 3,
            playback_speed: 1.0,
        },
        PlayerState {
            action_state: PlayerActionState::Idle,
        }, // Initialize with player not running
        Player {},
        Velocity { value: Vec3::ZERO },
    ));
}

fn player_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &PlayerState)>,
) {
    for (mut sprite, mut animation, player_state) in query.iter_mut() {
        animation.elapsed_time += time.delta_seconds() * animation.playback_speed;

        if animation.elapsed_time >= animation.base_duration {
            animation.elapsed_time -= animation.base_duration;

            match player_state.action_state {
                PlayerActionState::Idle => {
                    animation.frame_count = 3;
                    sprite.index = (sprite.index + 1) % animation.frame_count;
                }
                PlayerActionState::Moving | PlayerActionState::Running => {
                    animation.frame_count = 6;
                    sprite.index = 3 + (sprite.index - 3 + 1) % 6;
                }
                PlayerActionState::Jumping | PlayerActionState::RunningAndJumping => {
                    animation.frame_count = 4;
                    sprite.index = 5 + (sprite.index - 5 + 1) % 4;
                }
            }
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut PlayerState,
            &mut AnimationTimer,
            &mut Velocity,
        ),
        With<Player>,
    >,
    mut jump_state: ResMut<PlayerJumpState>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player_state, mut animation, mut velocity)) =
        player_query.get_single_mut()
    {
        let mut direction = Vec3::ZERO;
        let is_moving = keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A)
            || keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::D)
            || keyboard_input.pressed(KeyCode::Down)
            || keyboard_input.pressed(KeyCode::S);

        // Determine the current direction of the sprite (left or right)
        let mut facing_right = transform.scale.x > 0.0;

        // If pressing left and sprite was previously facing right, flip the sprite
        if (keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A))
            && facing_right
        {
            transform.scale.x = -1.0;
            facing_right = false;
        }

        // If pressing right and sprite was previously facing left, flip the sprite
        if (keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D))
            && !facing_right
        {
            transform.scale.x = 1.0;
            facing_right = true;
        }

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let is_running = keyboard_input.pressed(KeyCode::ShiftLeft)
            || keyboard_input.pressed(KeyCode::ShiftRight);
        let speed = if is_running {
            PLAYER_RUN_SPEED
        } else {
            PLAYER_SPEED
        };
        animation.playback_speed = match (is_running, player_state.action_state.clone()) {
            (true, PlayerActionState::Jumping) => 0.2,
            (true, PlayerActionState::Running) => 1.4,
            (true, PlayerActionState::RunningAndJumping) => 0.6,
            (false, PlayerActionState::Jumping) => 0.2,
            _ => 0.8,
        };

        transform.translation += direction * speed * time.delta_seconds();

        if keyboard_input.just_pressed(KeyCode::Space) && jump_state.can_jump {
            velocity.value.y += jump_state.jump_force;
            jump_state.can_jump = false;
            if is_running {
                player_state.action_state = PlayerActionState::RunningAndJumping;
            } else {
                player_state.action_state = PlayerActionState::Jumping;
            }
        }

        // Update player's action state based on their movements
        match player_state.action_state {
            PlayerActionState::Jumping | PlayerActionState::RunningAndJumping => {} // Do nothing if jumping
            _ => {
                if is_moving {
                    if is_running {
                        player_state.action_state = PlayerActionState::Running;
                    } else {
                        player_state.action_state = PlayerActionState::Moving;
                    }
                } else {
                    player_state.action_state = PlayerActionState::Idle;
                }
            }
        }
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

pub fn player_landing_system(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut PlayerState), With<Player>>,
    platform_query: Query<&Transform, (With<Platform>, Without<Player>)>,
    mut jump_state: ResMut<PlayerJumpState>,
) {
    for (mut player_transform, mut player_velocity, mut player_state) in player_query.iter_mut() {
        let player_bottom = player_transform.translation.y - PLAYER_SIZE / 2.0;

        for platform_transform in platform_query.iter() {
            let platform_top = platform_transform.translation.y + PLATFORM_HEIGHT as f32 / 2.0;

            // Bounding box checks (assuming origin is center for both player and platform)
            let player_left = player_transform.translation.x - PLAYER_SIZE / 2.0;
            let player_right = player_transform.translation.x + PLAYER_SIZE / 2.0;
            let platform_left = platform_transform.translation.x - PLATFORM_WIDTH as f32 / 2.0;
            let platform_right = platform_transform.translation.x + PLATFORM_WIDTH as f32 / 2.0;

            if player_bottom <= platform_top
                && player_velocity.value.y <= 0.0
                && player_left < platform_right
                && player_right > platform_left
            {
                player_velocity.value.y = 0.0;
                player_transform.translation.y = platform_top + PLAYER_SIZE / 2.0; // Adjust so player's bottom aligns with platform top
                jump_state.can_jump = true;

                // Adjusting player state
                if player_state.action_state == PlayerActionState::Jumping {
                    player_state.action_state = PlayerActionState::Idle;
                }
            }
            //println!("{}", player_velocity.value.y);
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        // Get the camera's X-axis position
        let camera_x = if let Ok(camera_transform) = camera_query.get_single() {
            camera_transform.translation.x
        } else {
            0.0
        };

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = camera_x - (window.width() / 2.0) + half_player_size; // Left boundary is camera's left edge
        let x_max = camera_x + (window.width() / 2.0) - half_player_size; // Right boundary is camera's right edge
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
