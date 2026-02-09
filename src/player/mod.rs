pub mod components;
pub mod input;
pub mod movement;

use bevy::prelude::*;

use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                input::input_system,
                movement::movement_system,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
        // Player spawning is handled by LobbyPlugin::spawn_players_from_lobby
        // Player animation is handled by SpritePlugin
    }
}
