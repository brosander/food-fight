use bevy::prelude::*;

use crate::audio::SoundEvent;
use crate::food::components::*;
use crate::food::launcher::{ChargingShot, EquippedLauncher};
use crate::player::components::{Eliminated, Health, Player};
use crate::score::CumulativeScores;
use crate::sprites::{AnimationState, FrameRange, SpriteAssets, effects_atlas_index};
use crate::states::Gameplay;

const PLAYER_HALF_SIZE: f32 = 16.0;

/// AABB collision between InFlight food and Player entities.
/// Skips the thrower and already-eliminated players.
/// When a player's health hits zero they are marked Eliminated and banished to their corner.
/// Accumulates damage dealt and detention slips into `CumulativeScores` keyed by attacker.
pub fn food_player_collision_system(
    mut commands: Commands,
    mut sound: EventWriter<SoundEvent>,
    projectiles: Query<(Entity, &Transform, &InFlight)>,
    mut players: Query<(Entity, &Transform, &mut Health, Option<&Eliminated>), With<Player>>,
    attackers: Query<&Player>,
    mut scores: ResMut<CumulativeScores>,
    sprite_assets: Res<SpriteAssets>,
) {
    for (proj_entity, proj_tf, flight) in &projectiles {
        let proj_pos = proj_tf.translation.truncate();
        let proj_half = Vec2::new(6.0, 6.0);

        for (player_entity, player_tf, mut health, eliminated) in &mut players {
            if player_entity == flight.thrown_by {
                continue;
            }

            // Eliminated players can't be hit
            if eliminated.is_some() {
                continue;
            }

            let player_pos = player_tf.translation.truncate();

            if aabb_overlap(
                proj_pos,
                proj_half,
                player_pos,
                Vec2::splat(PLAYER_HALF_SIZE),
            ) {
                // Clamp actual damage to remaining health so score is honest
                let actual_damage = flight.damage.min(health.0);
                health.0 = (health.0 - flight.damage).max(0.0);
                sound.send(SoundEvent::FoodHit);

                // Credit the attacker's cumulative score
                if let Ok(attacker) = attackers.get(flight.thrown_by) {
                    let idx = (attacker.id - 1) as usize;
                    scores.add_damage(idx, actual_damage);
                }

                // Spawn hit flash animation
                let hit_start = effects_atlas_index(0, 0);
                commands.spawn((
                    Sprite {
                        image: sprite_assets.effects_image.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: sprite_assets.effects_layout.clone(),
                            index: hit_start,
                        }),
                        custom_size: Some(Vec2::new(24.0, 24.0)),
                        ..default()
                    },
                    Transform::from_xyz(
                        player_tf.translation.x,
                        player_tf.translation.y,
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

                commands.entity(proj_entity).despawn();

                // Eliminate the player if health just hit zero
                if health.0 == 0.0 {
                    commands
                        .entity(player_entity)
                        .insert(Eliminated)
                        .insert(Inventory { held_food: None })
                        .remove::<EquippedLauncher>()
                        .remove::<ChargingShot>();
                    // detention_system will snap them to their corner next tick

                    // Credit the attacker with a detention slip
                    if let Ok(attacker) = attackers.get(flight.thrown_by) {
                        scores.add_detention((attacker.id - 1) as usize);
                    }
                }

                break;
            }
        }
    }
}

fn aabb_overlap(pos_a: Vec2, half_a: Vec2, pos_b: Vec2, half_b: Vec2) -> bool {
    let diff = (pos_a - pos_b).abs();
    diff.x < half_a.x + half_b.x && diff.y < half_a.y + half_b.y
}
