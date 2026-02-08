pub mod components;
pub mod launcher;
pub mod spawning;
pub mod throwing;
pub mod trajectory;

use bevy::prelude::*;

use crate::states::GameState;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (
                spawning::setup_food_spawns,
                spawning::initial_food_spawn,
                launcher::setup_launcher_spawns,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                spawning::food_respawn_system,
                spawning::reset_spawn_point_system,
                throwing::pickup_system,
                throwing::throw_system,
                launcher::launcher_pickup_system,
                launcher::launcher_fire_system,
                launcher::catapult_charge_system,
                trajectory::straight_trajectory_system,
                trajectory::arc_trajectory_system,
                trajectory::bounce_trajectory_system,
                trajectory::splat_fade_system,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
