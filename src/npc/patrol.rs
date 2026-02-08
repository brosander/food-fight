use bevy::prelude::*;

use super::components::*;

/// Moves NPCs along their patrol waypoints.
pub fn patrol_system(
    time: Res<Time>,
    mut npcs: Query<(&NpcAuthority, &mut NpcState, &PatrolPath, &mut Transform, &mut Facing)>,
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

/// Returns NPCs to their nearest patrol waypoint.
pub fn returning_system(
    time: Res<Time>,
    mut npcs: Query<(&NpcAuthority, &mut NpcState, &PatrolPath, &mut Transform, &mut Facing)>,
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
