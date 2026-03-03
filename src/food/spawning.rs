use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use crate::sprites::{
    AnimationState, FrameRange, SpriteAssets, food_atlas_index, food_type_row,
    melee_atlas_index, melee_weapon_type_row,
};
use crate::states::Gameplay;

const MELEE_RESPAWN_SECS: f32 = 15.0;
const MELEE_SPAWN_POSITIONS: &[(f32, f32)] = &[(-280.0, 50.0), (280.0, -50.0)];

/// Hardcoded food spawn positions for the placeholder arena.
const SPAWN_POSITIONS: &[(f32, f32)] = &[
    (-200.0, 150.0),
    (200.0, 150.0),
    (-200.0, -150.0),
    (200.0, -150.0),
    (0.0, 200.0),
    (0.0, -200.0),
    (-350.0, 0.0),
    (350.0, 0.0),
];

/// Creates food spawn point markers on entering the Playing state.
pub fn setup_food_spawns(mut commands: Commands) {
    for &(x, y) in SPAWN_POSITIONS {
        commands.spawn((
            Sprite {
                color: Color::srgba(0.5, 0.5, 0.5, 0.3),
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            FoodSpawnPoint {
                respawn_timer: Timer::from_seconds(5.0, TimerMode::Once),
                active: false,
            },
            Gameplay,
        ));
    }
}

/// Spawns food at spawn points when their timer expires.
pub fn food_respawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_points: Query<(&Transform, &mut FoodSpawnPoint)>,
    sprite_assets: Res<SpriteAssets>,
) {
    let mut rng = rand::thread_rng();

    for (transform, mut spawn_point) in &mut spawn_points {
        if spawn_point.active {
            continue;
        }

        spawn_point.respawn_timer.tick(time.delta());
        if spawn_point.respawn_timer.finished() {
            let food_type = FoodType::ALL[rng.gen_range(0..FoodType::ALL.len())];
            let stats = food_type.stats();
            let row = food_type_row(&food_type);
            let ground_index = food_atlas_index(row, 0);

            commands.spawn((
                Sprite {
                    image: sprite_assets.food_image.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprite_assets.food_layout.clone(),
                        index: ground_index,
                    }),
                    custom_size: Some(stats.size),
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.5,
                ),
                Throwable,
                FoodItem {
                    food_type,
                    damage: stats.damage,
                },
                AnimationState::new(
                    "ground",
                    FrameRange {
                        start: food_atlas_index(row, 0),
                        end: food_atlas_index(row, 1),
                        fps: 3.0,
                        looping: true,
                    },
                ),
                Gameplay,
            ));

            spawn_point.active = true;
        }
    }
}

/// When food is picked up near a spawn point, reset that spawn point's timer.
pub fn reset_spawn_point_system(
    throwable_food: Query<&Transform, With<Throwable>>,
    mut spawn_points: Query<(&Transform, &mut FoodSpawnPoint)>,
) {
    for (sp_transform, mut spawn_point) in &mut spawn_points {
        if !spawn_point.active {
            continue;
        }

        // Check if the food near this spawn point still exists
        let has_food = throwable_food.iter().any(|food_tf| {
            food_tf
                .translation
                .truncate()
                .distance(sp_transform.translation.truncate())
                < 10.0
        });

        if !has_food {
            spawn_point.active = false;
            spawn_point.respawn_timer.reset();
        }
    }
}

/// Creates melee weapon spawn point markers and places initial pickups.
pub fn setup_melee_spawns(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    let mut rng = rand::thread_rng();

    for &(x, y) in MELEE_SPAWN_POSITIONS {
        let weapon_type = if rng.gen_bool(0.5) {
            MeleeWeaponType::LunchTray
        } else {
            MeleeWeaponType::Baguette
        };
        let row = melee_weapon_type_row(&weapon_type);

        commands.spawn((
            Transform::from_xyz(x, y, 0.0),
            MeleeWeaponSpawnPoint {
                respawn_timer: Timer::from_seconds(MELEE_RESPAWN_SECS, TimerMode::Once),
                active: true,
            },
            Gameplay,
        ));

        commands.spawn((
            Sprite {
                image: sprite_assets.melee_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_assets.melee_layout.clone(),
                    index: melee_atlas_index(row, 0),
                }),
                custom_size: Some(Vec2::splat(32.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.6),
            MeleeWeaponPickup { weapon_type },
            AnimationState::new(
                "ground",
                FrameRange {
                    start: melee_atlas_index(row, 0),
                    end: melee_atlas_index(row, 1),
                    fps: 3.0,
                    looping: true,
                },
            ),
            Gameplay,
        ));
    }
}

/// Respawns a melee weapon at each spawn point when the timer expires.
pub fn melee_respawn_system(
    mut commands: Commands,
    time: Res<Time>,
    sprite_assets: Res<SpriteAssets>,
    mut spawn_points: Query<(&Transform, &mut MeleeWeaponSpawnPoint)>,
) {
    let mut rng = rand::thread_rng();

    for (transform, mut sp) in &mut spawn_points {
        if sp.active {
            continue;
        }

        sp.respawn_timer.tick(time.delta());
        if !sp.respawn_timer.finished() {
            continue;
        }

        let weapon_type = if rng.gen_bool(0.5) {
            MeleeWeaponType::LunchTray
        } else {
            MeleeWeaponType::Baguette
        };
        let row = melee_weapon_type_row(&weapon_type);

        commands.spawn((
            Sprite {
                image: sprite_assets.melee_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_assets.melee_layout.clone(),
                    index: melee_atlas_index(row, 0),
                }),
                custom_size: Some(Vec2::splat(32.0)),
                ..default()
            },
            Transform::from_xyz(transform.translation.x, transform.translation.y, 0.6),
            MeleeWeaponPickup { weapon_type },
            AnimationState::new(
                "ground",
                FrameRange {
                    start: melee_atlas_index(row, 0),
                    end: melee_atlas_index(row, 1),
                    fps: 3.0,
                    looping: true,
                },
            ),
            Gameplay,
        ));

        sp.active = true;
    }
}

/// Resets the melee spawn point timer when the pickup is gone.
pub fn reset_melee_spawn_point_system(
    pickups: Query<&Transform, With<MeleeWeaponPickup>>,
    mut spawn_points: Query<(&Transform, &mut MeleeWeaponSpawnPoint)>,
) {
    for (sp_tf, mut sp) in &mut spawn_points {
        if !sp.active {
            continue;
        }

        let has_pickup = pickups.iter().any(|tf| {
            tf.translation
                .truncate()
                .distance(sp_tf.translation.truncate())
                < 10.0
        });

        if !has_pickup {
            sp.active = false;
            sp.respawn_timer.reset();
        }
    }
}

/// Initial spawn: immediately spawn food at all points on game start.
pub fn initial_food_spawn(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    let mut rng = rand::thread_rng();

    for &(x, y) in SPAWN_POSITIONS {
        let food_type = FoodType::ALL[rng.gen_range(0..FoodType::ALL.len())];
        let stats = food_type.stats();
        let row = food_type_row(&food_type);
        let ground_index = food_atlas_index(row, 0);

        commands.spawn((
            Sprite {
                image: sprite_assets.food_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_assets.food_layout.clone(),
                    index: ground_index,
                }),
                custom_size: Some(stats.size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.5),
            Throwable,
            FoodItem {
                food_type,
                damage: stats.damage,
            },
            AnimationState::new(
                "ground",
                FrameRange {
                    start: food_atlas_index(row, 0),
                    end: food_atlas_index(row, 1),
                    fps: 3.0,
                    looping: true,
                },
            ),
            Gameplay,
        ));
    }
}

