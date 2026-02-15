use bevy::prelude::*;
use rand::Rng;

use super::components::*;

/// Moves NPCs along their patrol waypoints (excludes wandering NPCs).
pub fn patrol_system(
    time: Res<Time>,
    mut npcs: Query<
        (&NpcAuthority, &mut NpcState, &PatrolPath, &mut Transform, &mut Facing),
        Without<WanderZone>,
    >,
) {
    for (npc, mut state, path, mut transform, mut facing) in &mut npcs {
        let NpcState::Patrolling { waypoint_index } = state.as_ref() else {
            continue;
        };

        if path.waypoints.is_empty() {
            continue;
        }

        let target = path.waypoints[*waypoint_index];
        let current = transform.translation.truncate();
        let to_target = target - current;
        let distance = to_target.length();

        if distance < 5.0 {
            // Reached waypoint, advance to next
            let next_index = (*waypoint_index + 1) % path.waypoints.len();
            *state = NpcState::Patrolling {
                waypoint_index: next_index,
            };
        } else {
            let direction = to_target.normalize();
            facing.0 = direction;
            let movement = direction * npc.move_speed * time.delta_secs();
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

/// Returns patrol NPCs to their nearest waypoint (excludes wandering NPCs).
pub fn returning_system(
    time: Res<Time>,
    mut npcs: Query<
        (&NpcAuthority, &mut NpcState, &PatrolPath, &mut Transform, &mut Facing),
        Without<WanderZone>,
    >,
) {
    for (npc, mut state, path, mut transform, mut facing) in &mut npcs {
        let NpcState::Returning { waypoint_index } = state.as_ref() else {
            continue;
        };

        if path.waypoints.is_empty() {
            *state = NpcState::Patrolling { waypoint_index: 0 };
            continue;
        }

        let target = path.waypoints[*waypoint_index];
        let current = transform.translation.truncate();
        let to_target = target - current;
        let distance = to_target.length();

        if distance < 10.0 {
            *state = NpcState::Patrolling {
                waypoint_index: *waypoint_index,
            };
        } else {
            let direction = to_target.normalize();
            facing.0 = direction;
            let movement = direction * npc.move_speed * 0.8 * time.delta_secs();
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

/// Wandering NPCs pick random nearby targets within their zone and meander toward them.
pub fn wander_system(
    time: Res<Time>,
    mut npcs: Query<(
        &NpcAuthority,
        &mut NpcState,
        &WanderZone,
        &mut WanderTarget,
        &mut Transform,
        &mut Facing,
    )>,
) {
    let mut rng = rand::thread_rng();

    for (npc, mut state, zone, mut target, mut transform, mut facing) in &mut npcs {
        // Handle returning: go back to zone center, then resume wandering
        if matches!(state.as_ref(), NpcState::Returning { .. }) {
            let current = transform.translation.truncate();
            let to_center = zone.center - current;
            let distance = to_center.length();

            if distance < 15.0 {
                target.0 = pick_wander_point(&mut rng, zone);
                *state = NpcState::Patrolling { waypoint_index: 0 };
            } else {
                let direction = to_center.normalize();
                facing.0 = direction;
                let movement = direction * npc.move_speed * 0.8 * time.delta_secs();
                transform.translation.x += movement.x;
                transform.translation.y += movement.y;
            }
            continue;
        }

        if !matches!(state.as_ref(), NpcState::Patrolling { .. }) {
            continue;
        }

        let current = transform.translation.truncate();
        let to_target = target.0 - current;
        let distance = to_target.length();

        if distance < 8.0 {
            // Reached target, pick a new random point in zone
            target.0 = pick_wander_point(&mut rng, zone);
        } else {
            let direction = to_target.normalize();
            facing.0 = direction;
            let movement = direction * npc.move_speed * time.delta_secs();
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

fn pick_wander_point(rng: &mut impl Rng, zone: &WanderZone) -> Vec2 {
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let dist = rng.gen_range(0.0..zone.radius);
    zone.center + Vec2::new(angle.cos(), angle.sin()) * dist
}
