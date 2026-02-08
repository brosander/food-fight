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

/// Tracks which input scheme this player uses.
#[derive(Component)]
pub enum InputScheme {
    /// WASD + mouse aim
    KeyboardMouse,
    /// Arrow keys + right shift to throw (for local 2P)
    ArrowKeys,
}
