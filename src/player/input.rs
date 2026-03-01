use bevy::prelude::*;

use crate::input::ControllerInput;
use crate::npc::components::Caught;

use super::components::{Eliminated, Player, Velocity};

/// Reads controller input and sets player velocity.
/// Stunned and eliminated players can't move.
pub fn input_system(
    mut query: Query<(&Player, &ControllerInput, &mut Velocity, Option<&Caught>, Option<&Eliminated>)>,
) {
    for (player, input, mut velocity, caught, eliminated) in &mut query {
        if caught.is_some() || eliminated.is_some() {
            velocity.0 = Vec2::ZERO;
            continue;
        }

        velocity.0 = input.move_stick * player.speed;
    }
}
