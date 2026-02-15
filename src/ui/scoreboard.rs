use bevy::prelude::*;

use crate::input::ControllerRegistry;
use crate::player::components::{Health, Player};
use crate::states::GameState;

#[derive(Component)]
pub struct RoundOverUi;

/// Checks win condition: first player to reach 0 health loses.
pub fn win_check_system(
    mut next_state: ResMut<NextState<GameState>>,
    players: Query<(&Player, &Health)>,
) {
    for (_player, health) in &players {
        if health.0 <= 0.0 {
            next_state.set(GameState::RoundOver);
            return;
        }
    }
}

pub fn setup_round_over(mut commands: Commands, players: Query<(&Player, &Health)>) {
    let mut winner_id = 0u8;
    let mut max_health = -1.0f32;
    for (player, health) in &players {
        if health.0 > max_health {
            max_health = health.0;
            winner_id = player.id;
        }
    }

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
                Text::new(format!("Player {} wins!", winner_id)),
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
