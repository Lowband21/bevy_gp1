use crate::player::Player;
use bevy::prelude::*;

use crate::player::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 40;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const NUMBER_OF_STARS: usize = 100;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.
pub const STAR_SPAWN_TIME: f32 = 0.5;
pub const ENEMY_SPAWN_TIME: f32 = 3.0;
