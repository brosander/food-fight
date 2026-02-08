use bevy::prelude::*;

use crate::controller::read_left_stick;
use crate::npc::components::Caught;

use super::components::{GamepadLink, Player, Velocity};

/// Reads gamepad left stick and sets player velocity.
/// Stunned players can't move.
pub fn input_system(
    gamepads: Query<&Gamepad>,
    mut query: Query<(&Player, &GamepadLink, &mut Velocity, Option<&Caught>)>,
) {
    for (player, link, mut velocity, caught) in &mut query {
        if caught.is_some() {
            velocity.0 = Vec2::ZERO;
            continue;
        }

        let direction = if let Ok(gamepad) = gamepads.get(link.0) {
            read_left_stick(gamepad)
        } else {
            Vec2::ZERO
        };

        velocity.0 = direction * player.speed;
    }
}
