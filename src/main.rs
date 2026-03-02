mod audio;
mod combat;
mod controller;
mod food;
pub mod input;
mod lobby;
mod map;
mod npc;
mod player;
pub mod score;
mod sprites;
mod states;
#[cfg(feature = "steam")]
mod steam;
mod ui;

use bevy::prelude::*;
use score::CumulativeScores;
use states::{GameSessionActive, GameState};

fn main() {
    let mut app = App::new();

    app.init_resource::<CumulativeScores>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cafeteria Food Fight".to_string(),
                        resolution: window_resolution(),
                        #[cfg(feature = "steam")]
                        mode: bevy::window::WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.15, 0.15, 0.2)))
        .init_state::<GameState>()
        // Sprite assets (must be before gameplay plugins)
        .add_plugins(sprites::SpritePlugin)
        .add_plugins(audio::AudioPlugin);

    // Steam integration (must be before input plugin)
    #[cfg(feature = "steam")]
    app.add_plugins(steam::SteamInputPlugin);

    // Input abstraction layer + core plugins
    app.add_plugins(input::InputPlugin)
        .add_plugins(controller::ControllerPlugin)
        .add_plugins(lobby::LobbyPlugin)
        // Gameplay plugins
        .add_plugins(player::PlayerPlugin)
        .add_plugins(food::FoodPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(npc::NpcPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(ui::UiPlugin)
        // Mark session active on first enter Playing (prevents re-spawn on unpause)
        .add_systems(
            OnEnter(GameState::Playing),
            mark_session_active.run_if(not(resource_exists::<GameSessionActive>)),
        )
        // Core setup
        .add_systems(Startup, setup_camera)
        .run();
}

fn window_resolution() -> bevy::window::WindowResolution {
    #[cfg(target_os = "linux")]
    {
        bevy::window::WindowResolution::new(1280.0, 800.0).with_scale_factor_override(1.0)
    }
    #[cfg(not(target_os = "linux"))]
    {
        bevy::window::WindowResolution::new(1280.0, 800.0)
    }
}

fn mark_session_active(mut commands: Commands) {
    commands.insert_resource(GameSessionActive);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
