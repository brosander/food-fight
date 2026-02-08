use bevy::prelude::*;

use super::Wall;
use crate::food::components::InFlight;

/// Prevents players from walking through walls. Uses AABB push-out.
pub fn wall_collision_system(
    mut movers: Query<&mut Transform, (Without<Wall>, Without<InFlight>)>,
    walls: Query<(&Transform, &Wall)>,
) {
    for mut mover_tf in &mut movers {
        let mover_pos = mover_tf.translation.truncate();
        let mover_half = Vec2::splat(16.0); // Player half-size

        for (wall_tf, wall) in &walls {
            let wall_pos = wall_tf.translation.truncate();
            let wall_half = wall.half_size;

            let diff = mover_pos - wall_pos;
            let overlap_x = mover_half.x + wall_half.x - diff.x.abs();
            let overlap_y = mover_half.y + wall_half.y - diff.y.abs();

            if overlap_x > 0.0 && overlap_y > 0.0 {
                // Push out along the axis with the smallest overlap
                if overlap_x < overlap_y {
                    mover_tf.translation.x += overlap_x * diff.x.signum();
                } else {
                    mover_tf.translation.y += overlap_y * diff.y.signum();
                }
            }
        }
    }
}

/// Projectiles despawn or bounce when hitting walls.
pub fn projectile_wall_collision_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &InFlight)>,
    walls: Query<(&Transform, &Wall), Without<InFlight>>,
) {
    for (proj_entity, proj_tf, _flight) in &projectiles {
        let proj_pos = proj_tf.translation.truncate();
        let proj_half = Vec2::splat(4.0);

        for (wall_tf, wall) in &walls {
            let wall_pos = wall_tf.translation.truncate();
            let wall_half = wall.half_size;

            let diff = proj_pos - wall_pos;
            let overlap_x = proj_half.x + wall_half.x - diff.x.abs();
            let overlap_y = proj_half.y + wall_half.y - diff.y.abs();

            if overlap_x > 0.0 && overlap_y > 0.0 {
                // Check if this projectile has bounce component
                // For now, just despawn on wall hit
                commands.entity(proj_entity).despawn();
                break;
            }
        }
    }
}
