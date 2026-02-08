use bevy::prelude::*;

use crate::npc::components::Caught;

use super::components::{InputScheme, Player, Velocity};

/// Reads input and sets player velocity based on their input scheme.
/// Stunned players can't move.
pub fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &InputScheme, &mut Velocity, Option<&Caught>)>,
) {
    for (player, scheme, mut velocity, caught) in &mut query {
        // Stunned players can't move
        if caught.is_some() {
            velocity.0 = Vec2::ZERO;
            continue;
        }
        let mut direction = Vec2::ZERO;

        match scheme {
            InputScheme::KeyboardMouse => {
                if keyboard.pressed(KeyCode::KeyW) {
                    direction.y += 1.0;
                }
                if keyboard.pressed(KeyCode::KeyS) {
                    direction.y -= 1.0;
                }
                if keyboard.pressed(KeyCode::KeyA) {
                    direction.x -= 1.0;
                }
                if keyboard.pressed(KeyCode::KeyD) {
                    direction.x += 1.0;
                }
            }
            InputScheme::ArrowKeys => {
                if keyboard.pressed(KeyCode::ArrowUp) {
                    direction.y += 1.0;
                }
                if keyboard.pressed(KeyCode::ArrowDown) {
                    direction.y -= 1.0;
                }
                if keyboard.pressed(KeyCode::ArrowLeft) {
                    direction.x -= 1.0;
                }
                if keyboard.pressed(KeyCode::ArrowRight) {
                    direction.x += 1.0;
                }
            }
        }

        // Normalize so diagonal movement isn't faster
        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }

        velocity.0 = direction * player.speed;
    }
}
