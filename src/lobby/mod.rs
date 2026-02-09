pub mod ui;

use bevy::prelude::*;

use crate::food::components::Inventory;
use crate::player::components::*;
use crate::sprites::{AnimationState, FrameRange, PlayerSpriteId, SpriteAssets, player_atlas_index};
use crate::states::{GameSessionActive, GameState, Gameplay};

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lobby>()
            .add_systems(OnEnter(GameState::Lobby), ui::setup_lobby_ui)
            .add_systems(OnExit(GameState::Lobby), ui::cleanup_lobby_ui)
            .add_systems(
                Update,
                (
                    lobby_join_system,
                    lobby_leave_system,
                    lobby_ready_system,
                    keyboard_quick_start_system,
                    ui::update_lobby_ui,
                )
                    .chain()
                    .run_if(in_state(GameState::Lobby)),
            )
            .add_systems(
                OnEnter(GameState::Playing),
                spawn_players_from_lobby.run_if(not(resource_exists::<GameSessionActive>)),
            );
    }
}

pub const PLAYER_COLORS: [Color; 4] = [
    Color::srgb(0.2, 0.6, 1.0), // Blue
    Color::srgb(1.0, 0.3, 0.3), // Red
    Color::srgb(0.3, 1.0, 0.3), // Green
    Color::srgb(1.0, 0.9, 0.2), // Yellow
];

const SPAWN_POSITIONS: [Vec2; 4] = [
    Vec2::new(-150.0, 100.0),
    Vec2::new(150.0, 100.0),
    Vec2::new(-150.0, -100.0),
    Vec2::new(150.0, -100.0),
];

#[derive(Resource, Default)]
pub struct Lobby {
    pub slots: Vec<PlayerSlot>,
}

pub struct PlayerSlot {
    pub player_id: u8,
    pub gamepad_entity: Entity,
    pub ready: bool,
    pub color: Color,
    pub display_name: String,
}

fn lobby_join_system(
    mut lobby: ResMut<Lobby>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    for (gamepad_entity, gamepad) in &gamepads {
        if !gamepad.just_pressed(GamepadButton::South) {
            continue;
        }

        let already_joined = lobby
            .slots
            .iter()
            .any(|slot| slot.gamepad_entity == gamepad_entity);
        if already_joined {
            continue;
        }

        if lobby.slots.len() >= 4 {
            continue;
        }

        let player_id = lobby.slots.len() as u8;
        let color = PLAYER_COLORS[player_id as usize];

        lobby.slots.push(PlayerSlot {
            player_id,
            gamepad_entity,
            ready: false,
            color,
            display_name: format!("Player {}", player_id + 1),
        });

        info!(
            "Player {} joined! ({} players in lobby)",
            player_id + 1,
            lobby.slots.len()
        );
    }
}

fn lobby_leave_system(
    mut lobby: ResMut<Lobby>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    let mut to_remove = Vec::new();

    for (gamepad_entity, gamepad) in &gamepads {
        if gamepad.just_pressed(GamepadButton::East) {
            to_remove.push(gamepad_entity);
        }
    }

    for entity in to_remove {
        if let Some(pos) = lobby
            .slots
            .iter()
            .position(|s| s.gamepad_entity == entity)
        {
            let removed = lobby.slots.remove(pos);
            info!("Player {} left the lobby", removed.display_name);

            // Re-assign IDs and colors
            for (i, slot) in lobby.slots.iter_mut().enumerate() {
                slot.player_id = i as u8;
                slot.color = PLAYER_COLORS[i];
                slot.display_name = format!("Player {}", i + 1);
            }
        }
    }
}

fn lobby_ready_system(
    mut lobby: ResMut<Lobby>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (gamepad_entity, gamepad) in &gamepads {
        if gamepad.just_pressed(GamepadButton::Start) {
            if let Some(slot) = lobby
                .slots
                .iter_mut()
                .find(|s| s.gamepad_entity == gamepad_entity)
            {
                slot.ready = !slot.ready;
                info!(
                    "{} ready: {}",
                    slot.display_name, slot.ready
                );
            }
        }
    }

    // Check if all players are ready (minimum 1 for testing)
    let all_ready = !lobby.slots.is_empty() && lobby.slots.iter().all(|s| s.ready);

    if all_ready {
        info!("All players ready! Starting game...");
        next_state.set(GameState::Playing);
    }
}

/// Keyboard shortcut: press Space to auto-join all connected gamepads and start immediately.
fn keyboard_quick_start_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut lobby: ResMut<Lobby>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    // Auto-join all connected gamepads that aren't already in the lobby
    for (gamepad_entity, _gamepad) in &gamepads {
        let already_joined = lobby
            .slots
            .iter()
            .any(|slot| slot.gamepad_entity == gamepad_entity);
        if already_joined || lobby.slots.len() >= 4 {
            continue;
        }

        let player_id = lobby.slots.len() as u8;
        let color = PLAYER_COLORS[player_id as usize];

        lobby.slots.push(PlayerSlot {
            player_id,
            gamepad_entity,
            ready: true,
            color,
            display_name: format!("Player {}", player_id + 1),
        });

        info!("Quick start: Player {} joined with gamepad {:?}", player_id + 1, gamepad_entity);
    }

    if !lobby.slots.is_empty() {
        info!("Quick start: {} players, launching game!", lobby.slots.len());
        next_state.set(GameState::Playing);
    } else {
        info!("Quick start: no gamepads connected — cannot start");
    }
}

fn spawn_players_from_lobby(
    mut commands: Commands,
    lobby: Res<Lobby>,
    sprite_assets: Res<SpriteAssets>,
) {
    for slot in &lobby.slots {
        let player_id = slot.player_id as usize;
        let idle_start = player_atlas_index(0, 4);
        let idle_end = player_atlas_index(0, 5);

        commands.spawn((
            Sprite {
                image: sprite_assets.player_images[player_id].clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_assets.player_layout.clone(),
                    index: idle_start,
                }),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_translation(SPAWN_POSITIONS[slot.player_id as usize].extend(1.0)),
            Player {
                id: slot.player_id + 1,
                speed: 200.0,
            },
            Health(100.0),
            Velocity(Vec2::ZERO),
            Score(0),
            GamepadLink(slot.gamepad_entity),
            Inventory { held_food: None },
            PlayerSpriteId(slot.player_id),
            AnimationState::new(
                "idle",
                FrameRange {
                    start: idle_start,
                    end: idle_end,
                    fps: 3.0,
                    looping: true,
                },
            ),
            Gameplay,
        ));
    }
}
