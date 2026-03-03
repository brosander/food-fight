use bevy::prelude::*;

use super::components::*;
use super::launcher::{ChargingShot, EquippedLauncher, LauncherPickup, LauncherType};
use crate::audio::SoundEvent;
use crate::input::ControllerInput;
use crate::player::components::Eliminated;
use crate::sprites::{
    AnimationState, FrameRange, SpriteAssets, food_atlas_index, food_type_row,
};
use crate::states::Gameplay;

const PICKUP_RANGE: f32 = 70.0;

/// Unified pickup system: South button picks up the nearest food or launcher.
/// If the player already holds something, it is dropped to the ground first.
pub fn pickup_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    mut players: Query<
        (
            Entity,
            &Transform,
            &ControllerInput,
            &mut Inventory,
            Option<&EquippedLauncher>,
        ),
        Without<Eliminated>,
    >,
    food_items: Query<(Entity, &Transform, &FoodItem), With<Throwable>>,
    launchers: Query<(Entity, &Transform, &LauncherPickup)>,
) {
    for (player_entity, player_tf, input, mut inventory, equipped_launcher) in &mut players {
        if !input.pickup_food.just_pressed {
            continue;
        }

        let player_pos = player_tf.translation.truncate();

        // Find nearest item (food or launcher) within range
        enum NearestItem {
            Food(Entity, FoodType),
            Launcher(Entity, LauncherType),
        }

        let mut nearest: Option<(f32, NearestItem)> = None;

        for (food_entity, food_tf, food_item) in &food_items {
            let dist = player_pos.distance(food_tf.translation.truncate());
            if dist < PICKUP_RANGE {
                if nearest.is_none() || dist < nearest.as_ref().unwrap().0 {
                    nearest = Some((dist, NearestItem::Food(food_entity, food_item.food_type)));
                }
            }
        }

        for (launcher_entity, launcher_tf, pickup) in &launchers {
            let dist = player_pos.distance(launcher_tf.translation.truncate());
            if dist < PICKUP_RANGE {
                if nearest.is_none() || dist < nearest.as_ref().unwrap().0 {
                    nearest = Some((dist, NearestItem::Launcher(launcher_entity, pickup.launcher_type)));
                }
            }
        }

        let Some((_, item)) = nearest else { continue };

        // Destroy whatever the player is currently holding
        if inventory.held_food.is_some() {
            inventory.held_food = None;
        } else if equipped_launcher.is_some() {
            commands
                .entity(player_entity)
                .remove::<EquippedLauncher>()
                .remove::<ChargingShot>();
        }

        // Pick up the nearest item
        match item {
            NearestItem::Food(food_entity, food_type) => {
                commands.entity(food_entity).despawn();
                inventory.held_food = Some(food_type);
                sound.send(SoundEvent::FoodPickup);
            }
            NearestItem::Launcher(launcher_entity, launcher_type) => {
                let stats = launcher_type.stats();
                commands.entity(launcher_entity).despawn();
                commands.entity(player_entity).insert(EquippedLauncher {
                    launcher_type,
                    cooldown_timer: Timer::from_seconds(stats.cooldown_secs, TimerMode::Once),
                    uses_remaining: stats.uses,
                });
                sound.send(SoundEvent::LauncherPickup);
            }
        }
    }
}

/// Throw system: Right trigger to throw, right stick (or left stick fallback) to aim.
pub fn throw_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    players: Query<(Entity, &Transform, &ControllerInput, &Inventory), Without<Eliminated>>,
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

        sound.send(SoundEvent::FoodThrow);

        // Clear inventory
        commands.entity(player_entity).insert(Inventory {
            held_food: None,
        });
    }
}
