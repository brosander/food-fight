use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::player::components::{InputScheme, Velocity as PlayerVelocity};
use crate::states::Gameplay;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LauncherType {
    Slingshot,
    KetchupGun,
    SporkLauncher,
    LunchTrayCatapult,
    StrawBlowgun,
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
            },
            LauncherType::KetchupGun => LauncherStats {
                cooldown_secs: 0.08,
                speed_multiplier: 0.8,
                range_multiplier: 0.5,
                damage_multiplier: 0.3,
                uses: 40,
                color: Color::srgb(0.9, 0.1, 0.1),
                size: Vec2::new(18.0, 12.0),
            },
            LauncherType::SporkLauncher => LauncherStats {
                cooldown_secs: 0.3,
                speed_multiplier: 2.0,
                range_multiplier: 2.0,
                damage_multiplier: 0.8,
                uses: 15,
                color: Color::srgb(0.7, 0.7, 0.8),
                size: Vec2::new(14.0, 14.0),
            },
            LauncherType::LunchTrayCatapult => LauncherStats {
                cooldown_secs: 1.5,
                speed_multiplier: 1.0,
                range_multiplier: 1.5,
                damage_multiplier: 2.0,
                uses: 5,
                color: Color::srgb(0.5, 0.5, 0.4),
                size: Vec2::new(20.0, 16.0),
            },
            LauncherType::StrawBlowgun => LauncherStats {
                cooldown_secs: 0.12,
                speed_multiplier: 2.5,
                range_multiplier: 1.2,
                damage_multiplier: 0.15,
                uses: 50,
                color: Color::srgb(0.9, 0.9, 0.5),
                size: Vec2::new(20.0, 6.0),
            },
        }
    }

    pub const ALL: &[LauncherType] = &[
        LauncherType::Slingshot,
        LauncherType::KetchupGun,
        LauncherType::SporkLauncher,
        LauncherType::LunchTrayCatapult,
        LauncherType::StrawBlowgun,
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

/// Launcher spawn point positions.
const LAUNCHER_SPAWN_POSITIONS: &[(f32, f32)] = &[
    (0.0, 0.0),
    (-300.0, 200.0),
    (300.0, -200.0),
];

/// Spawn launcher pickups on the map.
pub fn setup_launcher_spawns(mut commands: Commands) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for &(x, y) in LAUNCHER_SPAWN_POSITIONS {
        let launcher_type = LauncherType::ALL[rng.gen_range(0..LauncherType::ALL.len())];
        let stats = launcher_type.stats();

        commands.spawn((
            Sprite {
                color: stats.color,
                custom_size: Some(stats.size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.6),
            LauncherPickup { launcher_type },
            Gameplay,
        ));
    }
}

const PICKUP_RANGE: f32 = 40.0;

/// Pickup system for launchers: player presses pickup key near a launcher.
pub fn launcher_pickup_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    players: Query<(Entity, &Transform, &InputScheme, Option<&EquippedLauncher>)>,
    launchers: Query<(Entity, &Transform, &LauncherPickup)>,
) {
    for (player_entity, player_tf, scheme, equipped) in &players {
        // Already has a launcher
        if equipped.is_some() {
            continue;
        }

        let pickup_pressed = match scheme {
            InputScheme::KeyboardMouse => keyboard.just_pressed(KeyCode::KeyQ),
            InputScheme::ArrowKeys => keyboard.just_pressed(KeyCode::ControlRight),
        };

        if !pickup_pressed {
            continue;
        }

        // Find nearest launcher in range
        let mut nearest: Option<(Entity, f32, LauncherType)> = None;
        for (launcher_entity, launcher_tf, pickup) in &launchers {
            let dist = player_tf
                .translation
                .truncate()
                .distance(launcher_tf.translation.truncate());
            if dist < PICKUP_RANGE {
                if nearest.is_none() || dist < nearest.as_ref().unwrap().1 {
                    nearest = Some((launcher_entity, dist, pickup.launcher_type));
                }
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
        }
    }
}

/// Fire system for launchers. Overrides normal throw when launcher is equipped.
/// Handles: Slingshot, SporkLauncher, KetchupGun (rapid fire), StrawBlowgun (rapid fire).
/// LunchTrayCatapult uses charge system separately.
pub fn launcher_fire_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut players: Query<(
        Entity,
        &Transform,
        &InputScheme,
        &mut EquippedLauncher,
        &PlayerVelocity,
        Option<&ChargingShot>,
    )>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };

    for (player_entity, player_tf, scheme, mut launcher, player_vel, _charging) in &mut players {
        launcher.cooldown_timer.tick(time.delta());

        // Skip catapult — handled by charge system
        if launcher.launcher_type == LauncherType::LunchTrayCatapult {
            continue;
        }

        let (should_fire, aim_direction) = compute_fire_input(
            scheme,
            &keyboard,
            &mouse_buttons,
            window,
            camera,
            camera_transform,
            player_tf,
            player_vel,
        );

        // For rapid-fire weapons (ketchup, blowgun), fire while held
        let is_rapid = matches!(
            launcher.launcher_type,
            LauncherType::KetchupGun | LauncherType::StrawBlowgun
        );
        let trigger = if is_rapid {
            match scheme {
                InputScheme::KeyboardMouse => mouse_buttons.pressed(MouseButton::Left),
                InputScheme::ArrowKeys => keyboard.pressed(KeyCode::Enter),
            }
        } else {
            should_fire
        };

        if !trigger || aim_direction == Vec2::ZERO || !launcher.cooldown_timer.finished() {
            continue;
        }

        if launcher.uses_remaining == 0 {
            // Launcher is empty, remove it
            commands.entity(player_entity).remove::<EquippedLauncher>();
            continue;
        }

        let stats = launcher.launcher_type.stats();

        // Base projectile stats (launchers fire built-in ammo)
        let base_damage = 10.0 * stats.damage_multiplier;
        let base_speed = 300.0 * stats.speed_multiplier;
        let base_range = 400.0 * stats.range_multiplier;

        commands.spawn((
            Sprite {
                color: stats.color.with_alpha(0.8),
                custom_size: Some(Vec2::new(6.0, 6.0)),
                ..default()
            },
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

        launcher.uses_remaining -= 1;
        launcher.cooldown_timer.reset();
    }
}

/// Charge system for the LunchTrayCatapult.
pub fn catapult_charge_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut players: Query<(
        Entity,
        &Transform,
        &InputScheme,
        &mut EquippedLauncher,
        &PlayerVelocity,
        Option<&mut ChargingShot>,
    )>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };

    for (player_entity, player_tf, scheme, mut launcher, player_vel, mut charging) in &mut players {
        if launcher.launcher_type != LauncherType::LunchTrayCatapult {
            continue;
        }

        launcher.cooldown_timer.tick(time.delta());

        let holding = match scheme {
            InputScheme::KeyboardMouse => mouse_buttons.pressed(MouseButton::Left),
            InputScheme::ArrowKeys => keyboard.pressed(KeyCode::Enter),
        };
        let just_released = match scheme {
            InputScheme::KeyboardMouse => mouse_buttons.just_released(MouseButton::Left),
            InputScheme::ArrowKeys => keyboard.just_released(KeyCode::Enter),
        };

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
            // Get charge amount before removing
            let charge_fraction = if let Some(ref charge) = charging {
                charge.charge_time / charge.max_charge
            } else {
                0.0
            };

            commands.entity(player_entity).remove::<ChargingShot>();

            if charge_fraction > 0.05 && launcher.uses_remaining > 0 {
                let (_, aim_direction) = compute_fire_input(
                    scheme,
                    &keyboard,
                    &mouse_buttons,
                    window,
                    camera,
                    camera_transform,
                    player_tf,
                    player_vel,
                );

                let stats = launcher.launcher_type.stats();
                let base_damage = 10.0 * stats.damage_multiplier * (0.5 + charge_fraction * 1.5);
                let base_speed = 200.0 * stats.speed_multiplier * (0.5 + charge_fraction);
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

                // Arc trajectory for catapult
                proj.insert(ArcFlight {
                    vertical_velocity: 200.0 * (0.5 + charge_fraction),
                    gravity: 300.0,
                    simulated_z: 0.0,
                });

                launcher.uses_remaining -= 1;
                launcher.cooldown_timer.reset();
            }
        }
    }
}

/// Shared helper to compute fire button state and aim direction.
fn compute_fire_input(
    scheme: &InputScheme,
    keyboard: &ButtonInput<KeyCode>,
    mouse_buttons: &ButtonInput<MouseButton>,
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    player_tf: &Transform,
    player_vel: &PlayerVelocity,
) -> (bool, Vec2) {
    match scheme {
        InputScheme::KeyboardMouse => {
            let fire = mouse_buttons.just_pressed(MouseButton::Left);
            let dir = if let Some(cursor_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    (world_pos - player_tf.translation.truncate()).normalize_or_zero()
                } else {
                    Vec2::Y
                }
            } else {
                Vec2::Y
            };
            (fire, dir)
        }
        InputScheme::ArrowKeys => {
            let fire = keyboard.just_pressed(KeyCode::Enter);
            let dir = if player_vel.0.length_squared() > 0.01 {
                player_vel.0.normalize()
            } else {
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
            (fire, dir)
        }
    }
}
