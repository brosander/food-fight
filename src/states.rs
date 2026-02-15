//! Game state enum and session lifecycle marker components.

use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Lobby,
    #[allow(dead_code)]
    MapSelect, // planned: map selection screen
    #[allow(dead_code)]
    Loading, // planned: async asset loading screen
    Playing,
    Paused,
    RoundOver,
}

/// Marker component for entities that should be despawned when leaving gameplay.
#[derive(Component)]
pub struct Gameplay;

/// Marker resource: inserted on the first `OnEnter(Playing)` (from Lobby).
/// Prevents spawn systems from re-running when resuming from Paused.
/// Removed during cleanup when returning to MainMenu.
#[derive(Resource)]
pub struct GameSessionActive;
