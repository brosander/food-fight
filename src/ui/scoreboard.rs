use bevy::prelude::*;

use crate::input::ControllerRegistry;
use crate::player::components::{Eliminated, Player};
use crate::states::GameState;

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

pub fn setup_round_over(mut commands: Commands, players: Query<(&Player, Option<&Eliminated>)>) {
    let winner = players
        .iter()
        .find(|(_, elim)| elim.is_none())
        .map(|(p, _)| p.id);

    let title = match winner {
        Some(id) => format!("Player {} wins!", id),
        None => "Draw!".to_string(),
    };

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            RoundOverUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(title),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            parent.spawn((
                Text::new("Press START to return to menu"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
}

pub fn round_over_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    registry: Res<ControllerRegistry>,
    mut next_state: ResMut<NextState<GameState>>,
    ui_query: Query<Entity, With<RoundOverUi>>,
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
        for entity in &ui_query {
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameState::MainMenu);
    }
}
