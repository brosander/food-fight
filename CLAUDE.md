# Cafeteria Food Fight

**Git commits:** Do not add Claude as a co-author. No `Co-Authored-By` trailers.

**Keep this file up to date.** When adding new systems, files, plugins, constants, or architectural patterns, update the relevant sections here before finishing the work.

**Update README.md when finishing any task.** Keep the gameplay description and project status checklist current — add features to the bullet list, tick completed items, and remove anything that's no longer accurate.

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
├── score.rs             # CumulativeScores resource — damage_dealt + detention_slips per player, persists across rounds
├── input.rs             # ControllerInput, ControllerRegistry, ControllerId, InputPlugin (gilrs path)
├── steam.rs             # SteamInputPlugin — Steam Input action sets, controller polling (feature-gated)
├── controller.rs        # Gamepad utilities (read_left/right_stick), debug logging, disconnect/reconnect
├── sprites.rs           # SpritePlugin, SpriteAssets, AnimationState, frame-based animation
├── audio.rs             # AudioPlugin, SoundEvent (event-driven SFX), SoundAssets resource
├── lobby/
│   ├── mod.rs           # Lobby resource + PlayerSlot, join/leave/ready systems, player spawning
│   └── ui.rs            # Lobby screen UI (2x2 slot grid)
├── player/
│   ├── mod.rs           # PlayerPlugin registration
│   ├── components.rs    # Player, Health, Velocity, ControllerLink
│   ├── input.rs         # ControllerInput.move_stick → Velocity
│   ├── movement.rs      # Velocity → transform, bounds clamping
│   └── animation.rs     # Placeholder scale-pulse (no sprite sheets yet)
├── food/
│   ├── mod.rs           # FoodPlugin registration
│   ├── components.rs    # FoodType, FoodItem, Throwable, InFlight, ArcFlight, BounceFlight, Inventory, MeleeWeaponType, etc.
│   ├── spawning.rs      # Food + melee weapon spawn points, respawn timers, initial spawn
│   ├── throwing.rs      # Unified pickup (South = food or launcher with drop-replace) and throw (fire)
│   ├── launcher.rs      # Launcher types, fire, catapult charge (pickup handled by throwing.rs)
│   ├── melee.rs         # Melee pickup (West), LunchTray block/parry, Baguette swing
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
    └── scoreboard.rs    # Win check, round-over screen (cumulative scores, Play Again / Main Menu)
```

## Game State Flow

```
MainMenu → Lobby → Playing ⇄ Paused
                  Playing → RoundOver → Playing   (Play Again — scores persist, lobby kept)
                            RoundOver → MainMenu  (Main Menu — scores reset, lobby cleared)
