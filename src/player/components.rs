//! Player entity components.
//!
//! Core bundle: `Player` + `Health` + `Velocity` + `Score` + `ControllerLink` +
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

#[derive(Component)]
pub struct Score(#[allow(dead_code)] pub i32); // planned: scoring UI not yet wired up

/// Links this player to a specific controller for input.
#[derive(Component)]
pub struct ControllerLink(pub ControllerId);
