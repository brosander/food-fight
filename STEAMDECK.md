# Steam Deck Build & Run Guide

Guide for building the food fight game directly on the Steam Deck and playing it with wired Xbox 360 controllers via a USB hub.

---

## Hardware Setup

- Steam Deck (any model)
- USB-C hub/dock plugged into the Deck
- Wired Xbox 360 controllers plugged into the hub
- (Optional) HDMI to a TV/monitor from the hub for couch play

The Steam Deck's native resolution is **1280×800**. If docked to a TV, the Deck will output at the TV's resolution but the game should handle this automatically.

---

## Part 1: Development Environment on the Steam Deck

SteamOS has an immutable filesystem — system packages get wiped on OS updates. We use **Distrobox** to create a persistent Ubuntu container for our Rust build toolchain. Distrobox containers share your home directory, USB devices, display, and audio with the host, so the game runs natively on the Deck's GPU.

### 1.1 Initial Setup (Desktop Mode)

Switch to Desktop Mode (hold Power → Switch to Desktop).

Open Konsole (the terminal app) and set a password if you haven't:

```bash
passwd
```

### 1.2 Install Distrobox + Podman

Distrobox comes pre-installed on recent SteamOS versions. Verify:

```bash
distrobox --version
```

If it's missing or you want the latest version:

```bash
# Add ~/.local/bin to PATH (add to ~/.bashrc for persistence)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Install distrobox to ~/.local (survives SteamOS updates)
curl -s https://raw.githubusercontent.com/89luca89/distrobox/main/install | sh -s -- --prefix ~/.local

# Install podman to ~/.local (survives SteamOS updates)
curl -s https://raw.githubusercontent.com/89luca89/distrobox/main/extras/install-podman | sh -s -- --prefix ~/.local
```

### 1.3 Create an Ubuntu Container

```bash
distrobox create --name gamedev --image ubuntu:24.04
distrobox enter gamedev
```

The first run pulls the image and sets up the container. You'll land in an Ubuntu shell with full access to your home directory.

### 1.4 Install Rust and Bevy Dependencies Inside the Container

```bash
# System packages needed for Bevy
sudo apt update
sudo apt install -y \
    build-essential \
    g++ \
    pkg-config \
    libx11-dev \
    libasound2-dev \
    libudev-dev \
    libxkbcommon-x11-0 \
    libwayland-dev \
    libxkbcommon-dev \
    libvulkan-dev \
    curl \
    git

# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Verify
rustc --version
cargo --version
```

### 1.5 Clone and Build the Game

```bash
# Inside the distrobox container
cd ~
git clone <your-repo-url> food_fight
cd food_fight

# First build (slow — compiles all of Bevy, go get a snack)
cargo build --release

# Subsequent builds are fast
cargo run --release
```

The game window will appear on the Deck's screen — Distrobox shares the display server.

### 1.6 Quick-Entry Shortcut

Add a convenience alias to `~/.bashrc` (shared between host and container):

```bash
alias gamedev='distrobox enter gamedev'
alias foodfight='distrobox enter gamedev -- bash -c "cd ~/food_fight && cargo run --release"'
```

---

## Part 2: Xbox 360 Wired Controller Support

### 2.1 Drivers (Already Done)

Wired Xbox 360 controllers use the `xpad` kernel driver, which is built into the SteamOS kernel. There is nothing to install. Plug your controllers into the USB hub and they are immediately recognized as gamepad devices. The Deck handles this at the OS level — Distrobox containers inherit USB device access automatically.

### 2.2 Verify Controllers Are Detected

From a terminal (host or distrobox, either works):

```bash
# List input devices — you should see your Xbox controllers
cat /proc/bus/input/devices | grep -A 4 "Xbox"

# Or install and use evtest for detailed testing
sudo apt install evtest    # inside distrobox
evtest                     # pick your controller, press buttons to verify
```

Each wired controller gets its own device and Bevy assigns a unique `Gamepad` ID to each one.

### 2.3 Code Changes: Input System

Your current plan uses keyboard + mouse. For Xbox 360 controllers you need gamepad support. Add `leafwing-input-manager` for clean input abstraction.

#### Cargo.toml Addition

```toml
[dependencies]
leafwing-input-manager = "0.16"   # match your Bevy version
```

#### Define Actions

```rust
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    // Dual-axis actions (sticks)
    Move,           // Left stick — movement
    Aim,            // Right stick — aim direction

    // Button actions
    Throw,          // Right trigger (RT) — throw food / fire launcher
    Pickup,         // A button — pick up food / launcher
    SwitchWeapon,   // Y button — swap between hand-throw and launcher
    Pause,          // Start button — pause game
}
```

#### Create Input Maps Per Player

