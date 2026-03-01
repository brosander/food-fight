pub mod chase;
pub mod components;
pub mod detection;
pub mod patrol;

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::sprites::{AnimationState, FrameRange, SpriteAssets, atlas_index};
use crate::states::{GameSessionActive, GameState, Gameplay};
use components::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                OnEnter(GameState::Playing),
                spawn_npcs.run_if(not(resource_exists::<GameSessionActive>)),
            )
            .add_systems(
                FixedUpdate,
                (
                    detection::suspicion_system,
                    detection::teacher_launcher_alert_system,
                    detection::detection_system,
                    patrol::patrol_system,
                    patrol::returning_system,
                    patrol::wander_system,
                    chase::chase_system,
                    chase::catch_system,
                    chase::caught_penalty_system,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_npcs(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    // Teacher: wanders near starting area, chases students who throw food
    let teacher_start = Vec2::new(-200.0, 0.0);
    let teacher_idle = atlas_index(0, 6, 7); // patrol_idle
    commands.spawn((
        Sprite {
            image: sprite_assets.teacher_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprite_assets.teacher_layout.clone(),
                index: teacher_idle,
            }),
            custom_size: Some(Vec2::new(28.0, 28.0)),
            ..default()
        },
        Transform::from_xyz(teacher_start.x, teacher_start.y, 1.5),
        NpcAuthority {
            role: NpcRole::Teacher,
            detection_radius: 150.0,
            detection_angle: PI / 3.0,
            move_speed: 80.0,
            catch_radius: 30.0,
        },
        NpcState::Patrolling { waypoint_index: 0 },
        WanderZone {
            center: teacher_start,
            radius: 80.0,
        },
        WanderTarget(teacher_start),
        PatrolPath {
            waypoints: vec![teacher_start],
        },
        Facing(Vec2::Y),
        AnimationState::new(
            "patrol_idle",
            FrameRange {
                start: teacher_idle,
                end: teacher_idle,
                fps: 1.0,
                looping: true,
            },
        ),
        Gameplay,
    ));

    // Principal: slower, wider detection, patrols center area
    let principal_idle = atlas_index(0, 6, 7); // patrol_idle
    commands.spawn((
        Sprite {
            image: sprite_assets.principal_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprite_assets.principal_layout.clone(),
                index: principal_idle,
            }),
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
        AnimationState::new(
            "patrol_idle",
            FrameRange {
                start: principal_idle,
                end: principal_idle,
                fps: 1.0,
                looping: true,
            },
        ),
        Gameplay,
    ));

    // Lunch Lady: stationary near counter
    let ll_idle = atlas_index(0, 0, 6); // idle_stir frame 0
    commands.spawn((
        Sprite {
            image: sprite_assets.lunch_lady_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprite_assets.lunch_lady_layout.clone(),
                index: ll_idle,
            }),
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
        AnimationState::new(
            "idle_stir",
            FrameRange {
                start: ll_idle,
                end: atlas_index(0, 1, 6),
                fps: 2.0,
                looping: true,
            },
        ),
        Gameplay,
    ));
}
