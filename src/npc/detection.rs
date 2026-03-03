use bevy::prelude::*;

use super::components::*;
use crate::food::components::InFlight;
use crate::food::launcher::EquippedLauncher;
use crate::player::components::{Eliminated, Player};

/// Teachers immediately chase any player holding a launcher, bypassing cone/distance checks.
pub fn teacher_launcher_alert_system(
    mut npcs: Query<(&NpcAuthority, &mut NpcState, &Transform)>,
    players: Query<(Entity, &Transform), (With<Player>, With<EquippedLauncher>, Without<Eliminated>)>,
) {
    for (npc, mut state, npc_tf) in &mut npcs {
        if npc.role != NpcRole::Teacher {
            continue;
        }

        let target = players
            .iter()
            .min_by(|(_, a), (_, b)| {
                let da = a.translation.truncate().distance(npc_tf.translation.truncate());
                let db = b.translation.truncate().distance(npc_tf.translation.truncate());
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(e, _)| e);

        if let Some(target_entity) = target {
            *state = NpcState::Chasing { target: target_entity };
        }
    }
}

/// Marks players as Suspicious when they throw food, fire a launcher,
/// or are holding a weapon.
pub fn suspicion_system(
    mut commands: Commands,
    players: Query<(Entity, Option<&EquippedLauncher>), (With<Player>, Without<Eliminated>)>,
    projectiles: Query<&InFlight>,
) {
    for (player_entity, launcher) in &players {
        let has_launcher = launcher.is_some();
        let recently_threw = projectiles
            .iter()
            .any(|flight| flight.thrown_by == player_entity);

        if has_launcher || recently_threw {
            commands.entity(player_entity).insert(Suspicious);
        } else {
            commands.entity(player_entity).remove::<Suspicious>();
        }
    }
}

/// Detection cone check: sees if a Suspicious player is within NPC's FOV.
pub fn detection_system(
    time: Res<Time>,
    mut npcs: Query<(&NpcAuthority, &mut NpcState, &Transform, &Facing, &PatrolPath, Option<&Enraged>)>,
    players: Query<(Entity, &Transform), (With<Player>, With<Suspicious>, Without<Eliminated>)>,
) {
    for (npc, mut state, npc_tf, facing, path, enraged) in &mut npcs {
        match state.as_mut() {
            NpcState::Patrolling { .. } => {
                // Look for suspicious players in detection cone
                if find_visible_target(
                    npc_tf.translation.truncate(),
                    facing.0,
                    npc.detection_radius,
                    npc.detection_angle,
                    &players,
                )
                .is_some()
                {
                    *state = NpcState::Suspicious {
                        timer: Timer::from_seconds(2.0, TimerMode::Once),
                    };
                }
            }
            NpcState::Suspicious { ref mut timer } => {
                timer.tick(time.delta());

                // Check if we can still see a suspicious player
                if let Some((target, _pos)) = find_visible_target(
                    npc_tf.translation.truncate(),
                    facing.0,
                    npc.detection_radius,
                    npc.detection_angle * 1.5, // Wider cone when suspicious
                    &players,
                ) {
                    *state = NpcState::Chasing { target };
                } else if timer.finished() {
                    // Lost sight, return to patrol
                    let nearest = nearest_waypoint(npc_tf.translation.truncate(), path);
                    *state = NpcState::Returning {
                        waypoint_index: nearest,
                    };
                }
            }
            NpcState::Chasing { target } => {
                // Enraged teachers ignore detection rules — they know who hit them
                if enraged.is_some() {
                    continue;
                }

                // Check if we can still see the target (wider radius)
                let can_see = players.get(*target).is_ok_and(|(_, player_tf)| {
                    let dist = npc_tf
                        .translation
                        .truncate()
                        .distance(player_tf.translation.truncate());
                    dist < npc.detection_radius * 1.5
                });

                if !can_see {
                    let nearest = nearest_waypoint(npc_tf.translation.truncate(), path);
                    *state = NpcState::Returning {
                        waypoint_index: nearest,
                    };
                }
            }
            NpcState::Returning { .. } => {}
        }
    }
}

fn find_visible_target(
    npc_pos: Vec2,
    npc_facing: Vec2,
    radius: f32,
    half_angle: f32,
    players: &Query<(Entity, &Transform), (With<Player>, With<Suspicious>, Without<Eliminated>)>,
) -> Option<(Entity, Vec2)> {
    let mut closest: Option<(Entity, Vec2, f32)> = None;

    for (entity, player_tf) in players {
        let player_pos = player_tf.translation.truncate();
        let to_player = player_pos - npc_pos;
        let dist = to_player.length();

        if dist > radius {
            continue;
        }

        // Cone check: angle between facing direction and direction to player
        let angle = npc_facing.angle_to(to_player.normalize()).abs();
        if angle > half_angle {
            continue;
        }

        if closest.is_none() || dist < closest.as_ref().unwrap().2 {
            closest = Some((entity, player_pos, dist));
        }
    }

    closest.map(|(e, p, _)| (e, p))
}

fn nearest_waypoint(pos: Vec2, path: &PatrolPath) -> usize {
    path.waypoints
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.distance(pos)
                .partial_cmp(&b.distance(pos))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
        .unwrap_or(0)
}
