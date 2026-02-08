# MacBook Build & Run Guide

Guide for building the food fight game on macOS and playing it with Bluetooth controllers using dynamic "press to join" multiplayer.

---

## Part 1: Development Environment

### 1.1 Prerequisites

You need Xcode Command Line Tools (for the C linker and system frameworks):

```bash
xcode-select --install
```

And Rust via rustup (if you don't already have it):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

That's it. Bevy on macOS uses Metal for rendering — no Vulkan SDK needed. No additional system libraries required (unlike Linux).

### 1.2 Build and Run

```bash
cd food_fight
cargo run --release
```

First build compiles all of Bevy and takes a few minutes. Subsequent builds are fast.

For development iteration, the fast-compile profile from `Cargo.toml` applies:

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

Use `cargo run` (debug) during development for faster compiles, `cargo run --release` for play sessions.

---

## Part 2: Bluetooth Controller Support

### 2.1 The macOS Gamepad Landscape

There are two ways Bevy can talk to controllers on macOS:

**Option 1: gilrs (Bevy's default)** — Cross-platform gamepad library. Works on macOS but has known limitations: historically incomplete d-pad support, analog trigger issues, and some Bluetooth controllers aren't detected reliably. It talks to IOKit/HID directly rather than going through Apple's framework.

**Option 2: `bevy_gamepad` plugin** — A community plugin that uses Apple's native Game Controller framework via Objective-C bindings. This is the better choice on macOS because Apple's framework is what actually manages Bluetooth controller pairing and has first-party support for all MFi, Xbox, PlayStation, and Switch controllers.

**Recommendation: Use `bevy_gamepad` on macOS, gilrs on Linux/Steam Deck.** Use conditional compilation to pick the right backend per platform.

### 2.2 Compatible Bluetooth Controllers

Apple's Game Controller framework (macOS 11+) officially supports:

| Controller | Bluetooth Pairing | Notes |
|------------|------------------|-------|
| **PS5 DualSense** | Hold PS + Create until light bar blinks | Best macOS support, all buttons work |
| **PS4 DualShock 4** | Hold PS + Share until light bar blinks | Fully supported |
| **Xbox Series X/S** | Hold pairing button until Xbox logo flashes | Supported on macOS 11.3+ |
| **Xbox One (Bluetooth model)** | Hold pairing button until Xbox logo flashes | Must be the 1708+ revision with Bluetooth |
| **Nintendo Switch Pro** | Hold sync button on top | Supported; button labels swapped (A/B, X/Y) |
| **Nintendo Joy-Cons** | Hold sync on rail; pair L+R separately | Show up as single gamepad when both paired |
| **MFi controllers** | Varies by model | Native Apple support |
| **8BitDo controllers** | Set to X-input or Apple mode | Most models work; check 8BitDo docs for mode |

**Not supported on macOS via Bluetooth:**
- Xbox 360 controllers (no Bluetooth — these are your wired Steam Deck controllers, won't work wirelessly on Mac)
- Xbox One controllers pre-2016 (no Bluetooth hardware)
- Generic/unbranded HID gamepads (Apple's framework only supports known controllers)

### 2.3 Pairing Controllers

1. Open **System Settings → Bluetooth** on the Mac
2. Put the controller in pairing mode (see table above)
3. It appears in the Bluetooth device list — click Connect
4. Verify: open **System Settings → Game Controllers** — the controller should appear

You can pair up to 4 Bluetooth controllers simultaneously on most Macs. The practical limit depends on your Bluetooth radio and wireless environment.

### 2.4 Setting Up `bevy_gamepad`

#### Cargo.toml

```toml
[dependencies]
bevy = { version = "0.15", default-features = false, features = [
    # List all features you need EXCEPT gilrs
    "bevy_asset",
    "bevy_audio",
    "bevy_color",
    "bevy_core_pipeline",
    "bevy_input",
    "bevy_log",
    "bevy_render",
    "bevy_scene",
    "bevy_sprite",
    "bevy_state",
    "bevy_text",
    "bevy_ui",
    "bevy_window",
    "bevy_winit",
    "default_font",
    "multi_threaded",
    "png",
    "tonemapping_luts",
    "x11",        # Linux
    "wayland",    # Linux
] }
leafwing-input-manager = "0.16"

# Apple Game Controller framework — macOS/iOS only
[target.'cfg(target_os = "macos")'.dependencies]
bevy_gamepad = "0.1"

# gilrs — Linux/Windows (for Steam Deck)
[target.'cfg(not(target_os = "macos"))'.dependencies]
bevy = { version = "0.15", features = ["gilrs"] }
```

**Key point:** We disable Bevy's default features to exclude `gilrs` on macOS, then conditionally add either `bevy_gamepad` (macOS) or the `gilrs` feature (Linux). This avoids conflicts between the two input backends.

#### Plugin Registration (main.rs)

```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Food Fight!".into(),
            resolution: bevy::window::WindowResolution::new(1280.0, 800.0),
            ..default()
        }),
        ..default()
    }));

    // Platform-specific gamepad backend
    #[cfg(target_os = "macos")]
    app.add_plugins(bevy_gamepad::GamepadPlugin);

    // On Linux/Windows, gilrs is included via Bevy's DefaultPlugins
    // when the "gilrs" feature is enabled — no extra plugin needed.

    app.add_plugins((
        // Your game plugins...
        PlayerPlugin,
        FoodPlugin,
        CombatPlugin,
        NpcPlugin,
        MapPlugin,
        UiPlugin,
    ));

    app.run();
}
```

Both backends feed into the same Bevy `Gamepad` component and `GamepadEvent` system. Your gameplay code doesn't need to know which backend is active — `leafwing-input-manager` and raw Bevy gamepad queries work identically either way.

---

## Part 3: Dynamic Controller Joining

This is the "press A to join" lobby system. It works the same on macOS and Steam Deck since it uses Bevy's platform-agnostic gamepad API.

### 3.1 Lobby State

```rust
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum LobbyState {
    #[default]
    WaitingForPlayers,
    AllReady,
}

#[derive(Resource, Default)]
pub struct Lobby {
    pub slots: Vec<PlayerSlot>,
}

pub struct PlayerSlot {
    pub player_id: u8,
    pub gamepad_entity: Entity,
    pub ready: bool,
    pub color: Color,           // Visual indicator per player
    pub display_name: String,   // "Player 1", "Player 2", etc.
}

// Colors for up to 4 players
const PLAYER_COLORS: [Color; 4] = [
    Color::srgb(0.2, 0.6, 1.0),   // Blue
    Color::srgb(1.0, 0.3, 0.3),   // Red
    Color::srgb(0.3, 1.0, 0.3),   // Green
    Color::srgb(1.0, 0.9, 0.2),   // Yellow
];
```

### 3.2 Join System

```rust
fn lobby_join_system(
    mut lobby: ResMut<Lobby>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut commands: Commands,
    // For spawning lobby UI elements
) {
    for (gamepad_entity, gamepad) in &gamepads {
        // Check if this gamepad pressed the South button (A / Cross)
        if !gamepad.just_pressed(GamepadButton::South) {
            continue;
        }

        // Skip if already joined
        let already_joined = lobby.slots.iter()
            .any(|slot| slot.gamepad_entity == gamepad_entity);
        if already_joined {
            continue;
        }

        // Reject if lobby full
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

        // TODO: Spawn lobby UI element showing the new player slot
    }
}
```

### 3.3 Ready System

```rust
fn lobby_ready_system(
    mut lobby: ResMut<Lobby>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut next_state: ResMut<NextState<LobbyState>>,
) {
    for (gamepad_entity, gamepad) in &gamepads {
        // Find this gamepad's slot
        let Some(slot) = lobby.slots.iter_mut()
            .find(|s| s.gamepad_entity == gamepad_entity) else {
            continue;
        };

        // Start/Y toggles ready
        if gamepad.just_pressed(GamepadButton::Start) {
            slot.ready = !slot.ready;
            info!("Player {} ready: {}", slot.player_id + 1, slot.ready);
        }

        // East button (B / Circle) to leave
        if gamepad.just_pressed(GamepadButton::East) {
            // Remove this player from lobby
            // (handle in a separate system to avoid borrow issues)
        }
    }

    // Check if all players are ready (minimum 2)
    let all_ready = lobby.slots.len() >= 2
        && lobby.slots.iter().all(|s| s.ready);

    if all_ready {
        next_state.set(LobbyState::AllReady);
    }
}
```

### 3.4 Spawning Players from Lobby

When transitioning from lobby to gameplay, spawn player entities using the lobby data:

```rust
fn spawn_players_from_lobby(
    lobby: Res<Lobby>,
    mut commands: Commands,
    // asset handles, spawn points, etc.
) {
    for slot in &lobby.slots {
        let input_map = create_gamepad_input_map(slot.gamepad_entity);

        commands.spawn((
            Player {
                id: slot.player_id,
                speed: 200.0,
            },
            Health(100.0),
            Velocity(Vec2::ZERO),
            Inventory { held_food: None },
            InputManagerBundle::with_map(input_map),
            Sprite {
                color: slot.color,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_translation(
                get_spawn_point(slot.player_id).extend(1.0)
            ),
        ));
    }
}

fn create_gamepad_input_map(gamepad_entity: Entity) -> InputMap<PlayerAction> {
    let mut map = InputMap::default();
    map.insert(PlayerAction::Move, DualAxis::left_stick());
    map.insert(PlayerAction::Aim, DualAxis::right_stick());
    map.insert(PlayerAction::Throw, GamepadButton::RightTrigger2);
    map.insert(PlayerAction::Pickup, GamepadButton::South);
    map.insert(PlayerAction::SwitchWeapon, GamepadButton::North);
    map.insert(PlayerAction::Pause, GamepadButton::Start);
    map.set_gamepad(gamepad_entity);
    map
}
```

### 3.5 Handling Disconnections Mid-Game

Bluetooth controllers can disconnect unexpectedly. Handle this gracefully:

```rust
fn handle_disconnections(
    mut connection_events: EventReader<GamepadConnectionEvent>,
    mut lobby: ResMut<Lobby>,
    player_query: Query<(Entity, &Player)>,
    mut commands: Commands,
    current_state: Res<State<GameState>>,
) {
    for event in connection_events.read() {
        if let GamepadConnection::Disconnected = &event.connection {
            let disconnected_entity = event.gamepad;

            // Find which player slot this was
            if let Some(slot) = lobby.slots.iter()
                .find(|s| s.gamepad_entity == disconnected_entity)
            {
                let player_id = slot.player_id;
                info!("Player {} disconnected!", player_id + 1);

                match current_state.get() {
                    GameState::Playing => {
                        // Pause the game and show "reconnect controller" prompt
                        // Don't remove the player — let them reconnect
                    }
                    _ => {
                        // In lobby/menus, just remove the slot
                        lobby.slots.retain(|s| s.gamepad_entity != disconnected_entity);
                    }
                }
            }
        }
    }
}

fn handle_reconnections(
    mut connection_events: EventReader<GamepadConnectionEvent>,
    mut lobby: ResMut<Lobby>,
) {
    for event in connection_events.read() {
        if let GamepadConnection::Connected(info) = &event.connection {
            info!("Controller reconnected: {}", info.name);
            // If we have a disconnected player slot, reassign this gamepad to it
            // (Simplest: just let the player press A to rejoin)
        }
    }
}
```

---

## Part 4: Lobby UI

The lobby screen needs clear visual feedback so kids know what's happening:

```rust
// Lobby screen layout:
//
//  ┌─────────────────────────────────────────────┐
//  │           🍕  FOOD FIGHT!  🍕               │
//  │                                              │
//  │   ┌──────────┐  ┌──────────┐                │
//  │   │ Player 1 │  │ Player 2 │                │
//  │   │  (Blue)  │  │  (Red)   │                │
//  │   │  READY ✓ │  │  Press   │                │
//  │   │          │  │  START   │                │
//  │   └──────────┘  └──────────┘                │
//  │                                              │
//  │   ┌──────────┐  ┌──────────┐                │
//  │   │          │  │          │                │
//  │   │  Press   │  │  Press   │                │
//  │   │  A to    │  │  A to   │                │
//  │   │  join    │  │  join    │                │
//  │   └──────────┘  └──────────┘                │
//  │                                              │
//  │        All players press START               │
//  │            when ready!                       │
//  └─────────────────────────────────────────────┘
```

Implementation details:

- Show 4 player slots always visible (empty ones show "Press A to join")
- When a gamepad presses A, animate the slot filling in with the player's color
- Show controller name in the slot (e.g., "DualSense Wireless Controller") so kids know which physical controller maps to which player
- Toggle "READY ✓" on Start press with a visual indicator
- When all joined players are ready, show a 3-2-1 countdown then transition to map select or gameplay
- If a player presses B, they leave and the slot goes back to empty

---

## Part 5: Cross-Platform Input Abstraction

Since you're targeting both Steam Deck (wired Xbox 360) and MacBook (Bluetooth mixed controllers), keep the input layer platform-agnostic.

### 5.1 Unified PlayerAction Enum

This is shared across all platforms — defined once in your `input` module:

```rust
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Aim,
    Throw,
    Pickup,
    SwitchWeapon,
    Pause,
}
```

### 5.2 Button Prompt Display

Different controllers have different button labels. The kids will get confused if the screen says "Press A" but they're holding a PlayStation controller where it's "Cross" (✕). Use the gamepad name to show correct prompts:

```rust
#[derive(Debug, Clone, Copy)]
pub enum ControllerFamily {
    Xbox,
    PlayStation,
    Nintendo,
    Unknown,
}

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

pub fn south_button_label(family: ControllerFamily) -> &'static str {
    match family {
        ControllerFamily::Xbox => "A",
        ControllerFamily::PlayStation => "✕",
        ControllerFamily::Nintendo => "B",  // Nintendo's South is B
        ControllerFamily::Unknown => "A",
    }
}

pub fn north_button_label(family: ControllerFamily) -> &'static str {
    match family {
        ControllerFamily::Xbox => "Y",
        ControllerFamily::PlayStation => "△",
        ControllerFamily::Nintendo => "X",
        ControllerFamily::Unknown => "Y",
    }
}

// ... etc. for East, West, triggers, etc.
```

Use this in your lobby UI and in-game button prompts:

```
"Press [✕] to pick up"   ← Player with DualSense
"Press [A] to pick up"   ← Player with Xbox controller
```

### 5.3 Mixed Controller Support

With the lobby system, each player has their own `InputMap` tied to a specific gamepad entity. This means you can have one kid on a DualSense, another on an Xbox controller, and a third on a Switch Pro Controller — all at the same time. Each controller's input maps to the same `PlayerAction` enum and the game logic doesn't care which hardware is behind it.

---

## Part 6: macOS-Specific Considerations

### 6.1 Retina / HiDPI

MacBooks have Retina displays (2x or 3x scaling). Bevy handles this automatically through its window scale factor. However, if your pixel art looks blurry, you may need to set nearest-neighbor sampling on your sprite textures:

```rust
// In your asset loading or texture atlas setup
app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
```

This ensures 32×32 pixel art stays crisp on Retina screens.

### 6.2 Window Sizing

MacBook screens are typically 2560×1600 (scaled) or 1440×900 (effective). The game should be windowed by default on macOS since you're not in a console-like environment:

```rust
#[cfg(target_os = "macos")]
let window = Window {
    title: "Food Fight!".into(),
    resolution: WindowResolution::new(1280.0, 800.0),
    resizable: true,
    ..default()
};

#[cfg(target_os = "linux")]
let window = Window {
    title: "Food Fight!".into(),
    resolution: WindowResolution::new(1280.0, 800.0)
        .with_scale_factor_override(1.0),
    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
    ..default()
};
```

### 6.3 App Bundle (Optional Polish)

If you want to distribute the game as a proper macOS `.app` bundle (double-click to launch, shows up in Dock with an icon), you can use `cargo-bundle`:

```bash
cargo install cargo-bundle
```

Add to `Cargo.toml`:

```toml
[package.metadata.bundle]
name = "Food Fight"
identifier = "com.yourfamily.foodfight"
icon = ["assets/icon/icon.icns"]
category = "public.app-category.games"
```

Then build with:

```bash
cargo bundle --release
```

This creates `target/release/bundle/osx/Food Fight.app`. You can drag this to `/Applications` or share it directly. Not required for development — just nice to have when you want to show it off.

---

## Part 7: Troubleshooting

### Controller pairs but game doesn't see it

- Make sure it shows up in **System Settings → Game Controllers**
- If using `bevy_gamepad`, verify you disabled the `gilrs` feature to avoid conflicts
- Try forgetting the device in Bluetooth settings and re-pairing
- Restart the game after pairing — some controllers aren't detected if they connect after the game starts (this depends on the backend)

### Controller buttons are wrong / swapped

- Nintendo controllers have swapped A/B and X/Y positions compared to Xbox. This is by design — Apple's Game Controller framework reports them positionally, not by label. Your `detect_controller_family` + label system handles the display side.
- If actual input mapping is wrong, check if your controller is in the correct mode (8BitDo controllers have a physical mode switch)

### Game runs slow on MacBook Air

- Make sure you're using `--release` builds
- MacBook Air's GPU is shared memory — it's capable but not a powerhouse. For a 2D game with Bevy sprites this shouldn't be an issue.
- If you see thermal throttling, reduce the target frame rate: `app.insert_resource(bevy::winit::WinitSettings { ..default() })` or limit to 60fps

### Audio doesn't play

- macOS may block audio for unsigned apps. Running from `cargo run` usually works fine, but if you distribute the `.app` bundle, you may need to right-click → Open the first time.

### Multiple controllers show as one / inputs overlap

- This is a known issue with some gilrs versions on macOS — another reason to use `bevy_gamepad` on this platform
- With `bevy_gamepad`, each controller gets a unique entity and inputs are properly separated

---

## Quick Reference

| Task | Command |
|------|---------|
| Build (debug) | `cargo run` |
| Build (release) | `cargo run --release` |
| Build macOS app bundle | `cargo bundle --release` |
| Pair controller | System Settings → Bluetooth |
| Verify controller | System Settings → Game Controllers |
| Cross-compile for Linux (Steam Deck) | Use Docker container (see STEAMDECK.md) |
