use bevy::prelude::*;

use super::components::*;
use crate::audio::SoundEvent;
use crate::input::ControllerInput;
use crate::player::components::{Eliminated, Health, Player};
use crate::score::{CumulativeScores, RoundScores};
use crate::sprites::{
    AnimationState, FrameRange, SpriteAssets, effects_atlas_index,
};
use crate::states::Gameplay;

const PICKUP_RANGE: f32 = 70.0;
const PARRY_WINDOW_SECS: f32 = 0.2;
const BAGUETTE_SWING_DAMAGE: f32 = 20.0;
const BAGUETTE_SWING_RANGE: f32 = 50.0;
const BAGUETTE_SWING_COOLDOWN: f32 = 0.7;
const BAGUETTE_USES: u32 = 15;
const LUNCH_TRAY_USES: u32 = 999; // effectively unlimited blocks

/// West button picks up the nearest melee weapon. Drops the existing one if held.
pub fn melee_pickup_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    mut players: Query<
        (
            Entity,
            &Transform,
            &ControllerInput,
            Option<&EquippedMeleeWeapon>,
        ),
        Without<Eliminated>,
    >,
    pickups: Query<(Entity, &Transform, &MeleeWeaponPickup)>,
) {
    for (player_entity, player_tf, input, equipped) in &mut players {
        if !input.pickup_launcher.just_pressed {
            continue;
        }

        let player_pos = player_tf.translation.truncate();

        let mut nearest: Option<(Entity, f32, MeleeWeaponType)> = None;
        for (pickup_entity, pickup_tf, pickup) in &pickups {
            let dist = player_pos.distance(pickup_tf.translation.truncate());
            if dist < PICKUP_RANGE && (nearest.is_none() || dist < nearest.as_ref().unwrap().1) {
                nearest = Some((pickup_entity, dist, pickup.weapon_type));
            }
        }

        let Some((pickup_entity, _, weapon_type)) = nearest else { continue };

        // Destroy existing melee weapon
        if equipped.is_some() {
            commands
                .entity(player_entity)
                .remove::<EquippedMeleeWeapon>()
                .remove::<ParryWindow>()
                .remove::<Blocking>();
        }

        let cooldown = match weapon_type {
            MeleeWeaponType::Baguette => BAGUETTE_SWING_COOLDOWN,
            MeleeWeaponType::LunchTray => 0.0,
        };
        let uses = match weapon_type {
            MeleeWeaponType::Baguette => BAGUETTE_USES,
            MeleeWeaponType::LunchTray => LUNCH_TRAY_USES,
        };

        commands.entity(pickup_entity).despawn();
        commands.entity(player_entity).insert(EquippedMeleeWeapon {
            weapon_type,
            swing_cooldown: Timer::from_seconds(cooldown, TimerMode::Once),
            uses_remaining: uses,
            swinging: false,
            swing_facing: Vec2::Y,
        });
        sound.send(SoundEvent::LauncherPickup);
    }
}

/// Manages ParryWindow / Blocking state transitions for LunchTray holders.
/// Also removes both components when the weapon is no longer a LunchTray or R1 is released.
pub fn melee_block_system(
    mut commands: Commands,
    time: Res<Time>,
    mut players: Query<
        (
            Entity,
            &ControllerInput,
            Option<&EquippedMeleeWeapon>,
            Option<&mut ParryWindow>,
            Option<&Blocking>,
        ),
        Without<Eliminated>,
    >,
) {
    for (player_entity, input, equipped, parry_window, blocking) in &mut players {
        let is_tray = matches!(
            equipped.map(|e| e.weapon_type),
            Some(MeleeWeaponType::LunchTray)
        );

        // Clear state if no longer holding a tray or R1 released
        if !is_tray || input.melee.just_released {
            if parry_window.is_some() || blocking.is_some() {
                commands
                    .entity(player_entity)
                    .remove::<ParryWindow>()
                    .remove::<Blocking>();
            }
            continue;
        }

        if input.melee.just_pressed {
            commands.entity(player_entity).insert(ParryWindow {
                timer: Timer::from_seconds(PARRY_WINDOW_SECS, TimerMode::Once),
            });
        }

        if let Some(mut pw) = parry_window {
            pw.timer.tick(time.delta());
            if pw.timer.finished() && input.melee.pressed {
                commands
                    .entity(player_entity)
                    .remove::<ParryWindow>()
                    .insert(Blocking);
            }
        }
    }
}

