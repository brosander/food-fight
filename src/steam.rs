//! Steam Input backend (`--features steam`).
//!
//! [`SteamInputPlugin`] initializes `steamworks::Client`, loads the IGA manifest
//! (`steam_input_manifest.vdf`), caches action handles, and runs `steam_input_populate`
//! in `PreUpdate` to fill [`crate::input::ControllerRegistry`] each frame.
//! Action sets switch automatically: `GameplayControls` while Playing, `MenuControls`
//! everywhere else.
//!
//! `steamworks::Client` is `Send+Sync` — stored as a plain Bevy Resource.
//! `run_callbacks()` is called on `Client` directly (no `SingleClient` needed).

/// Steam Input integration using the raw `steamworks` crate.
///
/// `steamworks::Client` is Send+Sync — stored as a normal Resource.
/// `run_callbacks()` is called on `Client` directly (no SingleClient needed).
use std::collections::HashMap;

use bevy::prelude::*;
use steamworks::sys::{
    InputActionSetHandle_t, InputAnalogActionHandle_t, InputDigitalActionHandle_t, InputHandle_t,
};
use steamworks::Client;

use crate::input::{ButtonState, ControllerId, ControllerInput, ControllerRegistry, RegisteredController};
use crate::player::components::ControllerLink;
use crate::states::GameState;

/// Steam app ID for development (Valve's SpaceWar test app).
/// Replace with your actual app ID for release.
const DEV_APP_ID: u32 = 480;

// ── Resources ────────────────────────────────────────────────────────────────

/// The shared Steam client — Send+Sync so it can live in a regular Resource.
#[derive(Resource)]
pub struct SteamAppClient(pub Client);

/// Cached Steam Input action handles (resolved once at startup).
#[derive(Resource)]
struct SteamActionHandles {
    gameplay_set: InputActionSetHandle_t,
    menu_set: InputActionSetHandle_t,
    move_action: InputAnalogActionHandle_t,
    aim_action: InputAnalogActionHandle_t,
    pickup_food: InputDigitalActionHandle_t,
    pickup_launcher: InputDigitalActionHandle_t,
    fire: InputDigitalActionHandle_t,
    pause: InputDigitalActionHandle_t,
    join: InputDigitalActionHandle_t,
    leave: InputDigitalActionHandle_t,
    exit_game: InputDigitalActionHandle_t,
}

/// Previous-frame digital button state for edge detection.
#[derive(Resource, Default)]
struct SteamPrevState {
    prev: HashMap<InputHandle_t, [bool; 7]>,
}

const IDX_PICKUP_FOOD: usize = 0;
const IDX_PICKUP_LAUNCHER: usize = 1;
const IDX_FIRE: usize = 2;
const IDX_PAUSE: usize = 3;
const IDX_JOIN: usize = 4;
const IDX_LEAVE: usize = 5;
const IDX_EXIT: usize = 6;

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct SteamInputPlugin;

impl Plugin for SteamInputPlugin {
    fn build(&self, app: &mut App) {
        match Client::init_app(DEV_APP_ID) {
            Ok(client) => {
                // Client is Send+Sync — lives in a regular Resource.
                app.insert_resource(SteamAppClient(client));

                app.add_systems(Startup, init_steam_input);
                // run_callbacks can run from any thread (Client is Send+Sync)
                app.add_systems(PreUpdate, run_steam_callbacks);
                app.add_systems(PreUpdate, steam_input_populate.after(run_steam_callbacks));
                app.add_systems(OnEnter(GameState::Playing), activate_gameplay_actions);
                app.add_systems(OnEnter(GameState::MainMenu), activate_menu_actions);
                app.add_systems(OnEnter(GameState::Lobby), activate_menu_actions);
                app.add_systems(OnEnter(GameState::Paused), activate_menu_actions);
                app.add_systems(OnEnter(GameState::RoundOver), activate_menu_actions);
                info!("Steam initialized (app_id={})", DEV_APP_ID);
            }
            Err(e) => {
                warn!("Failed to initialize Steam ({:?}). Falling back to gilrs.", e);
            }
        }
    }
}

// ── Systems ───────────────────────────────────────────────────────────────────

/// Pump the Steam callback queue.
fn run_steam_callbacks(client: Res<SteamAppClient>) {
    client.0.run_callbacks();
}

