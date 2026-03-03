use bevy::prelude::*;

use super::components::*;
use crate::audio::SoundEvent;
use crate::input::ControllerInput;
use crate::player::components::Eliminated;
use crate::sprites::{AnimationState, FrameRange, SpriteAssets, launcher_atlas_index, launcher_type_row};
use crate::states::Gameplay;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LauncherType {
    Slingshot,
    KetchupGun,
    SporkLauncher,
    LunchTrayCatapult,
    StrawBlowgun,
    WatermelonCatapult,
}

impl LauncherType {
    pub fn stats(&self) -> LauncherStats {
        match self {
            LauncherType::Slingshot => LauncherStats {
                cooldown_secs: 0.5,
                speed_multiplier: 1.5,
                range_multiplier: 1.5,
                damage_multiplier: 1.0,
                uses: 10,
                color: Color::srgb(0.6, 0.4, 0.2),
                size: Vec2::new(16.0, 16.0),
                projectile_size: Vec2::new(6.0, 6.0),
            },
            LauncherType::KetchupGun => LauncherStats {
                cooldown_secs: 0.08,
                speed_multiplier: 0.8,
                range_multiplier: 0.5,
                damage_multiplier: 0.3,
                uses: 40,
                color: Color::srgb(0.9, 0.1, 0.1),
                size: Vec2::new(18.0, 12.0),
                projectile_size: Vec2::new(6.0, 6.0),
            },
            LauncherType::SporkLauncher => LauncherStats {
                cooldown_secs: 0.3,
                speed_multiplier: 2.0,
                range_multiplier: 2.0,
                damage_multiplier: 0.8,
                uses: 15,
                color: Color::srgb(0.7, 0.7, 0.8),
                size: Vec2::new(14.0, 14.0),
                projectile_size: Vec2::new(6.0, 6.0),
            },
            LauncherType::LunchTrayCatapult => LauncherStats {
                cooldown_secs: 1.5,
                speed_multiplier: 1.0,
                range_multiplier: 1.5,
                damage_multiplier: 2.0,
                uses: 5,
                color: Color::srgb(0.5, 0.5, 0.4),
                size: Vec2::new(20.0, 16.0),
                projectile_size: Vec2::new(6.0, 6.0),
            },
            LauncherType::StrawBlowgun => LauncherStats {
                cooldown_secs: 0.12,
                speed_multiplier: 2.5,
                range_multiplier: 1.2,
                damage_multiplier: 0.15,
                uses: 50,
                color: Color::srgb(0.9, 0.9, 0.5),
                size: Vec2::new(20.0, 6.0),
                projectile_size: Vec2::new(6.0, 6.0),
            },
            // Single-shot, slow, devastating. Fires a massive watermelon in an arc.
            LauncherType::WatermelonCatapult => LauncherStats {
                cooldown_secs: 1.0,
                speed_multiplier: 0.35,
                range_multiplier: 1.2,
                damage_multiplier: 5.0,
                uses: 1,
                color: Color::srgb(0.15, 0.65, 0.2),
                size: Vec2::new(22.0, 18.0),
                projectile_size: Vec2::new(20.0, 20.0),
            },
        }
    }

    pub const ALL: &[LauncherType] = &[
        LauncherType::Slingshot,
        LauncherType::KetchupGun,
        LauncherType::SporkLauncher,
        LauncherType::LunchTrayCatapult,
        LauncherType::StrawBlowgun,
        LauncherType::WatermelonCatapult,
    ];
}

pub struct LauncherStats {
    pub cooldown_secs: f32,
    pub speed_multiplier: f32,
    pub range_multiplier: f32,
    pub damage_multiplier: f32,
    pub uses: u32,
    pub color: Color,
    pub size: Vec2,
    pub projectile_size: Vec2,
}

/// Component for a launcher on the ground that can be picked up.
#[derive(Component)]
pub struct LauncherPickup {
    pub launcher_type: LauncherType,
}

/// Component on a player who is holding a launcher.
#[derive(Component)]
pub struct EquippedLauncher {
    pub launcher_type: LauncherType,
    pub cooldown_timer: Timer,
    pub uses_remaining: u32,
}

/// Marker for the lunch tray catapult charge-up mechanic.
#[derive(Component)]
pub struct ChargingShot {
    pub charge_time: f32,
    pub max_charge: f32,
}

const LAUNCHER_RESPAWN_SECS: f32 = 20.0;
/// Projectiles must always outrun a player (player speed = 200 px/s).
const MIN_PROJECTILE_SPEED: f32 = 250.0;