/// Melee swing for Baguette holders: R1 just_pressed deals damage to nearby players in front arc.
pub fn baguette_swing_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    time: Res<Time>,
    sprite_assets: Res<SpriteAssets>,
    mut attackers: Query<
        (Entity, &Transform, &ControllerInput, &Player, &mut EquippedMeleeWeapon),
        Without<Eliminated>,
    >,
    mut targets: Query<
        (Entity, &Transform, &Player, &mut Health, Option<&Eliminated>),
    >,
    mut round: ResMut<RoundScores>,
    mut scores: ResMut<CumulativeScores>,
) {
    for (attacker_entity, attacker_tf, input, attacker_player, mut weapon) in &mut attackers {
        if weapon.weapon_type != MeleeWeaponType::Baguette {
            continue;
        }

        weapon.swing_cooldown.tick(time.delta());

        if !input.melee.just_pressed || !weapon.swing_cooldown.finished() {
            continue;
        }

        if weapon.uses_remaining == 0 {
            commands.entity(attacker_entity).remove::<EquippedMeleeWeapon>();
            continue;
        }

        let facing = {
            let s = input.move_stick;
            if s != Vec2::ZERO { s.normalize() } else { Vec2::Y }
        };
        let attacker_pos = attacker_tf.translation.truncate();

        for (target_entity, target_tf, _, mut health, eliminated) in &mut targets {
            if target_entity == attacker_entity || eliminated.is_some() {
                continue;
            }

            let to_target = target_tf.translation.truncate() - attacker_pos;
            let dist = to_target.length();
            if dist > BAGUETTE_SWING_RANGE {
                continue;
            }

            // Forward 120° arc: dot > 0.5
            if dist > 0.0 && to_target.normalize().dot(facing) < 0.5 {
                continue;
            }

            let actual_damage = BAGUETTE_SWING_DAMAGE.min(health.0);
            health.0 = (health.0 - BAGUETTE_SWING_DAMAGE).max(0.0);
            sound.send(SoundEvent::FoodHit);

            let idx = (attacker_player.id - 1) as usize;
            round.add_damage(idx, actual_damage);
            scores.add_damage(idx, actual_damage);

            // Spawn hit flash on target
            commands.spawn((
                Sprite {
                    image: sprite_assets.effects_image.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprite_assets.effects_layout.clone(),
                        index: effects_atlas_index(0, 0),
                    }),
                    custom_size: Some(Vec2::new(24.0, 24.0)),
                    ..default()
                },
                Transform::from_xyz(
                    target_tf.translation.x,
                    target_tf.translation.y,
                    3.0,
                ),
                AnimationState::new(
                    "hit_flash",
                    FrameRange {
                        start: effects_atlas_index(0, 0),
                        end: effects_atlas_index(0, 5),
                        fps: 15.0,
                        looping: false,
                    },
                ),
                SplatEffect {
                    lifetime: Timer::from_seconds(0.4, TimerMode::Once),
                },
                Gameplay,
            ));

            // Eliminate if health reached zero
            if health.0 == 0.0 {
                commands
                    .entity(target_entity)
                    .insert(Eliminated)
                    .insert(Inventory { held_food: None })
                    .remove::<crate::food::launcher::EquippedLauncher>()
                    .remove::<crate::food::launcher::ChargingShot>()
                    .remove::<EquippedMeleeWeapon>()
                    .remove::<ParryWindow>()
                    .remove::<Blocking>();
                round.add_detention(idx);
                scores.add_detention(idx);
            }
        }

        weapon.swinging = true;
        weapon.swing_facing = facing;
        weapon.swing_cooldown.reset();
        weapon.uses_remaining -= 1;
        if weapon.uses_remaining == 0 {
            commands.entity(attacker_entity).remove::<EquippedMeleeWeapon>();
        }
    }
}

