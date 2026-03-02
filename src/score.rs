//! Scoring resources for the food fight game.
//!
//! Two separate resources track scores at different scopes:
//! - `RoundScores`      — current round only; reset at the start of each new round.
//! - `CumulativeScores` — session total; reset only when returning to the main menu.
//!
//! Both are indexed by player index (player.id - 1, range 0–3).

use bevy::prelude::*;

/// Per-player stats for a single scope (one round or full session).
#[derive(Default, Clone, Copy)]
pub struct PlayerScore {
    /// Total damage dealt to other players.
    pub damage_dealt: f32,
    /// Number of players sent to lunch detention (eliminations caused).
    pub detention_slips: u32,
}

/// Scores for the current round only. Reset at the start of every new round.
#[derive(Resource, Default)]
pub struct RoundScores {
    pub entries: [PlayerScore; 4],
}

/// Cumulative scores across all rounds in the session.
/// Survives round transitions; reset only on return to the main menu.
#[derive(Resource, Default)]
pub struct CumulativeScores {
    pub entries: [PlayerScore; 4],
}

macro_rules! impl_scores {
    ($t:ty) => {
        impl $t {
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
    };
}

impl_scores!(RoundScores);
impl_scores!(CumulativeScores);
