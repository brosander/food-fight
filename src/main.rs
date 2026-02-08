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
        .add_systems(OnEnter(GameState::Playing), spawn_play_area_border)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Draws a visible border around the play area so players know the bounds.
fn spawn_play_area_border(mut commands: Commands) {
    let bounds_x = 480.0;
    let bounds_y = 320.0;
    let thickness = 4.0;
    let border_color = Color::srgb(0.4, 0.4, 0.5);

    // Top
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(bounds_x * 2.0, thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, bounds_y, 0.0),
    ));
    // Bottom
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(bounds_x * 2.0, thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, -bounds_y, 0.0),
    ));
    // Left
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(thickness, bounds_y * 2.0)),
            ..default()
        },
        Transform::from_xyz(-bounds_x, 0.0, 0.0),
    ));
    // Right
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(thickness, bounds_y * 2.0)),
            ..default()
        },
        Transform::from_xyz(bounds_x, 0.0, 0.0),
    ));

    // Floor background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.22, 0.2, 0.25),
            custom_size: Some(Vec2::new(bounds_x * 2.0, bounds_y * 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}
