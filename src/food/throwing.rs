use bevy::prelude::*;

use super::components::*;
use crate::controller::read_aim_direction;
use crate::player::components::GamepadLink;
use crate::states::Gameplay;

const PICKUP_RANGE: f32 = 40.0;

/// Pickup system: when player presses South (A/Cross) near a Throwable food, add it to inventory.
pub fn pickup_system(
    mut commands: Commands,
    gamepads: Query<&Gamepad>,
    players: Query<(Entity, &Transform, &GamepadLink, &Inventory)>,
    food_items: Query<(Entity, &Transform, &FoodItem), With<Throwable>>,
) {
    for (player_entity, player_tf, link, inventory) in &players {
        if inventory.held_food.is_some() {
            continue;
        }

        let Ok(gamepad) = gamepads.get(link.0) else {
            continue;
        };

        if !gamepad.just_pressed(GamepadButton::South) {
            continue;
        }

        // Find nearest food in range
        let mut nearest: Option<(Entity, f32, FoodType)> = None;
        for (food_entity, food_tf, food_item) in &food_items {
            let dist = player_tf
                .translation
                .truncate()
                .distance(food_tf.translation.truncate());
            if dist < PICKUP_RANGE {
                if nearest.is_none() || dist < nearest.as_ref().unwrap().1 {
                    nearest = Some((food_entity, dist, food_item.food_type));
                }
            }
        }

        if let Some((food_entity, _, food_type)) = nearest {
            commands.entity(food_entity).despawn();
            commands.entity(player_entity).insert(Inventory {
                held_food: Some(food_type),
            });
        }
    }
}

/// Throw system: Right trigger to throw, right stick (or left stick fallback) to aim.
pub fn throw_system(
    mut commands: Commands,
    gamepads: Query<&Gamepad>,
    players: Query<(Entity, &Transform, &GamepadLink, &Inventory)>,
) {
    for (player_entity, player_tf, link, inventory) in &players {
        let Some(food_type) = &inventory.held_food else {
            continue;
        };

        let Ok(gamepad) = gamepads.get(link.0) else {
            continue;
        };

        if !gamepad.just_pressed(GamepadButton::RightTrigger2) {
            continue;
        }

        let aim_direction = read_aim_direction(gamepad);
        if aim_direction == Vec2::ZERO {
            continue;
        }

        let food_type = *food_type;
        let stats = food_type.stats();

        // Spawn projectile
        let mut projectile = commands.spawn((
            Sprite {
                color: stats.color,
                custom_size: Some(stats.size * 0.8),
                ..default()
            },
            Transform::from_xyz(
                player_tf.translation.x,
                player_tf.translation.y,
                2.0,
            ),
            InFlight {
                thrown_by: player_entity,
                direction: aim_direction,
                speed: stats.speed,
                damage: stats.damage,
                max_range: 600.0,
                distance_traveled: 0.0,
            },
            Gameplay,
        ));

        // Add trajectory-specific components
        match stats.trajectory {
            TrajectoryKind::Arc { gravity } => {
                projectile.insert(ArcFlight {
                    vertical_velocity: 150.0,
                    gravity,
                    simulated_z: 0.0,
                });
            }
            TrajectoryKind::Bounce { bounces } => {
                projectile.insert(BounceFlight {
                    bounces_remaining: bounces,
                });
            }
            TrajectoryKind::Straight => {}
        }

        // Clear inventory
        commands.entity(player_entity).insert(Inventory {
            held_food: None,
        });
    }
}
