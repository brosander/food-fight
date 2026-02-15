//! Gamepad utilities and connection event handling (gilrs backend).
//!
//! Provides `read_left_stick` and `read_right_stick` (used by `input::gilrs_populate_system`),
//! disconnect-to-pause behaviour, and raw event debug logging (gated on `Level::DEBUG`).
//! Also holds `ControllerFamily` and button-label helpers for future glyph-display UI.

use bevy::input::gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnection, GamepadConnectionEvent,
};
use bevy::log::Level;
use bevy::prelude::*;
use bevy::utils::tracing::enabled;

use crate::states::GameState;

/// Controller family detection — used for button glyph display (planned HUD feature).
// Suppressed until the HUD shows context-sensitive button prompts.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerFamily {
    Xbox,
    PlayStation,
    Nintendo,
    Unknown,
}

#[allow(dead_code)]
pub fn detect_controller_family(name: &str) -> ControllerFamily {
    let name_lower = name.to_lowercase();
    if name_lower.contains("xbox") || name_lower.contains("xinput") {
        ControllerFamily::Xbox
    } else if name_lower.contains("dualsense")
        || name_lower.contains("dualshock")
        || name_lower.contains("playstation")
        || name_lower.contains("ps5")
        || name_lower.contains("ps4")
    {
        ControllerFamily::PlayStation
    } else if name_lower.contains("nintendo")
        || name_lower.contains("switch")
        || name_lower.contains("joy-con")
        || name_lower.contains("pro controller")
    {
        ControllerFamily::Nintendo
    } else {
        ControllerFamily::Unknown
    }
}

#[allow(dead_code)]
pub fn south_button_label(family: ControllerFamily) -> &'static str {
    match family {
        ControllerFamily::Xbox => "A",
        ControllerFamily::PlayStation => "X",
        ControllerFamily::Nintendo => "B",
        ControllerFamily::Unknown => "A",
    }
}

#[allow(dead_code)]
pub fn east_button_label(family: ControllerFamily) -> &'static str {
    match family {
        ControllerFamily::Xbox => "B",
        ControllerFamily::PlayStation => "O",
        ControllerFamily::Nintendo => "A",
        ControllerFamily::Unknown => "B",
    }
}

#[allow(dead_code)]
pub fn west_button_label(family: ControllerFamily) -> &'static str {
    match family {
        ControllerFamily::Xbox => "X",
        ControllerFamily::PlayStation => "[]",
        ControllerFamily::Nintendo => "Y",
        ControllerFamily::Unknown => "X",
    }
}

#[allow(dead_code)]
pub fn start_button_label(family: ControllerFamily) -> &'static str {
    match family {
        ControllerFamily::Xbox => "Menu",
        ControllerFamily::PlayStation => "Options",
        ControllerFamily::Nintendo => "+",
        ControllerFamily::Unknown => "Start",
    }
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_disconnections, handle_reconnections, debug_gamepads),
        );
    }
}

/// Debug system: logs raw gamepad events at the event level.
/// This bypasses Bevy's Gamepad component to see if gilrs is sending any data at all.
fn debug_gamepads(
    gamepads: Query<(Entity, &Gamepad)>,
    mut last_count: Local<usize>,
    mut button_events: EventReader<GamepadButtonChangedEvent>,
    mut axis_events: EventReader<GamepadAxisChangedEvent>,
) {
    let count = gamepads.iter().count();
    if count != *last_count {
        info!("Gamepads connected: {}", count);
        for (entity, _gamepad) in &gamepads {
            info!("  Gamepad entity: {:?}", entity);
        }
        *last_count = count;
    }

    if enabled!(Level::DEBUG) {
        // Log ALL raw button change events — this is the lowest level before Gamepad processing
        for event in button_events.read() {
            debug!(
                "RAW BUTTON event: entity={:?} button={:?} state={:?}",
                event.entity, event.button, event.state
            );
        }

        // Log ALL raw axis change events
        for event in axis_events.read() {
            // Filter out near-zero noise
            if event.value.abs() > 0.01 {
                debug!(
                    "RAW AXIS event: entity={:?} axis={:?} value={:.3}",
                    event.entity, event.axis, event.value
                );
            }
        }
    }
}

fn handle_disconnections(
    mut connection_events: EventReader<GamepadConnectionEvent>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in connection_events.read() {
        if let GamepadConnection::Disconnected = &event.connection {
            info!("Controller disconnected: {:?}", event.gamepad);

            if *current_state.get() == GameState::Playing {
                next_state.set(GameState::Paused);
            }
        }
    }
}

fn handle_reconnections(mut connection_events: EventReader<GamepadConnectionEvent>) {
    for event in connection_events.read() {
        if let GamepadConnection::Connected { name, .. } = &event.connection {
            info!("Controller connected: {}", name);
        }
    }
}

/// Read the left stick as a normalized Vec2 direction.
pub fn read_left_stick(gamepad: &Gamepad) -> Vec2 {
    let x = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
    let y = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
    let raw = Vec2::new(x, y);
    // Apply deadzone
    if raw.length() < 0.15 {
        Vec2::ZERO
    } else {
        raw.normalize()
    }
}

/// Read the right stick as a normalized Vec2 direction.
pub fn read_right_stick(gamepad: &Gamepad) -> Vec2 {
    let x = gamepad.get(GamepadAxis::RightStickX).unwrap_or(0.0);
    let y = gamepad.get(GamepadAxis::RightStickY).unwrap_or(0.0);
    let raw = Vec2::new(x, y);
    // Apply deadzone
    if raw.length() < 0.15 {
        Vec2::ZERO
    } else {
        raw.normalize()
    }
}

/// Compute aim direction from right stick, falling back to left stick (movement direction).
/// Note: prefer `ControllerInput::aim_direction()` in gameplay systems.
#[allow(dead_code)]
pub fn read_aim_direction(gamepad: &Gamepad) -> Vec2 {
    let right = read_right_stick(gamepad);
    if right != Vec2::ZERO {
        return right;
    }
    let left = read_left_stick(gamepad);
    if left != Vec2::ZERO {
        return left;
    }
    Vec2::Y // default aim up
}
