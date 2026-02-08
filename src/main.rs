mod combat;
mod food;
mod map;
mod npc;
mod player;
mod states;
mod ui;

use bevy::prelude::*;
use states::GameState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Cafeteria Food Fight".to_string(),
                    resolution: (960.0, 640.0).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(ClearColor(Color::srgb(0.15, 0.15, 0.2)))
        .init_state::<GameState>()
        // Gameplay plugins
        .add_plugins(player::PlayerPlugin)
        .add_plugins(food::FoodPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(npc::NpcPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(ui::UiPlugin)
        // Core setup
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
