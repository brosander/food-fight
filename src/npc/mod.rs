pub mod chase;
pub mod components;
pub mod detection;
pub mod patrol;

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::states::{GameState, Gameplay};
use components::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_npcs)
            .add_systems(
                FixedUpdate,
                (
                    detection::suspicion_system,
                    detection::detection_system,
                    patrol::patrol_system,
                    patrol::returning_system,
                    chase::chase_system,
                    chase::catch_system,
                    chase::caught_penalty_system,
                    npc_visual_feedback_system,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_npcs(mut commands: Commands) {
    // Teacher: patrols between tables
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.2),
            custom_size: Some(Vec2::new(28.0, 28.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 1.5),
        NpcAuthority {
            role: NpcRole::Teacher,
            detection_radius: 150.0,
            detection_angle: PI / 3.0,
            move_speed: 80.0,
            catch_radius: 30.0,
        },
        NpcState::Patrolling { waypoint_index: 0 },
        PatrolPath {
            waypoints: vec![
                Vec2::new(-200.0, 200.0),
                Vec2::new(200.0, 200.0),
                Vec2::new(200.0, -200.0),
                Vec2::new(-200.0, -200.0),
            ],
        },
        Facing(Vec2::Y),
        Gameplay,
    ));

    // Principal: slower, wider detection, patrols center area
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.6),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.5),
        NpcAuthority {
            role: NpcRole::Principal,
            detection_radius: 200.0,
            detection_angle: PI / 2.0,
            move_speed: 60.0,
            catch_radius: 35.0,
        },
        NpcState::Patrolling { waypoint_index: 0 },
        PatrolPath {
            waypoints: vec![
                Vec2::new(0.0, 200.0),
                Vec2::new(300.0, 0.0),
                Vec2::new(0.0, -200.0),
                Vec2::new(-300.0, 0.0),
            ],
        },
        Facing(Vec2::Y),
        Gameplay,
    ));

    // Lunch Lady: stationary near counter
    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.5, 0.7),
            custom_size: Some(Vec2::new(28.0, 28.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 240.0, 1.5),
        NpcAuthority {
            role: NpcRole::LunchLady,
            detection_radius: 80.0,
            detection_angle: PI, // Full front hemisphere
            move_speed: 0.0,    // Stationary
            catch_radius: 40.0,
        },
        NpcState::Patrolling { waypoint_index: 0 },
        PatrolPath {
            waypoints: vec![Vec2::new(0.0, 240.0)],
        },
        Facing(-Vec2::Y),
        Gameplay,
    ));
}

/// Visual feedback for NPC state: change color/brightness based on state.
fn npc_visual_feedback_system(mut npcs: Query<(&NpcState, &NpcAuthority, &mut Sprite)>) {
    for (state, npc, mut sprite) in &mut npcs {
        let base_color = match npc.role {
            NpcRole::Teacher => Color::srgb(0.8, 0.6, 0.2),
            NpcRole::Principal => Color::srgb(0.3, 0.3, 0.6),
            NpcRole::LunchLady => Color::srgb(0.9, 0.5, 0.7),
            NpcRole::Janitor => Color::srgb(0.4, 0.6, 0.4),
        };

        sprite.color = match state {
            NpcState::Patrolling { .. } => base_color,
            NpcState::Suspicious { .. } => Color::srgb(1.0, 0.9, 0.0), // Yellow alert
            NpcState::Chasing { .. } => Color::srgb(1.0, 0.2, 0.2),   // Red chase
            NpcState::Returning { .. } => base_color,
        };
    }
}
