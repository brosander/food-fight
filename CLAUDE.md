# Cafeteria Food Fight

2D top-down multiplayer food fight game. Students battle with food in a school cafeteria while avoiding patrolling NPC authority figures. Built with Rust + Bevy 0.15.

## Tech Stack

- **Bevy 0.15** (ECS game engine)
- **rand 0.8** (food/launcher randomization)
- **gilrs** (gamepad input via Bevy's built-in `bevy_gilrs` feature — no extra crate)
- **No physics engine** — custom AABB collision
- **No tilemap crate yet** — map is procedurally spawned colored rectangles

## Build & Run

```bash
cargo run          # debug build (opt-level=1 for dev, opt-level=3 for deps)
cargo run --release
```

Platform-specific: Linux gets borderless fullscreen + scale factor override 1.0 (Steam Deck). macOS gets windowed 1280x800.

## Project Structure

```
src/
├── main.rs              # App setup, plugins, camera, window config
├── states.rs            # GameState enum, Gameplay marker component
├── controller.rs        # Gamepad utilities, debug logging, disconnect/reconnect handling
├── lobby/
│   ├── mod.rs           # Lobby resource, join/leave/ready systems, player spawning
│   └── ui.rs            # Lobby screen UI (2x2 slot grid)
├── player/
│   ├── mod.rs           # PlayerPlugin registration
│   ├── components.rs    # Player, Health, Velocity, Score, GamepadLink
│   ├── input.rs         # Gamepad left stick → velocity
│   ├── movement.rs      # Velocity → transform, bounds clamping
│   └── animation.rs     # Placeholder scale-pulse (no sprite sheets yet)
├── food/
│   ├── mod.rs           # FoodPlugin registration
│   ├── components.rs    # FoodType, FoodItem, Throwable, InFlight, ArcFlight, BounceFlight, Inventory, etc.
│   ├── spawning.rs      # Food spawn points, respawn timers, initial spawn
│   ├── throwing.rs      # Pickup (South button) and throw (RT + right stick aim)
│   ├── launcher.rs      # Launcher types, pickup (West), fire, catapult charge
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
    ├── mod.rs           # UiPlugin, main menu, pause, cleanup systems
    ├── hud.rs           # Health bars, status text per player (dynamic from lobby)
    └── scoreboard.rs    # Win check, round-over screen
```

## Game State Flow

```
MainMenu → Lobby → Playing ⇄ Paused
                  Playing → RoundOver → MainMenu
```

States defined in `states.rs`: `MainMenu` (default), `Lobby`, `MapSelect` (unused), `Loading` (unused), `Playing`, `Paused`, `RoundOver`.

The `Gameplay` marker component is added to all in-game entities and used for bulk cleanup on return to MainMenu.

## Plugin Registration Order (main.rs)

1. `DefaultPlugins` (with `ImagePlugin::default_nearest()` for pixel art)
2. `ControllerPlugin` — gamepad debug logging, disconnect → auto-pause, reconnect logging
3. `LobbyPlugin` — lobby UI, join/leave/ready, spawns players on `OnEnter(Playing)`
4. `PlayerPlugin` — input, movement, animation (FixedUpdate, gated on Playing)
5. `FoodPlugin` — food spawns, throwing, launchers, trajectories (FixedUpdate, gated on Playing)
6. `CombatPlugin` — food-player collision (FixedUpdate, gated on Playing)
7. `NpcPlugin` — NPC spawning, detection, patrol, chase, catch (FixedUpdate, gated on Playing)
8. `MapPlugin` — cafeteria spawn, wall collision (FixedUpdate, gated on Playing)
9. `UiPlugin` — main menu, HUD, pause, round over, cleanup

## Input Scheme

**Gamepad (primary):**
- Left stick: move
- Right stick: aim direction (falls back to left stick)
- South (A/Cross/B): pick up food
- West (X/Square/Y): pick up launcher
- RightTrigger2 (RT): throw food / fire launcher
- Start: menu navigation, pause/unpause, ready up in lobby
- East (B/Circle/A): leave lobby

**Keyboard fallbacks (for testing when controllers don't work):**
- Space: navigate menus, quick-start lobby (auto-joins all gamepads), unpause
- Escape: pause

**Controller detection:** `ControllerFamily` enum (Xbox/PlayStation/Nintendo/Unknown) with per-family button labels. Detected from gamepad name string.

**Known issue:** Nintendo Switch Pro Controller via 8BitDo UM2 USB dongle connects (detected as "8BitDo UM 2 Receiver") but gilrs delivers no button/axis events on macOS. `debug_gamepads` system logs raw `GamepadButtonChangedEvent`/`GamepadAxisChangedEvent` for debugging this.

## Key Architecture Patterns

### Player Spawning
Players are NOT hardcoded. The `Lobby` resource (`Vec<PlayerSlot>`) manages join/leave. `spawn_players_from_lobby` runs on `OnEnter(Playing)` and creates player entities with `GamepadLink(Entity)` binding each player to their specific gamepad entity.

### Gamepad Input
All gameplay input goes through `GamepadLink(Entity)` → `Query<&Gamepad>`. Utility functions in `controller.rs`: `read_left_stick()`, `read_right_stick()`, `read_aim_direction()` (all apply 0.15 deadzone).

### Food System
8 food types with distinct stats (damage, speed, trajectory kind). 5 launcher types with cooldowns, multipliers, and limited uses. Three trajectory systems: straight, arc (simulated Z with gravity), bounce (wall reflection). Food spawns at fixed points with 5-second respawn timers.

### NPC State Machine
`NpcState` enum: `Patrolling` → `Suspicious` → `Chasing` → `Returning` → `Patrolling`. Detection uses cone check (angle + distance). NPCs change sprite color based on state (yellow=suspicious, red=chasing). Three NPCs: Teacher (medium speed, patrols tables), Principal (slow, wide detection), Lunch Lady (stationary at counter).

### Map Layout
Procedural cafeteria: 960x640 play area centered at origin. Bounds: ±480 x, ±320 y. Perimeter walls (16px thick), 6 tables in 2x3 grid, lunch counter at top, 2 pillars, 2 door markers. Everything uses the `Wall { half_size }` component for AABB collision.

## Assets

**No sprite assets exist yet.** Directory structure is stubbed:
```
assets/sprites/
├── effects/    (empty)
├── food/       (empty)
├── npcs/       (empty)
├── players/    (empty)
└── ui/         (empty)
```

All entities currently render as colored rectangles:
- Players: 32x32, color from lobby slot (Blue/Red/Green/Yellow)
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
- [x] Controller support: gamepad input throughout, lobby system, keyboard fallbacks
- [ ] Sprite art: all entities are colored rectangles, no PNGs loaded
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

## Reference Docs

- `PLAN.md` — original phased implementation plan with component designs and acceptance criteria
- `MACBOOK.md` — guide for Bluetooth controller support on macOS
- `STEAMDECK.md` — Steam Deck deployment guide
