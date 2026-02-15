use bevy::prelude::*;

use super::components::*;
use crate::input::ControllerInput;
use crate::sprites::{AnimationState, FrameRange, SpriteAssets, food_atlas_index, food_type_row};
use crate::states::Gameplay;

const PICKUP_RANGE: f32 = 40.0;

/// Pickup system: when player presses South (A/Cross) near a Throwable food, add it to inventory.
pub fn pickup_system(
    mut commands: Commands,
    players: Query<(Entity, &Transform, &ControllerInput, &Inventory)>,
    food_items: Query<(Entity, &Transform, &FoodItem), With<Throwable>>,
) {
    for (player_entity, player_tf, input, inventory) in &players {
        if inventory.held_food.is_some() {
            continue;
        }

        if !input.pickup_food.just_pressed {
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
    players: Query<(Entity, &Transform, &ControllerInput, &Inventory)>,
    sprite_assets: Res<SpriteAssets>,
) {
    for (player_entity, player_tf, input, inventory) in &players {
        let Some(food_type) = &inventory.held_food else {
            continue;
        };

        if !input.fire.just_pressed {
            continue;
        }

        let aim_direction = input.aim_direction();
        if aim_direction == Vec2::ZERO {
            continue;
        }

        let food_type = *food_type;
        let stats = food_type.stats();
        let row = food_type_row(&food_type);
        let flight_start = food_atlas_index(row, 2);

        // Spawn projectile
        let mut projectile = commands.spawn((
            Sprite {
                image: sprite_assets.food_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_assets.food_layout.clone(),
                    index: flight_start,
                }),
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
            FoodItem {
                food_type,
                damage: stats.damage,
            },
            AnimationState::new(
                "flight",
                FrameRange {
                    start: food_atlas_index(row, 2),
                    end: food_atlas_index(row, 3),
                    fps: 10.0,
                    looping: true,
                },
            ),
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
