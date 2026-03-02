//! Cumulative scoring that persists across rounds in a session.
//!
//! `CumulativeScores` is a resource indexed by player index (0 = Player 1, etc.).
//! It is never removed mid-session — only reset when returning to the main menu.

use bevy::prelude::*;

/// Per-player cumulative stats for the current session.
#[derive(Default, Clone, Copy)]
pub struct PlayerScore {
    /// Total damage dealt to other players across all rounds.
    pub damage_dealt: f32,
    /// Number of players this player personally sent to lunch detention (eliminations caused).
    pub detention_slips: u32,
}

/// Cumulative scores for all players, indexed by player index (player.id - 1).
/// Survives round transitions; reset only on return to the main menu.
#[derive(Resource, Default)]
pub struct CumulativeScores {
    pub entries: [PlayerScore; 4],
}

impl CumulativeScores {
    pub fn reset(&mut self) {
        self.entries = [PlayerScore::default(); 4];
    }

    pub fn add_damage(&mut self, player_idx: usize, damage: f32) {
        if player_idx < 4 {
            self.entries[player_idx].damage_dealt += damage;
        }
    }

    pub fn add_detention(&mut self, player_idx: usize) {
        if player_idx < 4 {
            self.entries[player_idx].detention_slips += 1;
        }
    }
}