```rust
use bevy::input::gamepad::Gamepad;

fn create_gamepad_input_map(gamepad: Gamepad) -> InputMap<PlayerAction> {
    let mut map = InputMap::default();

    // Sticks
    map.insert(PlayerAction::Move, DualAxis::left_stick());
    map.insert(PlayerAction::Aim, DualAxis::right_stick());

    // Xbox 360 button mappings
    map.insert(PlayerAction::Throw, GamepadButton::RightTrigger2);     // RT
    map.insert(PlayerAction::Pickup, GamepadButton::South);            // A
    map.insert(PlayerAction::SwitchWeapon, GamepadButton::North);      // Y
    map.insert(PlayerAction::Pause, GamepadButton::Start);

    map.set_gamepad(gamepad);
    map
}
```

#### Dynamic Controller Assignment ("Press A to Join")

```rust
#[derive(Resource, Default)]
pub struct ControllerAssignments {
    pub assignments: Vec<(u8, Gamepad)>,   // (player_id, gamepad)
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum LobbyState {
    #[default]
    WaitingForPlayers,
    Ready,
}

fn handle_controller_join(
    mut assignments: ResMut<ControllerAssignments>,
    gamepads: Query<(Entity, &Gamepad)>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut commands: Commands,
) {
    for (entity, gamepad) in &gamepads {
        // Check if this gamepad pressed A and isn't already assigned
        let a_button = GamepadButton {
            gamepad: *gamepad,
            button_type: GamepadButtonType::South,
        };

        if buttons.just_pressed(a_button) {
            let already_assigned = assignments.assignments
                .iter()
                .any(|(_, g)| *g == *gamepad);

            if !already_assigned {
                let player_id = assignments.assignments.len() as u8;
                assignments.assignments.push((player_id, *gamepad));

                // Spawn the player entity with their input map
                commands.spawn((
                    Player { id: player_id, speed: 200.0 },
                    Health(100.0),
                    Velocity(Vec2::ZERO),
                    Inventory { held_food: None },
                    InputManagerBundle::with_map(
                        create_gamepad_input_map(*gamepad)
                    ),
                    // ... sprite, transform, etc.
                ));

                println!("Player {} joined with gamepad {:?}", player_id, gamepad);
            }
        }
    }
}
```

#### Using Actions in Gameplay Systems

Systems become input-source agnostic:

```rust
fn movement_system(
    mut query: Query<(&ActionState<PlayerAction>, &Player, &mut Velocity)>,
) {
    for (action_state, player, mut vel) in &mut query {
        if let Some(axis_pair) = action_state.axis_pair(&PlayerAction::Move) {
            let input = axis_pair.xy();
            if input.length() > 0.15 {   // dead zone
                vel.0 = input.normalize() * player.speed;
            } else {
                vel.0 = Vec2::ZERO;
            }
        }
    }
}

fn aim_system(
    mut query: Query<(&ActionState<PlayerAction>, &mut AimDirection)>,
) {
    for (action_state, mut aim) in &mut query {
        if let Some(axis_pair) = action_state.axis_pair(&PlayerAction::Aim) {
            let stick = axis_pair.xy();
            if stick.length() > 0.2 {   // dead zone — don't snap aim when idle
                aim.0 = stick.normalize();
            }
            // When stick is neutral, keep the last aim direction
        }
    }
}

fn throw_system(
    query: Query<(&ActionState<PlayerAction>, &Transform, &AimDirection, &mut Inventory)>,
    mut commands: Commands,
) {
    for (action_state, transform, aim, mut inventory) in &query {
        if action_state.just_pressed(&PlayerAction::Throw) {
            if let Some(food_type) = inventory.held_food.take() {
                // Spawn projectile in aim direction
                // ... (same logic as before, but using aim.0 instead of mouse position)
            }
        }
    }
}
```

### 2.4 Xbox 360 Button Reference

For reference when setting up UI prompts and button hints:

| Xbox 360 Button | Bevy GamepadButtonType | Game Action |
|-----------------|----------------------|-------------|
| A (green)       | `South`              | Pick up     |
| B (red)         | `East`               | Cancel / Drop |
| X (blue)        | `West`               | Special     |
| Y (yellow)      | `North`              | Switch weapon |
| LB              | `LeftTrigger`        | —           |
| RB              | `RightTrigger`       | —           |
| LT              | `LeftTrigger2`       | —           |
| RT              | `RightTrigger2`      | Throw / Fire |
| Start           | `Start`              | Pause       |
| Back            | `Select`             | Scoreboard  |
| Left Stick      | `LeftStickX/Y`       | Move        |
| Right Stick     | `RightStickX/Y`      | Aim         |
| L3 (click)      | `LeftThumb`          | —           |
| R3 (click)      | `RightThumb`         | —           |

---

## Part 3: Window Configuration for Steam Deck

### 3.1 Resolution and DPI Fix

The Deck reports incorrect DPI scaling in Gaming Mode. Set explicit overrides:

```rust
use bevy::window::{Window, WindowPlugin, WindowResolution, WindowMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Food Fight!".into(),
                resolution: WindowResolution::new(1280.0, 800.0)
                    .with_scale_factor_override(1.0),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        // ... rest of plugins
        .run();
}
```

Use `BorderlessFullscreen` — it avoids title bar/taskbar issues in Gaming Mode and handles docked (TV) resolution automatically.