```

States defined in `states.rs`: `MainMenu` (default), `Lobby`, `MapSelect` (stub), `Loading` (stub), `Playing`, `Paused`, `RoundOver`.

The `Gameplay` marker component is added to all in-game entities and used for bulk cleanup on return to MainMenu.

`GameSessionActive` is a marker resource inserted once on the first `OnEnter(Playing)` transition. It prevents spawn/setup systems from re-running when unpausing.

## Plugin Registration Order (main.rs)

1. `DefaultPlugins` (with `ImagePlugin::default_nearest()` for pixel art)
2. `SpritePlugin` — loads sprite assets (must precede gameplay plugins)
3. `AudioPlugin` — registers `SoundEvent`, loads `SoundAssets`, plays sounds on `Update`
4. `SteamInputPlugin` (feature-gated) — must precede `InputPlugin`; populates `ControllerRegistry` via Steam Input
5. `InputPlugin` — registers `ControllerRegistry`, runs gilrs population system in `PreUpdate`
6. `ControllerPlugin` — gamepad debug logging, disconnect → auto-pause, reconnect logging
7. `LobbyPlugin` — lobby UI, join/leave/ready, spawns players on `OnEnter(Playing)`
8. `PlayerPlugin` — input→velocity, movement, animation (FixedUpdate, gated on Playing)
9. `FoodPlugin` — food spawns, throwing, launchers, trajectories (FixedUpdate, gated on Playing)
10. `CombatPlugin` — food-player collision (FixedUpdate, gated on Playing)
11. `NpcPlugin` — NPC spawning, detection, patrol, chase, catch (FixedUpdate, gated on Playing)
12. `MapPlugin` — cafeteria spawn, wall collision (FixedUpdate, gated on Playing)
13. `UiPlugin` — main menu, HUD, pause, round over, cleanup

## Input Architecture

All input flows through a backend-agnostic abstraction in `src/input.rs`.

### ControllerInput (component on player entities)

```rust
pub struct ControllerInput {
    pub move_stick: Vec2,      // left stick, 0.15 deadzone applied
    pub aim_stick: Vec2,       // right stick, 0.15 deadzone applied
    pub pickup_food: ButtonState,    // South: pick up food OR launcher (unified); drops current item
    pub pickup_launcher: ButtonState, // West: pick up melee weapon; drops current melee weapon
    pub fire: ButtonState,     // RT: throw food / fire launcher
    pub melee: ButtonState,    // R1: baguette swing / lunch tray block+parry
    pub pause: ButtonState,    // Start: pause/unpause, ready-up, menu select
    pub join: ButtonState,     // South: lobby join / menu confirm
    pub leave: ButtonState,    // East: lobby leave / menu back
    pub exit_game: ButtonState, // Select: quit app
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
| South (A/X/B) | `South` | `pickup_food` / `join` — picks up food OR launcher, drops current |
| West (X/□/Y) | `West` | `pickup_launcher` — picks up melee weapon, drops current |
| RT | `RightTrigger2` | `fire` — throw food / fire launcher |
| R1 | `RightTrigger` | `melee` — baguette swing / lunch tray block+parry |
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

8 food types with distinct stats (damage, speed, trajectory kind). 6 launcher types with cooldowns, multipliers, and limited uses. Three trajectory systems: straight, arc (simulated Z with gravity), bounce (wall reflection). Food spawns at fixed points with 5-second respawn timers.

### Launcher Spawning

Single spawn point at map center (0, 0). One launcher spawns immediately on game start. After pickup (or uses exhausted), a new random launcher respawns after **20 seconds**. Implemented via `LauncherSpawnPoint` component (mirrors `FoodSpawnPoint` pattern): `active=true` while pickup present, `reset_launcher_spawn_point_system` detects pickup gone → starts timer, `launcher_respawn_system` fires when timer finishes.

### Elimination & Detention

When a player's health hits zero they are eliminated:

1. `Eliminated` marker component added; `EquippedLauncher`, `ChargingShot`, and held food removed.
2. `detention_system` (runs after `movement_system`) snaps them to their corner table and zeroes velocity every tick.
3. Eliminated players are excluded from food/launcher pickup & throw, NPC detection/chase/catch, and projectile collision.
4. Win condition: ≤1 non-eliminated player remaining (among ≥2 total) → `RoundOver`. Surviving player wins.

**Corner assignment** (`DETENTION_CORNERS` in `player/components.rs`, indexed by `player.id - 1`):

| Player | Corner | Position |
|--------|--------|----------|
| 1 | Bottom-left | (-400, -260) |
| 2 | Bottom-right | (400, -260) |
| 3 | Top-left | (-400, 260) |
| 4 | Top-right | (400, 260) |

Corner tables are visual-only sprites (no `Wall` component) in `map/loading.rs`.

### Cumulative Scoring

`CumulativeScores` resource (`src/score.rs`) persists across rounds until the session returns to MainMenu.

- **`damage_dealt: f32`** — total damage dealt to other players, credited in `food_player_collision_system` as `flight.damage.min(health.0)` (never overcounts past remaining HP).
- **`detention_slips: u32`** — number of players this player personally eliminated (sent to lunch detention). Displayed in the round-over scoreboard as "Detention Slips."

Indexed by `(player.id - 1)` (0–3). Updated in `combat/collision.rs`. Reset via `reset_scores` on `OnEnter(MainMenu)`.

**Round-over screen** (`ui/scoreboard.rs`):
- Shows round winner title + full cumulative table (Player | Damage | Detention Slips).
- START = Play Again: despawns all `Gameplay` entities + HUD, removes `GameSessionActive`, transitions to `Playing` (spawn systems run again, scores carry over).
- EAST (B) / Escape = Main Menu: transitions to `MainMenu`, triggering full cleanup + score reset.

### NPC State Machine

`NpcState` enum: `Patrolling` → `Suspicious` → `Chasing` → `Returning` → `Patrolling`. Detection uses cone check (angle + distance). NPCs change sprite color based on state (yellow=suspicious, red=chasing). Three NPCs: Teacher (medium speed, patrols tables), Principal (slow, wide detection), Lunch Lady (stationary at counter). Janitor is a planned fourth role (not yet spawned).

**Launcher alert rule:** `launcher_alert_system` (runs before `detection_system`) immediately forces **all NPCs** (Teacher, Principal, Lunch Lady) into `Chasing` toward the nearest player holding an `EquippedLauncher` — bypasses cone and distance checks entirely. Each NPC resumes normal detection once no player holds a launcher. Lunch Lady has `move_speed: 70.0` so she can chase when triggered (she is otherwise stationary at her patrol waypoint).

### Melee Weapon System

Two melee weapon types in `food/melee.rs`, picked up with West (`pickup_launcher` button):

**LunchTray** (defensive only):
- R1 `just_pressed` → adds `ParryWindow { timer: 0.2s }`. Food hitting the front arc (move-facing direction) during this window is deflected back at the thrower (`InFlight` spawned with reversed direction, `thrown_by = tray holder`).
- R1 held past 0.2s → `ParryWindow` removed, `Blocking` added. Food in front arc is despawned (no damage).
- R1 released → both `ParryWindow` and `Blocking` removed.
- While `Blocking`: move speed drops to 25% (checked in `player/input.rs`).
- "In front" = `dot(flight.direction, move_stick_normalized) < -0.4`.

**Baguette** (offensive):
- R1 `just_pressed` → melee swing. Deals 20 damage to players within 50px in a forward 120° arc (`dot(to_target, move_facing) > 0.5`). 0.7s swing cooldown, 15 uses.

**Spawn points:** Two fixed locations at `(-280, 50)` and `(280, -50)`, 15s respawn, random LunchTray or Baguette each spawn. Systems: `setup_melee_spawns`, `melee_respawn_system`, `reset_melee_spawn_point_system` in `spawning.rs`.

**Components:** `MeleeWeaponType`, `MeleeWeaponPickup`, `MeleeWeaponSpawnPoint`, `EquippedMeleeWeapon { weapon_type, swing_cooldown, uses_remaining }`, `ParryWindow { timer }`, `Blocking`, `MeleeVisual { player_entity }`.

**Weapon visual overlay** (`sprites.rs`):
- `MeleeVisual` is a separate sprite entity (z=1.2) that follows the player at their exact position.
- Spawned by `spawn_melee_visuals` on `Added<EquippedMeleeWeapon>`, despawned by `despawn_melee_visuals` on `RemovedComponents<EquippedMeleeWeapon>`.
- `sync_melee_visual_position` copies player transform each FixedUpdate tick.
- `update_melee_animation` drives frame selection based on `ParryWindow`/`Blocking`/`ControllerInput::melee`:
  - LunchTray: parry → col 3 (flash, looping); blocking → cols 4-5 loop 4fps; idle → col 2
  - Baguette: just_pressed + cooldown ready → cols 3-5 one-shot 15fps; finished/idle → col 2

### Audio

Sound effects use a simple event-driven pattern in `src/audio.rs`:

1. Gameplay systems send `SoundEvent` variants (e.g., `sound.send(SoundEvent::FoodHit)`)
2. `play_sounds` system (Update) reads the events and spawns `(AudioPlayer(handle), PlaybackSettings::DESPAWN)` — entities auto-despawn when playback finishes
3. Round-over fanfare is triggered directly via `OnEnter(GameState::RoundOver)` — no event needed

**SoundEvent variants and where they're sent:**

| Variant | Sent from |
|---|---|
| `FoodPickup` | `food/throwing.rs` → `pickup_system` |
| `FoodThrow` | `food/throwing.rs` → `throw_system` |
| `LauncherPickup` | `food/launcher.rs` → `launcher_pickup_system` |
| `LauncherFire` | `food/launcher.rs` → `launcher_fire_system`, `catapult_charge_system` |
| `FoodHit` | `combat/collision.rs` → `food_player_collision_system` |
| `PlayerCaught` | `npc/chase.rs` → `catch_system` |

Sound files live in `assets/sounds/*.ogg`. Sources and original filenames documented in `ATTRIBUTIONS.md`. Bevy logs an asset error (no crash) if a file is missing — audio for that event is silently skipped.

### Map Layout

Procedural cafeteria: 960x640 play area centered at origin. Bounds: ±480 x, ±320 y. Perimeter walls (16px thick), 6 tables in 2x3 grid, lunch counter at top, 2 pillars, 2 door markers. Everything uses the `Wall { half_size }` component for AABB collision.

## Assets

All gameplay entities use sprite atlases loaded from `assets/sprites/`. The map still uses procedural colored rectangles (no tilemap).

### Sprite Atlases

- **Players** (`sprites/players/player_{blue,red,green,yellow}.png`) — 32x32, 8×4 grid
  - Rows: walk_down (0), walk_up (1), walk_left (2), walk_right (3)
  - Cols 0–3: walk frames (8fps), 4–5: idle (3fps), 6–7: stunned (4fps)
  - Row 3 cols 4–5: holding_food idle (3fps)
  - Helper: `player_atlas_index(row, col)` (8 columns)

- **Teacher** (`sprites/npcs/teacher.png`) — 32x32, 7×4 grid
- **Principal** (`sprites/npcs/principal.png`) — 32x32, 7×4 grid
  - Both share `npc_standard_animation_for()`: rows 0–2 = patrol/suspicious/chase, 4 directions per row; row 3 = returning
  - Helper: `atlas_index(row, col, 7)`

- **Lunch Lady** (`sprites/npcs/lunch_lady.png`) — 32x32, 6×3 grid
  - Row 0: idle_stir (2fps), Row 1: suspicious_stir (2fps), Row 2: swing_left / swing_right (6fps)

- **Food** (`sprites/food/food_items.png`) — 16x16, 8×8 grid
  - One row per food type (see `food_type_row()`): Pizza=0, Meatball=1, Jello=2, Grape=3, MilkCarton=4, Spaghetti=5, BananaPeel=6, MysteryMeat=7
  - Helper: `food_atlas_index(row, col)` (8 columns)

- **Launchers** (`sprites/launchers/launchers.png`) — 32x32, 5×6 grid
  - One row per launcher type (see `launcher_type_row()`): Slingshot=0, KetchupGun=1, SporkLauncher=2, LunchTrayCatapult=3, StrawBlowgun=4, WatermelonCatapult=5
  - Helper: `launcher_atlas_index(row, col)` (5 columns)

- **Effects** (`sprites/effects/effects.png`) — 32x32, 6×3 grid
  - Row 1: splat decals by food type — col 0=red, 1=green, 2=purple, 3=white, 4=yellow, 5=brown
  - `food_splat_index(food_type)` maps food type → correct splat column
  - Helper: `effects_atlas_index(row, col)` (6 columns)

- **Melee weapons** (`sprites/melee/melee_weapons.png`) — 32x32, 6×2 grid
  - Columns: `ground_idle` (0), `ground_sparkle` (1), `equipped` (2), `active_1` (3), `active_2` (4), `active_3` (5)
  - Row 0: LunchTray — col 3 = parry flash (bright white-gold, 2px glow), col 4 = block pulse A (gold), col 5 = block pulse B (dim gold)
  - Row 1: Baguette — col 3 = swing windup (steep upward angle), col 4 = swing peak (diagonal + motion trail), col 5 = swing recovery (downward tilt + fading trail)
  - Animation sequences: ground (0-1 loop 3fps), equipped (2 static), parry (3 flash), blocking (4-5 loop 4fps), baguette_swing (3-5 one-shot 15fps)
  - Helpers: `melee_atlas_index(row, col)` (6 columns), `melee_weapon_type_row(weapon_type)`
  - Metadata: `assets/sprites/melee/melee_weapons.json`

### Sprite Generation

All sprite atlases are generated by Python/Pillow scripts run inside Docker. **Never generate sprites by hand or with a different tool.**

**Workflow:**

```bash
# Run a generation script (from project root):
./sprites/run.sh <script_name.py>

# Examples:
./sprites/run.sh generate_melee.py
./sprites/run.sh generate_launchers.py
./sprites/run.sh generate_cafeteria.py
```

`sprites/run.sh` mounts `sprites/` as `/scripts` and `assets/` as `/assets` inside a `python:3.12-slim` container with Pillow installed. Scripts write their output directly to `/assets/sprites/<category>/`.

**Generation scripts live in `sprites/` (project root), NOT in `assets/sprites/`.** Output PNGs go to `assets/sprites/<category>/`.

**Conventions for new sprite scripts:**

1. Use a named color palette at the top (named constants like `TR_RIM`, `BA_CRUST`) — never use raw tuples inline.
2. Draw pixel-by-pixel with a `px(img, x, y, color)` helper that bounds-checks.
3. Use a `ROW_GENERATORS = [(row_index, draw_fn), ...]` registry so rows can be added without rewriting `main()`.
4. For atlases that grow over time (like launchers), make the script **additive**: open the existing PNG, extend canvas if needed, draw only the new rows, save. For new atlases, create fresh.
5. Each script must include a docstring describing the atlas layout (columns, rows, cell size, what each column/row means).
6. After generating a PNG, add or update a matching `<name>.json` in the same `assets/sprites/<category>/` directory documenting tile_size, columns, rows, frame_labels, and per-type stats.
7. After generating sprites and updating Rust code, verify with `cargo build`.

**Existing generation scripts:**

| Script | Output | Atlas |
|---|---|---|
| `sprites/generate_melee.py` | `assets/sprites/melee/melee_weapons.png` | 4×2, 32×32 |
| `sprites/generate_launchers.py` | `assets/sprites/launchers/launchers.png` | 5×6, 32×32 (additive) |
| `sprites/generate_cafeteria.py` | `assets/sprites/map/cafeteria_bg.png` | 960×640 full-scene RGBA |

### Map
Single 960×640 background image (`assets/sprites/map/cafeteria_bg.png`) drawn by `sprites/generate_cafeteria.py`. Contains all visuals: 32px linoleum floor tiles, perimeter walls, cafeteria tables, lunch counter, pillars, detention corner tables, door markers, and melee spawn X marks. Wall/table/pillar/counter entities are collision-only (`Wall` component + `Transform`, no `Sprite`). The background is loaded via `SpriteAssets::cafeteria_bg` and spawned at z=-1.0.

## Constants & Play Area

- Play area: 960x640 (±480x, ±320y from center)
- Player size: 32x32 (half_size=16 for collision)
- Player speed: 200 px/s
- Wall thickness: 16px
- Pickup range: 70px
- Food spawn respawn: 5 seconds
- Launcher spawn: 1 point at center (0,0), 20-second respawn
- Melee spawn: 2 points at (-280, 50) and (280, -50), 15-second respawn
- Baguette swing range: 50px, 120° arc, 20 damage, 0.7s cooldown, 15 uses
- Lunch tray parry window: 0.2s; blocking speed penalty: 25% of normal
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
- [x] Multi-round sessions: Play Again from round-over screen, cumulative damage score + detention slip counter
- [x] Input abstraction: ControllerInput/ControllerRegistry, gilrs + Steam Input backends
- [x] Steam Deck: Gaming Mode launch via steam-launch.sh, steamdeck.sh build helper
- [x] Sprite art: players, NPCs (Teacher/Principal/LunchLady), food, launchers, effects — all atlased
- [x] Audio: `src/audio.rs` — SoundEvent system, 7 OGG slots in `assets/sounds/` (see ATTRIBUTIONS.md)
- [ ] Additional maps: only cafeteria
- [ ] Polish: no particles or screen shake

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

## Reference Docs

- `ATTRIBUTIONS.md` — sound effect sources and original filenames (all CC0 from Kenney.nl)
- `MACBOOK.md` — guide for Bluetooth controller support on macOS
- `STEAMDECK.md` — Steam Deck deployment guide
- `steamdeck.sh` — automated setup/build/run script for Steam Deck desktop mode
- `steam_input_manifest.vdf` — IGA action set definitions for Steam Input