/// Spawn the single center launcher spawn point and its initial pickup.
pub fn setup_launcher_spawns(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Spawn the persistent spawn-point marker at center, already active
    // because we immediately place a launcher there.
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.6),
        LauncherSpawnPoint {
            respawn_timer: Timer::from_seconds(LAUNCHER_RESPAWN_SECS, TimerMode::Once),
            active: true,
        },
        Gameplay,
    ));

    // Immediately place one launcher at center on game start.
    let launcher_type = LauncherType::ALL[rng.gen_range(0..LauncherType::ALL.len())];
    let stats = launcher_type.stats();
    let row = launcher_type_row(&launcher_type);
    let ground_index = launcher_atlas_index(row, 0);

    commands.spawn((
        Sprite {
            image: sprite_assets.launcher_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprite_assets.launcher_layout.clone(),
                index: ground_index,
            }),
            custom_size: Some(stats.size),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.6),
        LauncherPickup { launcher_type },
        Gameplay,
    ));
}

/// Respawn a launcher at center when the 20s timer expires.
pub fn launcher_respawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_points: Query<(&Transform, &mut LauncherSpawnPoint)>,
    sprite_assets: Res<SpriteAssets>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for (transform, mut sp) in &mut spawn_points {
        if sp.active {
            continue;
        }

        sp.respawn_timer.tick(time.delta());
        if !sp.respawn_timer.finished() {
            continue;
        }

        let launcher_type = LauncherType::ALL[rng.gen_range(0..LauncherType::ALL.len())];
        let stats = launcher_type.stats();
        let row = launcher_type_row(&launcher_type);
        let ground_index = launcher_atlas_index(row, 0);

        commands.spawn((
            Sprite {
                image: sprite_assets.launcher_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_assets.launcher_layout.clone(),
                    index: ground_index,
                }),
                custom_size: Some(stats.size),
                ..default()
            },
            Transform::from_xyz(transform.translation.x, transform.translation.y, 0.6),
            LauncherPickup { launcher_type },
            Gameplay,
        ));

        sp.active = true;
    }
}

/// When the launcher pickup at center is gone, reset the spawn point timer.
pub fn reset_launcher_spawn_point_system(
    pickups: Query<&Transform, With<LauncherPickup>>,
    mut spawn_points: Query<(&Transform, &mut LauncherSpawnPoint)>,
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

const PICKUP_RANGE: f32 = 70.0;

/// Pickup system for launchers: player presses West (X/Square) near a launcher.
pub fn launcher_pickup_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    players: Query<(Entity, &Transform, &ControllerInput, Option<&EquippedLauncher>), Without<Eliminated>>,
    launchers: Query<(Entity, &Transform, &LauncherPickup)>,
) {
    for (player_entity, player_tf, input, equipped) in &players {
        if equipped.is_some() {
            continue;
        }

        if !input.pickup_launcher.just_pressed {
            continue;
        }

        // Find nearest launcher in range
        let mut nearest: Option<(Entity, f32, LauncherType)> = None;
        for (launcher_entity, launcher_tf, pickup) in &launchers {
            let dist = player_tf
                .translation
                .truncate()
                .distance(launcher_tf.translation.truncate());
            if dist < PICKUP_RANGE && (nearest.is_none() || dist < nearest.as_ref().unwrap().1) {
                nearest = Some((launcher_entity, dist, pickup.launcher_type));
            }
        }

        if let Some((launcher_entity, _, launcher_type)) = nearest {
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

/// Fire system for launchers. Right trigger to fire. Overrides normal throw when equipped.
/// Rapid-fire weapons (ketchup, blowgun) fire while trigger is held.
/// LunchTrayCatapult uses charge system separately.
pub fn launcher_fire_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    time: Res<Time>,
    sprite_assets: Res<SpriteAssets>,
    mut players: Query<(
        Entity,
        &Transform,
        &ControllerInput,
        &mut EquippedLauncher,
        Option<&ChargingShot>,
    ), Without<Eliminated>>,
) {
    for (player_entity, player_tf, input, mut launcher, _charging) in &mut players {
        launcher.cooldown_timer.tick(time.delta());

        // Skip catapult — handled by charge system
        if launcher.launcher_type == LauncherType::LunchTrayCatapult {
            continue;
        }

        let aim_direction = input.aim_direction();

        // For rapid-fire weapons, fire while trigger is held
        let is_rapid = matches!(
            launcher.launcher_type,
            LauncherType::KetchupGun | LauncherType::StrawBlowgun
        );
        let trigger = if is_rapid {
            input.fire.pressed
        } else {
            input.fire.just_pressed
        };

        if !trigger || aim_direction == Vec2::ZERO || !launcher.cooldown_timer.finished() {
            continue;
        }

        if launcher.uses_remaining == 0 {
            commands.entity(player_entity).remove::<EquippedLauncher>();
            continue;
        }

        let stats = launcher.launcher_type.stats();

        let base_damage = 10.0 * stats.damage_multiplier;
        let base_speed = (300.0 * stats.speed_multiplier).max(MIN_PROJECTILE_SPEED);
        let base_range = 400.0 * stats.range_multiplier;

        let row = launcher_type_row(&launcher.launcher_type);
        let is_watermelon = launcher.launcher_type == LauncherType::WatermelonCatapult;

        let sprite = if is_watermelon {
            Sprite {
                image: sprite_assets.launcher_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_assets.launcher_layout.clone(),
                    index: launcher_atlas_index(row, 2),
                }),
                custom_size: Some(stats.projectile_size),
                ..default()
            }
        } else {
            Sprite {
                color: stats.color.with_alpha(0.8),
                custom_size: Some(stats.projectile_size),
                ..default()
            }
        };

        let mut proj = commands.spawn((
            sprite,
            Transform::from_xyz(player_tf.translation.x, player_tf.translation.y, 2.0),
            InFlight {
                thrown_by: player_entity,
                direction: aim_direction,
                speed: base_speed,
                damage: base_damage,
                max_range: base_range,
                distance_traveled: 0.0,
            },
            Gameplay,
        ));

        if is_watermelon {
            proj.insert(ArcFlight {
                vertical_velocity: 250.0,
                gravity: 350.0,
                simulated_z: 0.0,
            });
            proj.insert(AnimationState::new(
                "flight",
                FrameRange {
                    start: launcher_atlas_index(row, 2),
                    end: launcher_atlas_index(row, 3),
                    fps: 6.0,
                    looping: true,
                },
            ));
        }

        launcher.uses_remaining -= 1;
        launcher.cooldown_timer.reset();
        sound.send(SoundEvent::LauncherFire);
    }
}

