use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Lobby,
    MapSelect,
    Loading,
    Playing,
    Paused,
    RoundOver,
}

/// Marker component for entities that should be despawned when leaving gameplay.
#[derive(Component)]
pub struct Gameplay;
