# Cafeteria Food Fight

2D top-down multiplayer food fight game. Students battle with food in a school cafeteria while avoiding patrolling NPC authority figures. Built with Rust + Bevy 0.15.

## Tech Stack

- **Bevy 0.15** (ECS game engine)
- **rand 0.8** (food/launcher randomization)
- **gilrs** (gamepad input via Bevy's built-in `bevy_gilrs` feature — default backend)
- **steamworks 0.12.2** (Steam Input API — optional, enabled with `--features steam`)
- **No physics engine** — custom AABB collision
- **No tilemap crate yet** — map is procedurally spawned colored rectangles

## Build & Run

```bash
cargo run                    # gilrs gamepad backend (default)
cargo run --features steam   # Steam Input backend (requires Steam running)
cargo run --release
```

Platform-specific: Linux gets borderless fullscreen + scale factor override 1.0 (Steam Deck). macOS gets windowed 1280x800.

## Project Structure

```
src/
├── main.rs              # App setup, plugins, camera, window config
├── states.rs            # GameState enum, Gameplay/GameSessionActive markers
├── input.rs             # ControllerInput, ControllerRegistry, ControllerId, InputPlugin (gilrs path)
├── steam.rs             # SteamInputPlugin — Steam Input action sets, controller polling (feature-gated)
├── controller.rs        # Gamepad utilities (read_left/right_stick), debug logging, disconnect/reconnect
├── sprites.rs           # SpritePlugin, SpriteAssets, AnimationState, frame-based animation
├── lobby/
│   ├── mod.rs           # Lobby resource + PlayerSlot, join/leave/ready systems, player spawning
│   └── ui.rs            # Lobby screen UI (2x2 slot grid)
├── player/
│   ├── mod.rs           # PlayerPlugin registration
│   ├── components.rs    # Player, Health, Velocity, Score, ControllerLink
│   ├── input.rs         # ControllerInput.move_stick → Velocity
│   ├── movement.rs      # Velocity → transform, bounds clamping
│   └── animation.rs     # Placeholder scale-pulse (no sprite sheets yet)
├── food/
│   ├── mod.rs           # FoodPlugin registration
│   ├── components.rs    # FoodType, FoodItem, Throwable, InFlight, ArcFlight, BounceFlight, Inventory, etc.
│   ├── spawning.rs      # Food spawn points, respawn timers, initial spawn
│   ├── throwing.rs      # Pickup (pickup_food) and throw (fire + aim_direction)
│   ├── launcher.rs      # Launcher types, pickup (pickup_launcher), fire, catapult charge
│   └── trajectory.rs    # Straight, arc, bounce movement + splat fade
├── combat/
│   ├── mod.rs           # CombatPlugin
│   └── collision.rs     # InFlight vs Player AABB collision, damage, splat effects
├── npc/
│   ├── mod.rs           # NpcPlugin, spawn_npcs, visual feedback (color changes by state)
│   ├── components.rs    # NpcRole, NpcAuthority, NpcState, PatrolPath, Facing, Suspicious, Caught
│   ├── detection.rs     # Suspicion marking, detection cone, line-of-sight
│   ├── patrol.rs        # Waypoint following, returning to patrol
│   └── chase.rs         # Chase movement, catch system, caught stun/penalty
├── map/
│   ├── mod.rs           # MapPlugin, Wall component
│   ├── loading.rs       # Procedural cafeteria: floor, walls, tables, counter, pillars, doors
│   └── collision.rs     # Wall push-out for players, projectile-wall despawn
└── ui/
    ├── mod.rs           # UiPlugin, main menu, pause (with exit option), cleanup systems
    ├── hud.rs           # Health bars, status text per player (dynamic from lobby)
    └── scoreboard.rs    # Win check, round-over screen
```

## Game State Flow

```
MainMenu → Lobby → Playing ⇄ Paused
                  Playing → RoundOver → MainMenu
```

States defined in `states.rs`: `MainMenu` (default), `Lobby`, `MapSelect` (stub), `Loading` (stub), `Playing`, `Paused`, `RoundOver`.

The `Gameplay` marker component is added to all in-game entities and used for bulk cleanup on return to MainMenu.

`GameSessionActive` is a marker resource inserted once on the first `OnEnter(Playing)` transition. It prevents spawn/setup systems from re-running when unpausing.

## Plugin Registration Order (main.rs)

1. `DefaultPlugins` (with `ImagePlugin::default_nearest()` for pixel art)
2. `SpritePlugin` — loads sprite assets (must precede gameplay plugins)
3. `SteamInputPlugin` (feature-gated) — must precede `InputPlugin`; populates `ControllerRegistry` via Steam Input
4. `InputPlugin` — registers `ControllerRegistry`, runs gilrs population system in `PreUpdate`
5. `ControllerPlugin` — gamepad debug logging, disconnect → auto-pause, reconnect logging
6. `LobbyPlugin` — lobby UI, join/leave/ready, spawns players on `OnEnter(Playing)`
7. `PlayerPlugin` — input→velocity, movement, animation (FixedUpdate, gated on Playing)
8. `FoodPlugin` — food spawns, throwing, launchers, trajectories (FixedUpdate, gated on Playing)
9. `CombatPlugin` — food-player collision (FixedUpdate, gated on Playing)
10. `NpcPlugin` — NPC spawning, detection, patrol, chase, catch (FixedUpdate, gated on Playing)
11. `MapPlugin` — cafeteria spawn, wall collision (FixedUpdate, gated on Playing)
12. `UiPlugin` — main menu, HUD, pause, round over, cleanup

## Input Architecture

All input flows through a backend-agnostic abstraction in `src/input.rs`.

### ControllerInput (component on player entities)

```rust
pub struct ControllerInput {
    pub move_stick: Vec2,      // left stick, 0.15 deadzone applied
    pub aim_stick: Vec2,       // right stick, 0.15 deadzone applied
    pub pickup_food: ButtonState,
    pub pickup_launcher: ButtonState,
    pub fire: ButtonState,     // throw food / fire launcher
    pub pause: ButtonState,    // pause/unpause, ready-up, menu select
    pub join: ButtonState,     // lobby join / menu confirm (South)
    pub leave: ButtonState,    // lobby leave / menu back (East)
    pub exit_game: ButtonState, // quit app (Select)
}
// ButtonState has: pressed, just_pressed, just_released
// ControllerInput::aim_direction() → right stick, falls back to left stick, default Vec2::Y
```

Gameplay systems read `ControllerInput` directly — never query `Gamepad` or use Steam Input handles.

### ControllerRegistry (resource)

```rust
pub struct ControllerRegistry {
    pub controllers: Vec<RegisteredController>,  // id: ControllerId, input: ControllerInput
}
```

Menu/lobby systems iterate `registry.controllers` to detect any-controller input (join, leave, ready). Populated each frame in `PreUpdate` by whichever backend is active.

### ControllerId (enum)

```rust
pub enum ControllerId {
    Bevy(Entity),             // gilrs backend: Bevy gamepad entity
    #[cfg(feature = "steam")]
    Steam(u64),               // Steam backend: InputHandle_t
}
```

Stored in `PlayerSlot::controller_id` (lobby) and `ControllerLink` (player entity component). Backend switching: when Steam Input detects controllers, `steam_input_populate` replaces the registry contents; gilrs data is ignored for those frames.

### Gilrs (default) path

`gilrs_populate_system` in `PreUpdate`:
- Rebuilds `ControllerRegistry` from all `Query<(Entity, &Gamepad)>`
- Updates `ControllerInput` on player entities via their `ControllerLink`
- Button mapping: South=pickup_food/join, West=pickup_launcher, RT=fire, Start=pause, East=leave, Select=exit_game

### Steam Input path (`--features steam`)

`SteamInputPlugin` in `src/steam.rs`:
- `Client::init_app(480)` → `SteamAppClient` resource (480 = SpaceWar dev app ID)
- Action sets: `GameplayControls` (move/aim/pickup_food/pickup_launcher/fire/pause) and `MenuControls` (navigate/join/leave/pause/exit_game) — defined in `steam_input_manifest.vdf`
- Action set switching: `OnEnter(Playing)` activates `GameplayControls`, all other states activate `MenuControls`
- `steam_input_populate` in `PreUpdate` (after `run_steam_callbacks`): clears registry, re-populates from `get_connected_controllers()`
- Previous-frame tracking (`SteamPrevState`) for `just_pressed`/`just_released` edge detection (Steam only gives current state)

### Gamepad button → ControllerInput mapping

| Physical button | gilrs `GamepadButton` | Steam action name |
|---|---|---|
| Left stick | `LeftStickX/Y` | `move` (analog) |
| Right stick | `RightStickX/Y` | `aim` (analog) |
| South (A/X/B) | `South` | `pickup_food` / `join` |
| West (X/□/Y) | `West` | `pickup_launcher` |
| RT | `RightTrigger2` | `fire` |
| Start | `Start` | `pause` |
| East (B/○/A) | `East` | `leave` |
| Select | `Select` | `exit_game` |

### Keyboard fallbacks

- Space: navigate menus, quick-start lobby (auto-joins all controllers), unpause
- Escape: pause
- Q: exit game (from pause screen)

## Key Architecture Patterns

### Player Spawning

Players are NOT hardcoded. The `Lobby` resource (`Vec<PlayerSlot>`) manages join/leave. `spawn_players_from_lobby` runs on `OnEnter(Playing)` (guarded by `GameSessionActive` to skip on unpause) and creates player entities with:
- `ControllerLink(slot.controller_id)` — binds player to controller
- `ControllerInput::default()` — populated each frame by the input backend

### Food System

8 food types with distinct stats (damage, speed, trajectory kind). 5 launcher types with cooldowns, multipliers, and limited uses. Three trajectory systems: straight, arc (simulated Z with gravity), bounce (wall reflection). Food spawns at fixed points with 5-second respawn timers.

### NPC State Machine

`NpcState` enum: `Patrolling` → `Suspicious` → `Chasing` → `Returning` → `Patrolling`. Detection uses cone check (angle + distance). NPCs change sprite color based on state (yellow=suspicious, red=chasing). Three NPCs: Teacher (medium speed, patrols tables), Principal (slow, wide detection), Lunch Lady (stationary at counter). Janitor is a planned fourth role (not yet spawned).

### Map Layout

Procedural cafeteria: 960x640 play area centered at origin. Bounds: ±480 x, ±320 y. Perimeter walls (16px thick), 6 tables in 2x3 grid, lunch counter at top, 2 pillars, 2 door markers. Everything uses the `Wall { half_size }` component for AABB collision.

## Assets

Sprite atlas loaded from `assets/sprites/players/player{0-3}.png`. NPCs and food still render as colored rectangles.

All entities currently render as colored rectangles except players:
- Players: 32x32, sprite atlas from lobby slot color
- NPCs: 28-32px, color by role (Teacher=tan, Principal=blue, LunchLady=pink)
- Food: 6-16px, color by type (Pizza=yellow, Meatball=brown, Jello=green, etc.)
- Launchers: 6-20px, color by type
- Map: floor=dark brown, walls=brown, tables=tan, counter=beige
- NPC visual feedback overrides color: yellow when suspicious, red when chasing

## Constants & Play Area

- Play area: 960x640 (±480x, ±320y from center)
- Player size: 32x32 (half_size=16 for collision)
- Player speed: 200 px/s
- Wall thickness: 16px
- Pickup range: 40px
- Food spawn respawn: 5 seconds
- Projectile max range: 400-600px
- NPC detection: Teacher 150px/60deg, Principal 200px/90deg, LunchLady 80px/180deg
- NPC chase speed: 1.3x normal
- Catch stun: Teacher 3s, Principal 5s, LunchLady 2s

## Z-Layer Ordering

- -1.0: Floor
- -0.5: Door markers
- 0.0: Walls, tables, counter
- 0.1: Splat decals
- 0.5: Food items on ground
- 0.6: Launcher pickups
- 1.0: Players
- 1.5: NPCs
- 2.0: Projectiles in flight
- 3.0: Hit effects

## What's Implemented (Phase Status)

- [x] Phase 1: Core movement & rendering (placeholder art)
- [x] Phase 2: Food throwing (all 8 types, 3 trajectory kinds)
- [x] Phase 3: Launchers (5 types, cooldowns, charge-up catapult)
- [x] Phase 4: Map (procedural cafeteria, wall collision) — NO tilemap, no Tiled
- [x] Phase 5: NPCs (3 roles, state machine, detection, chase, catch, penalties)
- [x] Phase 6: Game flow & UI (menus, HUD, pause, round over, lobby)
- [x] Input abstraction: ControllerInput/ControllerRegistry, gilrs + Steam Input backends
- [x] Steam Deck: borderless fullscreen on Linux, steamdeck.sh setup script
- [ ] Sprite art: players have atlas sprites; NPCs/food are still colored rectangles
- [ ] Audio: none
- [ ] Additional maps: only cafeteria
- [ ] Polish: no particles, screen shake, or animations beyond scale-pulse

## Bevy 0.15 API Notes

These tripped us up and will trip you up too:

- `GamepadConnectionEvent` and `GamepadConnection` are NOT in `bevy::prelude::*` — import from `bevy::input::gamepad`
- `GamepadConnection::Connected` is a **struct variant** not a tuple: `Connected { name, vendor_id, product_id }` — destructure with `{ name, .. }`
- Gamepad input: `Query<(Entity, &Gamepad)>`, then `gamepad.just_pressed(GamepadButton::South)`, `gamepad.get(GamepadAxis::LeftStickX)`
- Sprite with image: `Sprite { image: asset_server.load("path.png"), custom_size: Some(...), ..default() }`
- `ImagePlugin::default_nearest()` is set for pixel-perfect rendering
- Use `FixedUpdate` for gameplay, `Update` for UI
- State gating: `.run_if(in_state(GameState::Playing))`
- Event sending: `EventWriter::send(event)` — NOT `.write()` (that method does not exist in 0.15)

## steamworks 0.12.2 API Notes

These are different from older `steamworks-rs` examples you'll find online:

- `Client` has **no generic parameters** — it's just `Client`, not `Client<ClientManager>`
- `Client::init_app(app_id)` returns `Result<Client, SteamAPIInitError>` — **single value**, not a tuple `(Client, SingleClient)`
- There is **no `SingleClient`** — `run_callbacks()` is a method on `Client` directly
- `Client` is `Send + Sync` — store as a regular Bevy `Resource`, call `run_callbacks()` from any thread
- Handle types (`InputHandle_t`, `InputActionSetHandle_t`, etc.) live in `steamworks::sys`, not the top-level crate — requires `features = ["raw-bindings"]` in `Cargo.toml`
- `InputAnalogActionData_t` fields: `.x` and `.y` (lowercase)
- `InputDigitalActionData_t` field: `.bState` (camelCase — matches the C struct)
- `Input` struct also has no generic parameters: `steamworks::Input`, not `Input<ClientManager>`
- steam_appid.txt with value `480` in the working directory lets you run without Steam launching the app (uses Valve's SpaceWar test app)

## Reference Docs

- `PLAN.md` — original phased implementation plan with component designs and acceptance criteria
- `MACBOOK.md` — guide for Bluetooth controller support on macOS
- `STEAMDECK.md` — Steam Deck deployment guide
- `steamdeck.sh` — automated setup/build/run script for Steam Deck desktop mode
- `steam_input_manifest.vdf` — IGA action set definitions for Steam Input
