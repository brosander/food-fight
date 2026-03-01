use bevy::prelude::*;

use super::components::{Eliminated, Player, Velocity, DETENTION_CORNERS};

/// Play area half-extents (pixels from center).
const BOUNDS_X: f32 = 480.0;
const BOUNDS_Y: f32 = 320.0;

/// Size of the player sprite (for clamping so edges stay in bounds).
const PLAYER_HALF_SIZE: f32 = 16.0;

/// Applies velocity to transform each fixed tick, clamped to play area bounds.
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        // Clamp to play area
        transform.translation.x = transform
            .translation
            .x
            .clamp(-BOUNDS_X + PLAYER_HALF_SIZE, BOUNDS_X - PLAYER_HALF_SIZE);
        transform.translation.y = transform
            .translation
            .y
            .clamp(-BOUNDS_Y + PLAYER_HALF_SIZE, BOUNDS_Y - PLAYER_HALF_SIZE);
    }
}

/// Locks eliminated players to their assigned corner detention table every tick.
/// Runs after movement_system to override any residual velocity.
pub fn detention_system(
    mut players: Query<(&Player, &mut Transform, &mut Velocity), With<Eliminated>>,
) {
    for (player, mut transform, mut velocity) in &mut players {
        velocity.0 = Vec2::ZERO;
        let idx = (player.id as usize).saturating_sub(1).min(3);
        let corner = DETENTION_CORNERS[idx];
        transform.translation.x = corner.x;
        transform.translation.y = corner.y;
    }
}
