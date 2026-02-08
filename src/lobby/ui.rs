use bevy::prelude::*;

use super::Lobby;

#[derive(Component)]
pub struct LobbyUi;

#[derive(Component)]
pub struct LobbySlotUi {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct LobbySlotText {
    pub slot_index: usize,
}

pub fn setup_lobby_ui(mut commands: Commands) {
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
            LobbyUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("FOOD FIGHT!"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            parent.spawn((
                Text::new("Press [A] to join!"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Player slots grid (2x2)
            parent
                .spawn(Node {
                    width: Val::Percent(80.0),
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|grid| {
                    for i in 0..4 {
                        spawn_slot(grid, i);
                    }
                });

            parent.spawn((
                Text::new("All players press START when ready"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn spawn_slot(parent: &mut ChildBuilder, slot_index: usize) {
    parent
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(120.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            LobbySlotUi { slot_index },
        ))
        .with_children(|slot| {
            slot.spawn((
                Text::new(format!("Slot {}\nPress [A] to join", slot_index + 1)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.4, 0.4, 0.4)),
                LobbySlotText { slot_index },
            ));
        });
}

pub fn update_lobby_ui(
    lobby: Res<Lobby>,
    mut slot_borders: Query<(&LobbySlotUi, &mut BorderColor, &mut BackgroundColor)>,
    mut slot_texts: Query<(&LobbySlotText, &mut Text, &mut TextColor)>,
) {
    for (slot_ui, mut border, mut bg) in &mut slot_borders {
        let idx = slot_ui.slot_index;

        if let Some(slot) = lobby.slots.get(idx) {
            border.0 = slot.color;
            bg.0 = Color::srgba(0.15, 0.15, 0.15, 0.9);
        } else {
            border.0 = Color::srgb(0.3, 0.3, 0.3);
            bg.0 = Color::srgba(0.1, 0.1, 0.1, 0.8);
        }
    }

    for (text_ui, mut text, mut text_color) in &mut slot_texts {
        let idx = text_ui.slot_index;

        if let Some(slot) = lobby.slots.get(idx) {
            let status = if slot.ready { "READY!" } else { "Press START" };
            **text = format!("{}\n{}", slot.display_name, status);

            text_color.0 = if slot.ready {
                Color::srgb(0.3, 1.0, 0.3)
            } else {
                slot.color
            };
        } else {
            **text = format!("Slot {}\nPress [A] to join", idx + 1);
            text_color.0 = Color::srgb(0.4, 0.4, 0.4);
        }
    }
}

pub fn cleanup_lobby_ui(mut commands: Commands, query: Query<Entity, With<LobbyUi>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
