use bevy::prelude::*;

use crate::food::components::Blocking;
use crate::input::ControllerInput;
use crate::npc::components::Caught;

use super::components::{Eliminated, Player, Velocity};

/// Reads controller input and sets player velocity.
/// Stunned and eliminated players can't move.
/// Blocking players (LunchTray held) move at 25% speed.
pub fn input_system(
    mut query: Query<(&Player, &ControllerInput, &mut Velocity, Option<&Caught>, Option<&Eliminated>, Option<&Blocking>)>,
) {
    for (player, input, mut velocity, caught, eliminated, blocking) in &mut query {
        if caught.is_some() || eliminated.is_some() {
            velocity.0 = Vec2::ZERO;
            continue;
        }

        let speed_scale = if blocking.is_some() { 0.25 } else { 1.0 };
        velocity.0 = input.move_stick * player.speed * speed_scale;
    }
}