/// Charge system for the LunchTrayCatapult.
/// Hold right trigger to charge, release to fire.
pub fn catapult_charge_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    time: Res<Time>,
    mut players: Query<(
        Entity,
        &Transform,
        &ControllerInput,
        &mut EquippedLauncher,
        Option<&mut ChargingShot>,
    ), Without<Eliminated>>,
) {
    for (player_entity, player_tf, input, mut launcher, mut charging) in &mut players {
        if launcher.launcher_type != LauncherType::LunchTrayCatapult {
            continue;
        }

        launcher.cooldown_timer.tick(time.delta());

        let holding = input.fire.pressed;
        let just_released = input.fire.just_released;

        if holding && launcher.cooldown_timer.finished() && launcher.uses_remaining > 0 {
            if let Some(ref mut charge) = charging {
                charge.charge_time = (charge.charge_time + time.delta_secs()).min(charge.max_charge);
            } else {
                commands.entity(player_entity).insert(ChargingShot {
                    charge_time: 0.0,
                    max_charge: 2.0,
                });
            }
        }

        if just_released {
            let charge_fraction = if let Some(ref charge) = charging {
                charge.charge_time / charge.max_charge
            } else {
                0.0
            };

            commands.entity(player_entity).remove::<ChargingShot>();

            if charge_fraction > 0.05 && launcher.uses_remaining > 0 {
                let aim_direction = input.aim_direction();

                let stats = launcher.launcher_type.stats();
                let base_damage = 10.0 * stats.damage_multiplier * (0.5 + charge_fraction * 1.5);
                let base_speed = (200.0 * stats.speed_multiplier * (0.5 + charge_fraction)).max(MIN_PROJECTILE_SPEED);
                let base_range = 400.0 * stats.range_multiplier * (0.5 + charge_fraction);

                let mut proj = commands.spawn((
                    Sprite {
                        color: stats.color.with_alpha(0.9),
                        custom_size: Some(Vec2::new(
                            12.0 + charge_fraction * 8.0,
                            12.0 + charge_fraction * 8.0,
                        )),
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
                        speed: base_speed,
                        damage: base_damage,
                        max_range: base_range,
                        distance_traveled: 0.0,
                    },
                    Gameplay,
                ));

                proj.insert(ArcFlight {
                    vertical_velocity: 200.0 * (0.5 + charge_fraction),
                    gravity: 300.0,
                    simulated_z: 0.0,
                });

                launcher.uses_remaining -= 1;
                launcher.cooldown_timer.reset();
                sound.send(SoundEvent::LauncherFire);
            }
        }
    }
}
