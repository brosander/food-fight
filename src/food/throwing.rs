use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::player::components::{InputScheme, Velocity as PlayerVelocity};
use crate::states::Gameplay;

const PICKUP_RANGE: f32 = 40.0;

/// Pickup system: when player presses pickup key near a Throwable food, add it to inventory.
pub fn pickup_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    players: Query<(Entity, &Transform, &InputScheme, &Inventory)>,
    food_items: Query<(Entity, &Transform, &FoodItem), With<Throwable>>,
) {
    for (player_entity, player_tf, scheme, inventory) in &players {
        if inventory.held_food.is_some() {
            continue;
        }

        let pickup_pressed = match scheme {
            InputScheme::KeyboardMouse => keyboard.just_pressed(KeyCode::KeyE),
            InputScheme::ArrowKeys => keyboard.just_pressed(KeyCode::ShiftRight),
        };

        if !pickup_pressed {
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

/// Aim system: computes aim direction for each player.
/// P1 aims toward mouse. P2 aims in the direction of last movement.
pub fn throw_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    players: Query<(Entity, &Transform, &InputScheme, &Inventory, &PlayerVelocity)>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };

    for (player_entity, player_tf, scheme, inventory, player_vel) in &players {
        let Some(food_type) = &inventory.held_food else {
            continue;
        };

        let (should_throw, aim_direction) = match scheme {
            InputScheme::KeyboardMouse => {
                let throw = mouse_buttons.just_pressed(MouseButton::Left);
                let dir = if let Some(cursor_pos) = window.cursor_position() {
                    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
                    {
                        (world_pos - player_tf.translation.truncate()).normalize_or_zero()
                    } else {
                        Vec2::Y
                    }
                } else {
                    Vec2::Y
                };
                (throw, dir)
            }
            InputScheme::ArrowKeys => {
                let throw = keyboard.just_pressed(KeyCode::Enter);
                // Aim in last movement direction, or default to up
                let dir = if player_vel.0.length_squared() > 0.01 {
                    player_vel.0.normalize()
                } else {
                    // Use arrow keys for aiming when stationary
                    let mut aim = Vec2::ZERO;
                    if keyboard.pressed(KeyCode::ArrowUp) {
                        aim.y += 1.0;
                    }
                    if keyboard.pressed(KeyCode::ArrowDown) {
                        aim.y -= 1.0;
                    }
                    if keyboard.pressed(KeyCode::ArrowLeft) {
                        aim.x -= 1.0;
                    }
                    if keyboard.pressed(KeyCode::ArrowRight) {
                        aim.x += 1.0;
                    }
                    if aim == Vec2::ZERO {
                        Vec2::Y
                    } else {
                        aim.normalize()
                    }
                };
                (throw, dir)
            }
        };

        if !should_throw || aim_direction == Vec2::ZERO {
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