### 3.2 Performance Profile

The Deck's APU is capable but has a power budget. In your `Cargo.toml`, make sure you have the fast-compile + optimized-deps config:

```toml
# Fast iteration during development
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

# Full optimization for play sessions
[profile.release]
opt-level = 3
lto = "thin"
```

Always use `cargo run --release` for actual play sessions. Debug builds will be too slow on the Deck's CPU.

---

## Part 4: Running the Game in Gaming Mode

You have two options for launching the game without switching to Desktop Mode every time.

### Option A: Add as Non-Steam Game (Recommended)

1. In Desktop Mode, open Steam
2. Click **Add a Game** → **Add a Non-Steam Game**
3. Click **Browse** and navigate to your compiled binary:
   ```
   /home/deck/food_fight/target/release/food_fight
   ```
4. Add it. It now appears in your Steam library.
5. Switch to Gaming Mode — the game shows up in your library and can be launched with controllers.

**Important:** If you rebuild, the binary is replaced in-place at the same path, so the Steam shortcut stays valid. No need to re-add.

### Option B: Launch Script (for Distrobox Builds)

If the binary is built inside Distrobox and needs the container's libraries at runtime, create a launch script:

```bash
#!/bin/bash
# ~/food_fight_launch.sh
cd /home/deck/food_fight
distrobox enter gamedev -- ./target/release/food_fight
```

```bash
chmod +x ~/food_fight_launch.sh
```

Add `food_fight_launch.sh` as the non-Steam game instead of the binary directly.

In practice, if you build with `--release` and the Deck's host has compatible Vulkan drivers (it does), the binary usually runs fine outside the container too. Try Option A first.

### Option C: Desktop Mode Only

Just run `cargo run --release` from Konsole inside the distrobox. The controllers work in Desktop Mode too. This is the simplest option during active development.

---

## Part 5: Development Workflow

### Recommended Daily Flow

1. **Switch to Desktop Mode** (hold Power → Switch to Desktop)
2. Open Konsole
3. Enter your dev container: `distrobox enter gamedev`
4. Navigate to project: `cd ~/food_fight`
5. Edit code (use `nano`, `vim`, or install VS Code inside the container)
6. Build and test: `cargo run --release`
7. Controllers work immediately — plug in and play
8. When done, `cargo build --release` and switch to Gaming Mode to test via Steam library

### Remote Development (Better Ergonomics)

If editing on the Deck's screen/keyboard is painful, develop on your main PC and sync to the Deck:

```bash
# From your dev machine — push code to the Deck over SSH
rsync -avz --exclude target/ ./food_fight/ deck@<deck-ip>:~/food_fight/

# Or just push to your git repo from your PC, pull on the Deck
# Inside distrobox on the Deck:
cd ~/food_fight && git pull && cargo run --release
```

Enable SSH on the Deck:

```bash
# On the Deck (host, not distrobox)
sudo systemctl enable sshd
sudo systemctl start sshd
```

This lets you code on a full keyboard/monitor and just build+test on the Deck.

---

## Part 6: Troubleshooting

### Controllers not detected

```bash
# Check that xpad driver is loaded
lsmod | grep xpad

# If not loaded:
sudo modprobe xpad

# Check input devices
ls /dev/input/js*
# Should show /dev/input/js0, js1, etc. for each controller
```

### Bevy doesn't see the gamepads

Bevy uses the `gilrs` crate for gamepad input. Make sure `libudev-dev` is installed in your distrobox (covered in Part 1). If you build without it, gilrs can't enumerate devices.

Rebuild after installing:
```bash
cargo clean && cargo build --release
```

### Game runs but screen is black / garbled in Gaming Mode

This is the DPI scaling bug. Make sure you have `with_scale_factor_override(1.0)` set (Part 3.1). Also try `Windowed` mode instead of `BorderlessFullscreen` as a diagnostic.

### Game is slow

Make sure you're running `--release`. Debug builds of Bevy are 10–20x slower due to unoptimized ECS queries and rendering.

### USB hub doesn't power controllers

Some hubs don't provide enough power for multiple Xbox 360 wired controllers (they draw ~500mA each). If controllers disconnect intermittently, try a **powered** USB hub or reduce to fewer controllers.

### Audio doesn't work

Distrobox uses the host's PipeWire/PulseAudio. If there's no sound, check your `~/.distroboxrc` includes:

```bash
export PIPEWIRE_RUNTIME_DIR=/dev/null
```

This forces PulseAudio compatibility mode which is more reliable in containers.

---

## Quick Reference

| Task | Command |
|------|---------|
| Enter dev container | `distrobox enter gamedev` |
| Build (debug) | `cargo build` |
| Build (release) | `cargo build --release` |
| Run game | `cargo run --release` |
| Test controllers | `evtest` |
| List gamepads | `ls /dev/input/js*` |
| Sync from PC | `rsync -avz --exclude target/ ./food_fight/ deck@IP:~/food_fight/` |
| SSH into Deck | `ssh deck@<deck-ip>` |
