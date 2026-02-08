use bevy::prelude::*;

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
pub struct Score(pub i32);

/// Links this player to a specific gamepad entity for input.
#[derive(Component)]
pub struct GamepadLink(pub Entity);
