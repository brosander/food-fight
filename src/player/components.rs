//! Player entity components.
//!
//! Core bundle: `Player` + `Health` + `Velocity` + `ControllerLink` +
//! `ControllerInput`. `ControllerLink` binds the entity to a specific controller;
//! `ControllerInput` is populated each frame by the active input backend.

use bevy::prelude::*;

use crate::input::ControllerId;

#[derive(Component)]
pub struct Player {
    pub id: u8,
    pub speed: f32,
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Links this player to a specific controller for input.
#[derive(Component)]
pub struct ControllerLink(pub ControllerId);

/// Marker for a player who has been eliminated and banished to a corner detention table.
#[derive(Component)]
pub struct Eliminated;

/// World positions of the 4 corner detention tables, indexed by player.id - 1.
/// Order: bottom-left, bottom-right, top-left, top-right.
pub const DETENTION_CORNERS: [Vec2; 4] = [
    Vec2::new(-400.0, -260.0),
    Vec2::new(400.0, -260.0),
    Vec2::new(-400.0, 260.0),
    Vec2::new(400.0, 260.0),
];
