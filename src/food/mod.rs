pub mod components;
pub mod launcher;
pub mod melee;
pub mod spawning;
pub mod throwing;
pub mod trajectory;

use bevy::prelude::*;

use crate::states::{GameSessionActive, GameState};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (
                spawning::setup_food_spawns,
                launcher::setup_launcher_spawns,
                spawning::setup_melee_spawns,
            )
                .run_if(not(resource_exists::<GameSessionActive>)),
        )
        .add_systems(
            FixedUpdate,
            (
                spawning::food_respawn_system,
                throwing::pickup_system,
                throwing::throw_system,
                launcher::reset_launcher_spawn_point_system,
                launcher::launcher_respawn_system,
                launcher::launcher_fire_system,
                launcher::catapult_charge_system,
                melee::melee_pickup_system,
                melee::melee_block_system,
                melee::baguette_swing_system,
                spawning::melee_respawn_system,
                trajectory::straight_trajectory_system,
                trajectory::arc_trajectory_system,
                trajectory::bounce_trajectory_system,
                trajectory::splat_fade_system,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
