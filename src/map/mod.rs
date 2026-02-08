pub mod collision;
pub mod loading;

use bevy::prelude::*;

use crate::states::GameState;

/// Marker component for solid wall/obstacle entities.
#[derive(Component)]
pub struct Wall {
    pub half_size: Vec2,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), loading::spawn_cafeteria)
            .add_systems(
                FixedUpdate,
                (
                    collision::wall_collision_system,
                    collision::projectile_wall_collision_system,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
