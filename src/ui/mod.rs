use bevy::prelude::*;

use crate::states::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                main_menu_system.run_if(in_state(GameState::MainMenu)),
            );
    }
}

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
            // Title
            parent.spawn((
                Text::new("CAFETERIA FOOD FIGHT"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            // Subtitle / instructions
            parent.spawn((
                Text::new("Press SPACE to start"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Controls info
            parent.spawn((
                Text::new("P1: WASD  |  P2: Arrow Keys"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

fn main_menu_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<MainMenuUi>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Clean up menu UI
        for entity in &menu_query {
            commands.entity(entity).despawn_recursive();
        }
        // Skip map select for now, go straight to playing
        next_state.set(GameState::Playing);
    }
}
