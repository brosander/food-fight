pub mod hud;
pub mod scoreboard;

use bevy::prelude::*;

use crate::input::ControllerRegistry;
use crate::lobby::Lobby;
use crate::states::{GameSessionActive, GameState, Gameplay};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Main menu
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                main_menu_system.run_if(in_state(GameState::MainMenu)),
            )
            // Playing: HUD + win check + pause
            .add_systems(
                OnEnter(GameState::Playing),
                hud::setup_hud.run_if(not(resource_exists::<GameSessionActive>)),
            )
            .add_systems(
                Update,
                (
                    hud::update_hud,
                    scoreboard::win_check_system,
                    pause_system,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Pause
            .add_systems(
                Update,
                unpause_system.run_if(in_state(GameState::Paused)),
            )
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_pause_menu)
            // Round over
            .add_systems(OnEnter(GameState::RoundOver), scoreboard::setup_round_over)
            .add_systems(
                Update,
                scoreboard::round_over_system.run_if(in_state(GameState::RoundOver)),
            )
            // Cleanup gameplay when returning to main menu
            .add_systems(OnEnter(GameState::MainMenu), (cleanup_gameplay, cleanup_lobby));
    }
}

// === Main Menu ===

#[derive(Component)]
struct MainMenuUi;

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            MainMenuUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("CAFETERIA FOOD FIGHT"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            parent.spawn((
                Text::new("Press START or SPACE"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            parent.spawn((
                Text::new("Left Stick = Move | Right Stick = Aim"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));

            parent.spawn((
                Text::new("[A] = Pickup food | [X] = Pickup launcher | RT = Throw/Fire"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

fn main_menu_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    registry: Res<ControllerRegistry>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<MainMenuUi>>,
    mut commands: Commands,
) {
    let mut go = keyboard.just_pressed(KeyCode::Space);
    if !go {
        for controller in &registry.controllers {
            if controller.input.pause.just_pressed {
                go = true;
                break;
            }
        }
    }
    if go {
        for entity in &menu_query {
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameState::Lobby);
    }
}

// === Pause ===

#[derive(Component)]
struct PauseUi;

fn pause_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    registry: Res<ControllerRegistry>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
        return;
    }
    for controller in &registry.controllers {
        if controller.input.pause.just_pressed {
            next_state.set(GameState::Paused);
            return;
        }
    }
}

fn unpause_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    registry: Res<ControllerRegistry>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit_events: EventWriter<AppExit>,
) {
    // Exit game check
    if keyboard.just_pressed(KeyCode::KeyQ) {
        exit_events.send(AppExit::Success);
        return;
    }
    for controller in &registry.controllers {
        if controller.input.exit_game.just_pressed {
            exit_events.send(AppExit::Success);
            return;
        }
    }

    // Resume check
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
        return;
    }
    for controller in &registry.controllers {
        if controller.input.pause.just_pressed {
            next_state.set(GameState::Playing);
            return;
        }
    }
}

fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            PauseUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
            parent.spawn((
                Text::new("Press START to resume"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
            parent.spawn((
                Text::new("Press SELECT or Q to Exit Game"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.3, 0.3)),
            ));
        });
}

fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseUi>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// === Cleanup ===

/// Despawn all gameplay entities when returning to main menu.
fn cleanup_gameplay(
    mut commands: Commands,
    gameplay_entities: Query<Entity, With<Gameplay>>,
    hud_query: Query<Entity, With<hud::HudRoot>>,
    round_over_query: Query<Entity, With<scoreboard::RoundOverUi>>,
) {
    for entity in &gameplay_entities {
        commands.entity(entity).despawn();
    }
    for entity in &hud_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &round_over_query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<GameSessionActive>();
}

/// Clear the lobby when returning to main menu.
fn cleanup_lobby(mut lobby: ResMut<Lobby>) {
    lobby.slots.clear();
}
