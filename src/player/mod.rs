pub mod animation;
pub mod components;
pub mod input;
pub mod movement;

use bevy::prelude::*;

use crate::food::components::Inventory;
use crate::states::{GameState, Gameplay};
use components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                input::input_system,
                movement::movement_system,
                animation::animation_system,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::Playing), spawn_players);
    }
}

/// Spawn two players with placeholder colored rectangle sprites.
fn spawn_players(mut commands: Commands) {
    // Player 1: Blue, WASD controls, spawns left side
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.4, 0.9),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(-100.0, 0.0, 1.0),
        Player {
            id: 1,
            speed: 200.0,
        },
        Health(100.0),
        Velocity(Vec2::ZERO),
        Score(0),
        InputScheme::KeyboardMouse,
        Inventory { held_food: None },
        Gameplay,
    ));

    // Player 2: Red, Arrow key controls, spawns right side
    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.2, 0.2),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 1.0),
        Player {
            id: 2,
            speed: 200.0,
        },
        Health(100.0),
        Velocity(Vec2::ZERO),
        Score(0),
        InputScheme::ArrowKeys,
        Inventory { held_food: None },
        Gameplay,
    ));
}
