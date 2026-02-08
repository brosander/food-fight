use bevy::prelude::*;

use crate::food::components::*;
use crate::player::components::{Health, Player};
use crate::states::Gameplay;

const PLAYER_HALF_SIZE: f32 = 16.0;

/// AABB collision between InFlight food and Player entities.
/// Skips the player who threw the food.
pub fn food_player_collision_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &InFlight)>,
    mut players: Query<(Entity, &Transform, &mut Health), With<Player>>,
) {
    for (proj_entity, proj_tf, flight) in &projectiles {
        let proj_pos = proj_tf.translation.truncate();
        let proj_half = Vec2::new(6.0, 6.0); // Approximate projectile half-size

        for (player_entity, player_tf, mut health) in &mut players {
            // Don't hit the thrower
            if player_entity == flight.thrown_by {
                continue;
            }

            let player_pos = player_tf.translation.truncate();

            // AABB overlap check
            if aabb_overlap(
                proj_pos,
                proj_half,
                player_pos,
                Vec2::splat(PLAYER_HALF_SIZE),
            ) {
                // Apply damage
                health.0 = (health.0 - flight.damage).max(0.0);

                // Spawn hit splat
                commands.spawn((
                    Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.8),
                        custom_size: Some(Vec2::new(24.0, 24.0)),
                        ..default()
                    },
                    Transform::from_xyz(
                        player_tf.translation.x,
                        player_tf.translation.y,
                        3.0,
                    ),
                    SplatEffect {
                        lifetime: Timer::from_seconds(0.3, TimerMode::Once),
                    },
                    Gameplay,
                ));

                // Despawn projectile
                commands.entity(proj_entity).despawn();
                break;
            }
        }
    }
}

fn aabb_overlap(pos_a: Vec2, half_a: Vec2, pos_b: Vec2, half_b: Vec2) -> bool {
    let diff = (pos_a - pos_b).abs();
    diff.x < half_a.x + half_b.x && diff.y < half_a.y + half_b.y
}
