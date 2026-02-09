use bevy::prelude::*;

use super::components::*;
use crate::sprites::{SpriteAssets, effects_atlas_index, food_splat_index};
use crate::states::Gameplay;

/// Play area bounds (must match movement.rs).
const BOUNDS_X: f32 = 480.0;
const BOUNDS_Y: f32 = 320.0;

/// Moves straight-flying projectiles and despawns them when out of range or off screen.
pub fn straight_trajectory_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<
        (Entity, &mut Transform, &mut InFlight),
        (Without<ArcFlight>, Without<BounceFlight>),
    >,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut flight) in &mut projectiles {
        let movement = flight.direction * flight.speed * dt;
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        flight.distance_traveled += movement.length();

        // Despawn if out of range or off screen
        if flight.distance_traveled > flight.max_range
            || transform.translation.x.abs() > BOUNDS_X + 50.0
            || transform.translation.y.abs() > BOUNDS_Y + 50.0
        {
            commands.entity(entity).despawn();
        }
    }
}

/// Moves arc-trajectory projectiles (lob/throw with gravity).
pub fn arc_trajectory_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(
        Entity,
        &mut Transform,
        &mut InFlight,
        &mut ArcFlight,
        Option<&FoodItem>,
    )>,
    sprite_assets: Res<SpriteAssets>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut flight, mut arc, food_item) in &mut projectiles {
        // Horizontal movement
        let movement = flight.direction * flight.speed * dt;
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        flight.distance_traveled += movement.length();

        // Simulated vertical arc (affects scale to fake height)
        arc.vertical_velocity -= arc.gravity * dt;
        arc.simulated_z += arc.vertical_velocity * dt;

        // Scale sprite based on simulated height
        let height_scale = 1.0 + (arc.simulated_z / 100.0).max(0.0) * 0.5;
        transform.scale = Vec3::splat(height_scale);

        // "Landed" when simulated_z goes below 0
        if arc.simulated_z < 0.0 && arc.vertical_velocity < 0.0 {
            let food_type = food_item.map(|fi| &fi.food_type);
            spawn_splat(
                &mut commands,
                transform.translation.truncate(),
                food_type,
                &sprite_assets,
            );
            commands.entity(entity).despawn();
            continue;
        }

        // Also despawn if way off screen
        if transform.translation.x.abs() > BOUNDS_X + 50.0
            || transform.translation.y.abs() > BOUNDS_Y + 50.0
        {
            commands.entity(entity).despawn();
        }
    }
}

/// Moves bounce-trajectory projectiles, reflecting off walls.
pub fn bounce_trajectory_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(
        Entity,
        &mut Transform,
        &mut InFlight,
        &mut BounceFlight,
        Option<&FoodItem>,
    )>,
    sprite_assets: Res<SpriteAssets>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut flight, mut bounce, food_item) in &mut projectiles {
        let movement = flight.direction * flight.speed * dt;
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        flight.distance_traveled += movement.length();

        // Bounce off walls
        let mut bounced = false;
        if transform.translation.x.abs() > BOUNDS_X - 5.0 {
            flight.direction.x = -flight.direction.x;
            transform.translation.x =
                transform.translation.x.clamp(-BOUNDS_X + 5.0, BOUNDS_X - 5.0);
            bounced = true;
        }
        if transform.translation.y.abs() > BOUNDS_Y - 5.0 {
            flight.direction.y = -flight.direction.y;
            transform.translation.y =
                transform.translation.y.clamp(-BOUNDS_Y + 5.0, BOUNDS_Y - 5.0);
            bounced = true;
        }

        let food_type = food_item.map(|fi| &fi.food_type);

        if bounced {
            if bounce.bounces_remaining == 0 {
                spawn_splat(
                    &mut commands,
                    transform.translation.truncate(),
                    food_type,
                    &sprite_assets,
                );
                commands.entity(entity).despawn();
                continue;
            }
            bounce.bounces_remaining -= 1;
            // Slow down on each bounce
            flight.speed *= 0.8;
        }

        // Despawn after max range
        if flight.distance_traveled > flight.max_range {
            spawn_splat(
                &mut commands,
                transform.translation.truncate(),
                food_type,
                &sprite_assets,
            );
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_splat(
    commands: &mut Commands,
    position: Vec2,
    food_type: Option<&FoodType>,
    sprite_assets: &SpriteAssets,
) {
    let index = food_type
        .map(|ft| food_splat_index(ft))
        .unwrap_or(effects_atlas_index(1, 0)); // default red splat

    commands.spawn((
        Sprite {
            image: sprite_assets.effects_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprite_assets.effects_layout.clone(),
                index,
            }),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.1),
        SplatEffect {
            lifetime: Timer::from_seconds(3.0, TimerMode::Once),
        },
        Gameplay,
    ));
}

/// Fades and despawns splat effects.
pub fn splat_fade_system(
    mut commands: Commands,
    time: Res<Time>,
    mut splats: Query<(Entity, &mut SplatEffect, &mut Sprite)>,
) {
    for (entity, mut splat, mut sprite) in &mut splats {
        splat.lifetime.tick(time.delta());
        let alpha = 1.0 - splat.lifetime.fraction();
        sprite.color = sprite.color.with_alpha(alpha * 0.5);
        if splat.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
