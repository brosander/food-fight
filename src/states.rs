use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    MapSelect,
    Loading,
    Playing,
    Paused,
    RoundOver,
}
