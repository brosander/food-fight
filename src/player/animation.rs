use bevy::prelude::*;

use super::components::Velocity;

/// Placeholder animation: rotates the sprite slightly in the direction of movement
/// to give visual feedback. Will be replaced with real sprite sheet animation later.
pub fn animation_system(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        if velocity.0.length_squared() > 0.01 {
            // Slight scale pulse when moving
            let t = transform.translation.x.sin() * 0.02;
            transform.scale = Vec3::splat(1.0 + t);
        } else {
            transform.scale = Vec3::ONE;
        }
    }
}
