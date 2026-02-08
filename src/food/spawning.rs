use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use crate::states::Gameplay;

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

            commands.spawn((
                Sprite {
                    color: stats.color,
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

/// Initial spawn: immediately spawn food at all points on game start.
pub fn initial_food_spawn(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for &(x, y) in SPAWN_POSITIONS {
        let food_type = FoodType::ALL[rng.gen_range(0..FoodType::ALL.len())];
        let stats = food_type.stats();

        commands.spawn((
            Sprite {
                color: stats.color,
                custom_size: Some(stats.size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.5),
            Throwable,
            FoodItem {
                food_type,
                damage: stats.damage,
            },
            Gameplay,
        ));
    }
}