fn init_steam_input(mut commands: Commands, steam: Res<SteamAppClient>) {
    let input = steam.0.input();
    input.init(false);

    // Load the IGA manifest from the working directory
    let manifest_path = std::env::current_dir()
        .map(|p| p.join("steam_input_manifest.vdf"))
        .unwrap_or_default();
    if manifest_path.exists() {
        input.set_input_action_manifest_file_path(&manifest_path.to_string_lossy());
        info!("Steam Input manifest loaded: {:?}", manifest_path);
    } else {
        warn!(
            "steam_input_manifest.vdf not found at {:?}. \
             Controller remapping will not work.",
            manifest_path
        );
    }

    let gameplay_set = input.get_action_set_handle("GameplayControls");
    let menu_set = input.get_action_set_handle("MenuControls");

    if gameplay_set == 0 || menu_set == 0 {
        warn!(
            "Steam Input action sets not found (gameplay={}, menu={}). \
             Check steam_input_manifest.vdf.",
            gameplay_set, menu_set
        );
    }

    commands.insert_resource(SteamActionHandles {
        gameplay_set,
        menu_set,
        move_action: input.get_analog_action_handle("move"),
        aim_action: input.get_analog_action_handle("aim"),
        pickup_food: input.get_digital_action_handle("pickup_food"),
        pickup_launcher: input.get_digital_action_handle("pickup_launcher"),
        fire: input.get_digital_action_handle("fire"),
        pause: input.get_digital_action_handle("pause"),
        join: input.get_digital_action_handle("join"),
        leave: input.get_digital_action_handle("leave"),
        exit_game: input.get_digital_action_handle("exit_game"),
    });
    commands.insert_resource(SteamPrevState::default());
    info!("Steam Input action handles cached");
}

fn activate_gameplay_actions(
    steam: Option<Res<SteamAppClient>>,
    handles: Option<Res<SteamActionHandles>>,
) {
    let (Some(steam), Some(handles)) = (steam, handles) else { return };
    let input = steam.0.input();
    for controller in input.get_connected_controllers() {
        input.activate_action_set_handle(controller, handles.gameplay_set);
    }
}

fn activate_menu_actions(
    steam: Option<Res<SteamAppClient>>,
    handles: Option<Res<SteamActionHandles>>,
) {
    let (Some(steam), Some(handles)) = (steam, handles) else { return };
    let input = steam.0.input();
    for controller in input.get_connected_controllers() {
        input.activate_action_set_handle(controller, handles.menu_set);
    }
}

/// Replaces the gilrs-based ControllerRegistry with Steam Input data
/// when at least one Steam controller is connected.
fn steam_input_populate(
    steam: Option<Res<SteamAppClient>>,
    handles: Option<Res<SteamActionHandles>>,
    prev_state: Option<ResMut<SteamPrevState>>,
    mut registry: ResMut<ControllerRegistry>,
    mut players: Query<(&ControllerLink, &mut ControllerInput)>,
) {
    let (Some(steam), Some(handles), Some(ref mut prev)) = (steam, handles, prev_state) else {
        return;
    };

    let input = steam.0.input();
    let controllers = input.get_connected_controllers();
    if controllers.is_empty() {
        return; // Let gilrs data stand
    }

    registry.controllers.clear();
    for &handle in &controllers {
        let (ci, cur_bools) = read_steam_controller(&input, &handles, handle, &prev.prev);
        registry.controllers.push(RegisteredController {
            id: ControllerId::Steam(handle),
            input: ci.clone(),
        });
        prev.prev.insert(handle, cur_bools);
    }
    // Clean up handles for disconnected controllers
    prev.prev.retain(|h, _| controllers.contains(h));

    // Push input into player entities
    for (link, mut controller_input) in &mut players {
        if let ControllerId::Steam(handle) = link.0 {
            if let Some(rc) = registry.controllers.iter().find(|c| c.id == ControllerId::Steam(handle)) {
                *controller_input = rc.input.clone();
            }
        }
    }
}

// ── Input reading ─────────────────────────────────────────────────────────────

type PrevMap = HashMap<InputHandle_t, [bool; 7]>;

fn read_steam_controller(
    input: &steamworks::Input,
    handles: &SteamActionHandles,
    controller: InputHandle_t,
    prev: &PrevMap,
) -> (ControllerInput, [bool; 7]) {
    // Analog sticks
    let mv = input.get_analog_action_data(controller, handles.move_action);
    let aim = input.get_analog_action_data(controller, handles.aim_action);

    let move_raw = Vec2::new(mv.x, mv.y);
    let aim_raw = Vec2::new(aim.x, aim.y);
    let move_stick = if move_raw.length() < 0.15 { Vec2::ZERO } else { move_raw.normalize() };
    let aim_stick = if aim_raw.length() < 0.15 { Vec2::ZERO } else { aim_raw.normalize() };

    // Digital buttons — current frame state
    let d = |h| input.get_digital_action_data(controller, h).bState;
    let cur = [
        d(handles.pickup_food),
        d(handles.pickup_launcher),
        d(handles.fire),
        d(handles.pause),
        d(handles.join),
        d(handles.leave),
        d(handles.exit_game),
    ];
    let old = prev.get(&controller).copied().unwrap_or([false; 7]);

    let btn = |i: usize| ButtonState {
        pressed: cur[i],
        just_pressed: cur[i] && !old[i],
        just_released: !cur[i] && old[i],
    };

    let ci = ControllerInput {
        move_stick,
        aim_stick,
        pickup_food: btn(IDX_PICKUP_FOOD),
        pickup_launcher: btn(IDX_PICKUP_LAUNCHER),
        fire: btn(IDX_FIRE),
        pause: btn(IDX_PAUSE),
        join: btn(IDX_JOIN),
        leave: btn(IDX_LEAVE),
        exit_game: btn(IDX_EXIT),
    };

    (ci, cur)
}
