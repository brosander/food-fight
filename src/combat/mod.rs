pub mod collision;

use bevy::prelude::*;

use crate::states::GameState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                collision::food_player_collision_system,
                collision::food_teacher_collision_system,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
