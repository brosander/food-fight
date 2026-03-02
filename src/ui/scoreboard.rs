use bevy::prelude::*;

use crate::input::ControllerRegistry;
use crate::lobby::PLAYER_COLORS;
use crate::player::components::{Eliminated, Player};
use crate::score::CumulativeScores;
use crate::states::{GameSessionActive, Gameplay};
use crate::states::GameState;

use super::hud::HudRoot;

#[derive(Component)]
pub struct RoundOverUi;

/// Checks win condition: last player standing (everyone else eliminated).
pub fn win_check_system(
    mut next_state: ResMut<NextState<GameState>>,
    players: Query<Option<&Eliminated>, With<Player>>,
) {
    let total = players.iter().count();
    if total < 2 {
        return;
    }
    let alive = players.iter().filter(|e| e.is_none()).count();
    if alive <= 1 {
        next_state.set(GameState::RoundOver);
    }
}

pub fn setup_round_over(
    mut commands: Commands,
    players: Query<(&Player, Option<&Eliminated>)>,
    scores: Res<CumulativeScores>,
) {
    let winner = players
        .iter()
        .find(|(_, elim)| elim.is_none())
        .map(|(p, _)| p.id);

    let round_title = match winner {
        Some(id) => format!("Player {} wins this round!", id),
        None => "Draw!".to_string(),
    };

    // Collect active players sorted by id
    let mut active: Vec<(u8, Color)> = players
        .iter()
        .map(|(p, _)| (p.id, PLAYER_COLORS[(p.id - 1) as usize]))
        .collect();
    active.sort_by_key(|(id, _)| *id);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.82)),
            RoundOverUi,
        ))
        .with_children(|root| {
            // Round result title
            root.spawn((
                Text::new(round_title),
                TextFont { font_size: 44.0, ..default() },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            // Scoreboard header
            root.spawn((
                Text::new("CUMULATIVE SCORES"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));

            // Table container
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            })
            .with_children(|table| {
                // Column header row
                table
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(0.0),
                        ..default()
                    })
                    .with_children(|row| {
                        score_cell(row, "Player", 160.0, Color::srgb(0.5, 0.5, 0.5));
                        score_cell(row, "Damage", 120.0, Color::srgb(0.5, 0.5, 0.5));
                        score_cell(row, "Detention Slips", 200.0, Color::srgb(0.5, 0.5, 0.5));
                    });

                // Per-player rows
                for (player_id, color) in &active {
                    let idx = (player_id - 1) as usize;
                    let entry = &scores.entries[idx];
                    let name = format!("Player {}", player_id);
                    let damage = format!("{}", entry.damage_dealt as u32);
                    let slips = format!("{}", entry.detention_slips);

                    table
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(0.0),
                            ..default()
                        })
                        .with_children(|row| {
                            score_cell(row, &name, 160.0, *color);
                            score_cell(row, &damage, 120.0, Color::srgb(0.9, 0.9, 0.9));
                            score_cell(row, &slips, 200.0, Color::srgb(1.0, 0.5, 0.15));
                        });
                }
            });

            // Controls hint
            root.spawn((
                Text::new("START = Play Again         EAST (B) = Main Menu"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

/// Spawns a fixed-width text cell for the score table.
fn score_cell(parent: &mut ChildBuilder, text: &str, width: f32, color: Color) {
    parent
        .spawn(Node {
            width: Val::Px(width),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|cell| {
            cell.spawn((
                Text::new(text),
                TextFont { font_size: 20.0, ..default() },
                TextColor(color),
            ));
        });
}

pub fn round_over_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    registry: Res<ControllerRegistry>,
    mut next_state: ResMut<NextState<GameState>>,
    ui_query: Query<Entity, With<RoundOverUi>>,
    gameplay_query: Query<Entity, With<Gameplay>>,
    hud_query: Query<Entity, With<HudRoot>>,
) {
    let mut play_again = keyboard.just_pressed(KeyCode::Space);
    let mut go_menu = keyboard.just_pressed(KeyCode::Escape);

    for controller in &registry.controllers {
        if controller.input.pause.just_pressed {
            play_again = true;
        }
        if controller.input.leave.just_pressed {
            go_menu = true;
        }
    }

    if play_again {
        // Tear down everything gameplay-related, then re-enter Playing.
        // commands are applied before OnEnter(Playing) schedules run, so
        // spawn systems will see the fresh state (no GameSessionActive).
        for entity in &ui_query {
            commands.entity(entity).despawn_recursive();
        }
        for entity in &hud_query {
            commands.entity(entity).despawn_recursive();
        }
        for entity in &gameplay_query {
            commands.entity(entity).despawn();
        }
        commands.remove_resource::<GameSessionActive>();
        next_state.set(GameState::Playing);
    } else if go_menu {
        for entity in &ui_query {
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameState::MainMenu);
    }
}
