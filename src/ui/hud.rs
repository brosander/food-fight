use bevy::prelude::*;

use crate::food::components::Inventory;
use crate::food::launcher::EquippedLauncher;
use crate::npc::components::Caught;
use crate::player::components::{Health, Player};

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HealthBar {
    pub player_id: u8,
}

#[derive(Component)]
pub struct HealthBarFill {
    pub player_id: u8,
}

#[derive(Component)]
pub struct PlayerStatusText {
    pub player_id: u8,
}

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            // Top bar: both players' health
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|top| {
                    // Player 1 HUD (left)
                    spawn_player_hud(top, 1, Color::srgb(0.2, 0.4, 0.9));
                    // Player 2 HUD (right)
                    spawn_player_hud(top, 2, Color::srgb(0.9, 0.2, 0.2));
                });
        });
}

fn spawn_player_hud(parent: &mut ChildBuilder, player_id: u8, color: Color) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|col| {
            // Player label
            col.spawn((
                Text::new(format!("Player {}", player_id)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(color),
            ));

            // Health bar background
            col.spawn((
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(16.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                HealthBar { player_id },
            ))
            .with_children(|bar| {
                // Health bar fill
                bar.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                    HealthBarFill { player_id },
                ));
            });

            // Status text (held item, stunned, etc.)
            col.spawn((
                Text::new(""),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                PlayerStatusText { player_id },
            ));
        });
}

pub fn update_hud(
    players: Query<(&Player, &Health, &Inventory, Option<&EquippedLauncher>, Option<&Caught>)>,
    mut health_fills: Query<(&HealthBarFill, &mut Node)>,
    mut status_texts: Query<(&PlayerStatusText, &mut Text)>,
) {
    for (player, health, inventory, launcher, caught) in &players {
        // Update health bar
        let health_pct = (health.0 / 100.0 * 100.0).clamp(0.0, 100.0);
        for (fill, mut node) in &mut health_fills {
            if fill.player_id == player.id {
                node.width = Val::Percent(health_pct);
            }
        }

        // Update status text
        let mut status = String::new();
        if caught.is_some() {
            status.push_str("STUNNED! ");
        }
        if let Some(food) = &inventory.held_food {
            status.push_str(&format!("Holding: {:?} ", food));
        }
        if let Some(l) = launcher {
            status.push_str(&format!("[{:?} x{}]", l.launcher_type, l.uses_remaining));
        }
        if status.is_empty() {
            status.push_str("Empty handed");
        }

        for (text_marker, mut text) in &mut status_texts {
            if text_marker.player_id == player.id {
                **text = status.clone();
            }
        }
    }
}
