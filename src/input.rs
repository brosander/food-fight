//! Input abstraction layer.
//!
//! Defines [`ControllerInput`] (component on player entities), [`ControllerRegistry`]
//! (resource listing all connected controllers), and [`ControllerId`] (backend-agnostic
//! controller handle). [`InputPlugin`] runs the gilrs population system each `PreUpdate`.
//! When `--features steam` is active, `steam::steam_input_populate` overwrites the
//! registry before gilrs sees it, so both backends coexist without conflict.

use bevy::prelude::*;

use crate::controller::{read_left_stick, read_right_stick};
use crate::player::components::ControllerLink;

/// Unified input state for one controller, updated each frame.
/// All gameplay and menu systems read from this instead of Gamepad directly.
#[derive(Component, Default, Clone)]
pub struct ControllerInput {
    pub move_stick: Vec2,
    pub aim_stick: Vec2,
    pub pickup_food: ButtonState,
    pub pickup_launcher: ButtonState,
    pub fire: ButtonState,
    pub melee: ButtonState,
    pub pause: ButtonState,
    pub join: ButtonState,
    pub leave: ButtonState,
    pub exit_game: ButtonState,
}

impl ControllerInput {
    /// Compute aim direction from aim stick, falling back to move stick, defaulting to up.
    pub fn aim_direction(&self) -> Vec2 {
        if self.aim_stick != Vec2::ZERO {
            return self.aim_stick;
        }
        if self.move_stick != Vec2::ZERO {
            return self.move_stick;
        }
        Vec2::Y
    }
}

#[derive(Default, Clone, Copy)]
pub struct ButtonState {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

/// Opaque identifier that works for both Bevy gamepad and Steam Input backends.
#[derive(Clone, Copy, Debug)]
pub enum ControllerId {
    Bevy(Entity),
    #[cfg(feature = "steam")]
    Steam(u64),
}

impl PartialEq for ControllerId {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ControllerId::Bevy(a), ControllerId::Bevy(b)) => a == b,
            #[cfg(feature = "steam")]
            (ControllerId::Steam(a), ControllerId::Steam(b)) => a == b,
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}
impl Eq for ControllerId {}

impl std::hash::Hash for ControllerId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ControllerId::Bevy(e) => {
                0u8.hash(state);
                e.hash(state);
            }
            #[cfg(feature = "steam")]
            ControllerId::Steam(h) => {
                1u8.hash(state);
                h.hash(state);
            }
        }
    }
}

/// Tracks all connected controllers and their input state.
/// Menu/lobby systems iterate this to find unbound controllers.
#[derive(Resource, Default)]
pub struct ControllerRegistry {
    pub controllers: Vec<RegisteredController>,
}

pub struct RegisteredController {
    pub id: ControllerId,
    pub input: ControllerInput,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControllerRegistry>()
            .add_systems(PreUpdate, gilrs_populate_system);
    }
}

/// Populates ControllerRegistry and ControllerInput components from Bevy's Gamepad (gilrs backend).
fn gilrs_populate_system(
    gamepads: Query<(Entity, &Gamepad)>,
    mut registry: ResMut<ControllerRegistry>,
    mut players: Query<(&ControllerLink, &mut ControllerInput)>,
) {
    // Rebuild the registry from all connected gamepads
    registry.controllers.clear();
    for (entity, gamepad) in &gamepads {
        let input = build_input_from_gamepad(gamepad);
        registry.controllers.push(RegisteredController {
            id: ControllerId::Bevy(entity),
            input: input.clone(),
        });
    }

    // Update ControllerInput on player entities
    for (link, mut controller_input) in &mut players {
        #[cfg(feature = "steam")]
        let entity = match link.0 {
            ControllerId::Bevy(e) => e,
            ControllerId::Steam(_) => continue,
        };
        #[cfg(not(feature = "steam"))]
        let ControllerId::Bevy(entity) = link.0;
        if let Ok((_, gamepad)) = gamepads.get(entity) {
            *controller_input = build_input_from_gamepad(gamepad);
        } else {
            *controller_input = ControllerInput::default();
        }
    }
}

fn build_input_from_gamepad(gamepad: &Gamepad) -> ControllerInput {
    ControllerInput {
        move_stick: read_left_stick(gamepad),
        aim_stick: read_right_stick(gamepad),
        pickup_food: ButtonState {
            pressed: gamepad.pressed(GamepadButton::South),
            just_pressed: gamepad.just_pressed(GamepadButton::South),
            just_released: gamepad.just_released(GamepadButton::South),
        },
        pickup_launcher: ButtonState {
            pressed: gamepad.pressed(GamepadButton::West),
            just_pressed: gamepad.just_pressed(GamepadButton::West),
            just_released: gamepad.just_released(GamepadButton::West),
        },
        fire: ButtonState {
            pressed: gamepad.pressed(GamepadButton::RightTrigger2),
            just_pressed: gamepad.just_pressed(GamepadButton::RightTrigger2),
            just_released: gamepad.just_released(GamepadButton::RightTrigger2),
        },
        melee: ButtonState {
            pressed: gamepad.pressed(GamepadButton::RightTrigger),
            just_pressed: gamepad.just_pressed(GamepadButton::RightTrigger),
            just_released: gamepad.just_released(GamepadButton::RightTrigger),
        },
        pause: ButtonState {
            pressed: gamepad.pressed(GamepadButton::Start),
            just_pressed: gamepad.just_pressed(GamepadButton::Start),
            just_released: gamepad.just_released(GamepadButton::Start),
        },
        join: ButtonState {
            pressed: gamepad.pressed(GamepadButton::South),
            just_pressed: gamepad.just_pressed(GamepadButton::South),
            just_released: gamepad.just_released(GamepadButton::South),
        },
        leave: ButtonState {
            pressed: gamepad.pressed(GamepadButton::East),
            just_pressed: gamepad.just_pressed(GamepadButton::East),
            just_released: gamepad.just_released(GamepadButton::East),
        },
        exit_game: ButtonState {
            pressed: gamepad.pressed(GamepadButton::Select),
            just_pressed: gamepad.just_pressed(GamepadButton::Select),
            just_released: gamepad.just_released(GamepadButton::Select),
        },
    }
}
