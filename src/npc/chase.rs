use bevy::prelude::*;

use super::components::{Caught, Enraged, Facing, NpcAuthority, NpcRole, NpcState};
use crate::audio::SoundEvent;
use crate::food::components::Inventory;
use crate::food::launcher::EquippedLauncher;
use crate::player::components::{Eliminated, Player};

const PLAYER_SPEED: f32 = 200.0;

/// Chasing NPC moves toward target player.
pub fn chase_system(
    time: Res<Time>,
    mut npcs: Query<(&NpcAuthority, &NpcState, &mut Transform, &mut Facing, Option<&Enraged>)>,
    players: Query<&Transform, (With<Player>, Without<NpcAuthority>, Without<Eliminated>)>,
) {
    for (npc, state, mut npc_tf, mut facing, enraged) in &mut npcs {
        let NpcState::Chasing { target } = state else {
            continue;
        };

        let Ok(player_tf) = players.get(*target) else {
            continue;
        };

        let to_player = player_tf.translation.truncate() - npc_tf.translation.truncate();
        let dist = to_player.length();

        if dist > 1.0 {
            let direction = to_player.normalize();
            facing.0 = direction;
            let speed = if npc.role == NpcRole::Teacher && enraged.is_some() {
                PLAYER_SPEED * 1.2 // 240 px/s — 20% faster than a student
            } else {
                npc.move_speed * 1.3
            };
            let movement = direction * speed * time.delta_secs();
            npc_tf.translation.x += movement.x;
            npc_tf.translation.y += movement.y;
        }
    }
}

/// When chasing NPC gets close enough, catch the player.
pub fn catch_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    npcs: Query<(Entity, &NpcAuthority, &NpcState, &Transform)>,
    players: Query<(Entity, &Transform, Option<&Caught>), (With<Player>, Without<Eliminated>)>,
) {
    for (npc_entity, npc, state, npc_tf) in &npcs {
        let NpcState::Chasing { target } = state else {
            continue;
        };

        let Ok((player_entity, player_tf, already_caught)) = players.get(*target) else {
            continue;
        };

        if already_caught.is_some() {
            continue;
        }

        let dist = npc_tf
            .translation
            .truncate()
            .distance(player_tf.translation.truncate());

        if dist < npc.catch_radius {
            let stun_duration = match npc.role {
                NpcRole::Teacher => 3.0,
                NpcRole::Principal => 5.0,
                NpcRole::LunchLady => 2.0,
                NpcRole::Janitor => 0.0,
            };

            if stun_duration > 0.0 {
                commands.entity(player_entity).insert(Caught {
                    stun_timer: Timer::from_seconds(stun_duration, TimerMode::Once),
                });
                sound.send(SoundEvent::PlayerCaught);
                // Cool down: enrage ends once the teacher catches their target
                commands.entity(npc_entity).remove::<Enraged>();
            }
        }
    }
}

/// Caught penalty: stuns player, drops items. Removes Caught when timer expires.
pub fn caught_penalty_system(
    mut commands: Commands,
    time: Res<Time>,
    mut caught_players: Query<(Entity, &mut Caught, &mut Inventory), With<Player>>,
) {
    for (entity, mut caught, mut inventory) in &mut caught_players {
        caught.stun_timer.tick(time.delta());

        // Drop held food on first frame of being caught
        if caught.stun_timer.elapsed_secs() < time.delta_secs() * 2.0 {
            inventory.held_food = None;
            commands.entity(entity).remove::<EquippedLauncher>();
        }

        if caught.stun_timer.finished() {
            commands.entity(entity).remove::<Caught>();
        }
    }
}
