# Cafeteria Food Fight — Implementation Plan

## Overview

A 2D top-down player-vs-player game where students battle with food in school environments. Players throw food by hand or use launchers, while avoiding teachers and other authority figures who patrol the maps. Built in Rust with the Bevy game engine.

## Tech Stack

- **Language:** Rust
- **Engine:** Bevy (latest stable, currently 0.15+)
- **Physics:** Custom AABB collision (no physics engine needed initially)
- **Tilemaps:** `bevy_ecs_tilemap` with Tiled editor (`.tmx`/`.json`) map files
- **Audio:** Bevy built-in `bevy_audio`
- **Input:** Keyboard + mouse (twin-stick style: WASD move, mouse aim/shoot)

## Project Structure

```
food_fight/
├── Cargo.toml
├── assets/
│   ├── sprites/
│   │   ├── players/          # Player character sprite sheets
│   │   ├── food/             # Food item sprites
│   │   ├── npcs/             # Teacher, principal, lunch lady, janitor
│   │   ├── effects/          # Splat animations, particles
│   │   └── ui/               # HUD elements, menus
│   ├── maps/
│   │   ├── cafeteria.tmx     # Tiled map files
│   │   ├── playground.tmx
│   │   ├── gym.tmx
│   │   └── tilesets/         # Tileset images
│   └── audio/
│       ├── sfx/              # Splat, throw, caught, pickup sounds
│       └── music/            # Background music per map
├── src/
│   ├── main.rs               # App entry point, plugin registration
│   ├── states.rs             # Game state machine
│   ├── player/
│   │   ├── mod.rs
│   │   ├── components.rs     # Player, Health, Score, Inventory
│   │   ├── input.rs          # Input reading system
│   │   ├── movement.rs       # Player movement system
│   │   └── animation.rs      # Sprite animation system
│   ├── food/
│   │   ├── mod.rs
│   │   ├── components.rs     # FoodItem, Throwable, InFlight, FoodType
│   │   ├── spawning.rs       # Food spawn points, respawn timers
│   │   ├── throwing.rs       # Hand-throw system
│   │   ├── launcher.rs       # Launcher/gun weapon system
│   │   └── trajectory.rs     # Projectile movement (straight, arc, bounce, stream)
│   ├── combat/
│   │   ├── mod.rs
│   │   ├── collision.rs      # AABB collision detection
│   │   ├── damage.rs         # Hit registration, damage application
│   │   └── effects.rs        # Splat effects, screen shake, knockback
│   ├── npc/
│   │   ├── mod.rs
│   │   ├── components.rs     # NpcAuthority, PatrolPath, DetectionCone, NpcState
│   │   ├── patrol.rs         # Waypoint following system
│   │   ├── detection.rs      # Vision/detection system
│   │   ├── chase.rs          # Chase behavior system
│   │   └── roles.rs          # Role-specific behavior (teacher, principal, etc.)
│   ├── map/
│   │   ├── mod.rs
│   │   ├── loading.rs        # Tilemap loading from Tiled files
│   │   ├── collision.rs      # Wall/obstacle collision for entities
│   │   └── hazards.rs        # Map-specific interactive elements
│   └── ui/
│       ├── mod.rs
│       ├── hud.rs            # In-game HUD (health, score, ammo)
│       ├── menu.rs           # Main menu, map select, pause menu
│       └── scoreboard.rs     # End-of-round results
```

---

## Phases

### Phase 1: Core Movement & Rendering

**Goal:** One player character moving around a bounded area with placeholder art.

**Components:**
```rust
#[derive(Component)]
struct Player {
    id: u8,
    speed: f32,
}

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Velocity(Vec2);
```

**Systems:**
- `input_system` — reads WASD, sets player `Velocity` based on input direction
- `movement_system` — applies `Velocity` to `Transform` each frame, respects bounds
- `camera_system` — camera follows the play area (fixed for now)
- `animation_system` — cycles sprite sheet frames based on movement direction

**Tasks:**
1. Scaffold the Bevy app with a window, camera, and background color
2. Load a placeholder sprite (colored rectangle or free sprite) and spawn a player entity
3. Implement WASD movement with velocity and delta time
4. Add screen bounds clamping so the player can't leave the area
5. Add sprite sheet animation (idle, walk in 4 directions)
6. Add a second player on arrow keys (or gamepad) to confirm multi-entity input works

**Acceptance:** Two players move independently on screen with animated sprites.

---

### Phase 2: Food Throwing (Hand Throw)

**Goal:** Players can pick up food items and throw them at each other.

**Components:**
```rust
#[derive(Component)]
struct FoodItem {
    food_type: FoodType,
    damage: f32,
}

#[derive(Clone)]
enum FoodType {
    Pizza,
    Meatball,
    Jello,
    Grape,
    MilkCarton,
    Spaghetti,
    BananaPeel,
    MysteryMeat,
}

#[derive(Component)]
struct Throwable;              // Can be picked up

#[derive(Component)]
struct InFlight {
    thrown_by: Entity,         // Don't hit the thrower
    trajectory: Trajectory,
}

#[derive(Component)]
struct Pickupable;             // In range to grab

#[derive(Clone)]
enum Trajectory {
    Straight { speed: f32 },
    Arc { speed: f32, gravity: f32 },
    Bounce { speed: f32, bounces_remaining: u8 },
}

#[derive(Component)]
struct Inventory {
    held_food: Option<FoodType>,
}
```

**Systems:**
- `food_spawn_system` — places food items at designated spawn points, respawns after a timer
- `pickup_system` — when player overlaps a `Throwable` food and presses pickup key, add to inventory
- `aim_system` — tracks mouse position relative to player, computes aim direction
- `throw_system` — on click/key, spawns an `InFlight` projectile from the player's position toward aim direction
- `trajectory_system` — updates `InFlight` entities each frame based on `Trajectory` variant
- `food_collision_system` — checks `InFlight` food against player hitboxes (AABB)
- `food_hit_system` — on collision: apply damage, despawn food, spawn splat effect

**Tasks:**
1. Define food types with stats (damage, speed, trajectory type) in a config/data file or const table
2. Spawn food items at hardcoded positions with `Throwable` marker
3. Implement pickup: proximity check + key press → remove `Throwable`, set `Inventory.held_food`
4. Implement aiming: compute angle from player to mouse cursor
5. Implement throw: spawn projectile entity at player pos, give it `Velocity` toward aim, add `InFlight`
6. Implement `Trajectory::Straight` movement (velocity applied each frame, despawn after max range or off screen)
7. Implement AABB collision between `InFlight` entities and `Player` entities (skip thrower)
8. On hit: decrement `Health`, despawn projectile, spawn a short-lived splat sprite
9. Add `Trajectory::Arc` (apply gravity each frame) and `Trajectory::Bounce` (reflect velocity on wall hit)

**Acceptance:** Players pick up food, throw it at each other, hits register and reduce health, different foods behave differently.

---

### Phase 3: Launcher / Food Gun

**Goal:** Players can find and equip a launcher weapon that changes how food is fired.

**Components:**
```rust
#[derive(Component)]
struct Launcher {
    launcher_type: LauncherType,
    cooldown_timer: Timer,
    ammo: Option<FoodType>,     // Loaded food
}

enum LauncherType {
    Slingshot,        // Standard aimed shot, adds range + speed
    KetchupGun,      // Stream, short range, rapid fire, low per-hit damage
    SporkLauncher,    // Fast straight projectile, high accuracy
    LunchTrayCatapult,// Charge-up, arc shot, area damage on impact
    StrawBlowgun,     // Rapid-fire peas/spitballs, very fast, very low damage
}

#[derive(Component)]
struct ChargingShot {
    charge_time: f32,
    max_charge: f32,
}

#[derive(Component)]
struct AreaDamage {
    radius: f32,
}
```

**Systems:**
- `launcher_pickup_system` — launchers spawn on the map, picked up like food
- `launcher_fire_system` — override throw behavior when launcher equipped; fire with launcher-specific properties
- `charge_system` — for `LunchTrayCatapult`: hold button to charge, release to fire with charge-scaled power
- `stream_system` — for `KetchupGun`: continuous stream of small projectiles while button held
- `area_damage_system` — on impact, damage all players within radius

**Tasks:**
1. Spawn launcher pickups on the map with distinct sprites
2. Modify inventory to hold either food or a launcher (or both: launcher + food ammo)
3. Implement `Slingshot` — same as hand throw but with boosted speed/range
4. Implement `SporkLauncher` — fires built-in spork ammo (no food needed), fast cooldown
5. Implement `KetchupGun` — rapid fire stream, drains over time, short range
6. Implement `LunchTrayCatapult` — hold-to-charge mechanic, arc trajectory, `AreaDamage` on impact
7. Implement `StrawBlowgun` — rapid pea projectiles, minimal damage, fast fire rate
8. Add cooldown timers between shots per launcher type
9. Add ammo/durability system: launchers break after N uses or run out of ammo

**Acceptance:** Multiple launcher types with distinct fire behaviors. Charge-up and stream mechanics work. Launchers feel meaningfully different from hand-throwing.

---

### Phase 4: Tilemap & First Map (Cafeteria)

**Goal:** Replace the empty bounded area with a real tile-based cafeteria map with walls and obstacles.

**Systems:**
- `map_load_system` — loads `.tmx` file via `bevy_ecs_tilemap`, sets up tile layers
- `wall_collision_system` — prevents entities from walking through wall/obstacle tiles
- `projectile_wall_system` — food projectiles collide with walls (despawn or bounce depending on type)

**Tasks:**
1. Install and configure `bevy_ecs_tilemap`
2. Design the cafeteria map in Tiled editor:
   - Floor tiles
   - Walls around the perimeter
   - Tables (act as cover — block movement and projectiles)
   - Lunch line counter (food resupply zone)
   - Doors/entry points (spawn locations)
3. Create a collision layer in Tiled (mark which tiles are solid)
4. Load the map and render it in Bevy
5. Implement tile-based collision: check player movement against collision layer
6. Implement projectile-vs-wall collision: `InFlight` entities check ahead and despawn/bounce on wall hit
7. Place food spawn points on the map (on tables, in the lunch line)
8. Place launcher spawn points
9. Place player spawn points

**Acceptance:** Cafeteria renders, players navigate around tables, projectiles hit walls, food spawns on tables.

---

### Phase 5: NPCs — Teachers & Authority Figures

**Goal:** AI-controlled NPCs patrol the map and chase players who are caught misbehaving.

**Components:**
```rust
#[derive(Component)]
struct NpcAuthority {
    role: NpcRole,
    detection_radius: f32,
    detection_angle: f32,    // FOV in radians (e.g., PI/3 for 60 degrees)
    move_speed: f32,
    catch_radius: f32,
}

enum NpcRole {
    Teacher,
    Principal,
    LunchLady,
    Janitor,
}

#[derive(Component)]
enum NpcState {
    Patrolling { path_index: usize },
    Suspicious { last_seen: Vec2, investigate_timer: Timer },
    Chasing { target: Entity },
    Returning { path_index: usize },
}

#[derive(Component)]
struct PatrolPath {
    waypoints: Vec<Vec2>,
}

#[derive(Component)]
struct Suspicious;  // Marker on players doing suspicious things (throwing, holding launcher)

#[derive(Component)]
struct Caught {
    stun_timer: Timer,
    penalty_applied: bool,
}
```

**Systems:**
- `patrol_system` — NPCs follow their waypoint path, moving to next waypoint when close enough
- `detection_system` — each frame, check if any `Suspicious` player is within detection cone; raycast against walls for line-of-sight
- `npc_state_transition_system` — handles state changes:
  - `Patrolling` → `Suspicious` (saw something at edge of range)
  - `Suspicious` → `Chasing` (confirmed visual on player)
  - `Suspicious` → `Returning` (investigate timer expired, lost sight)
  - `Chasing` → `Returning` (lost sight for N seconds)
  - `Returning` → `Patrolling` (reached nearest waypoint)
- `chase_system` — NPC moves toward target player, navigating around obstacles
- `catch_system` — when chasing NPC gets within `catch_radius` of target: apply `Caught` to player
- `caught_penalty_system` — `Caught` players are stunned (can't move/throw), lose points or drop items
- `suspicion_system` — marks players as `Suspicious` when they: throw food, fire a launcher, are near a fresh splat, or are holding a weapon in NPC line of sight

**NPC Role Specifics:**
- **Teacher:** Medium speed, medium detection radius, patrols set route. Penalty: 3-second stun + drop held item.
- **Principal:** Slow but wide detection, patrols randomly or is attracted to areas with lots of splats. Penalty: 5-second stun + lose points.
- **Lunch Lady:** Stationary near lunch line. Detects only within a small radius. Penalty: banned from food resupply for 10 seconds. Optionally gives food to well-behaved players.
- **Janitor:** Follows splat marks, cleans them up (removes ground effects). Not a threat — but removes banana peel traps and ketchup slicks, indirectly affecting gameplay.

**Tasks:**
1. Create `NpcAuthority` component and spawn a single teacher with a hardcoded patrol path
2. Implement `patrol_system`: move NPC along waypoints, loop back to start
3. Implement `suspicion_system`: tag players as `Suspicious` when throwing or holding weapons
4. Implement `detection_system`: cone check (angle + distance) from NPC facing direction to player position
5. Implement line-of-sight: simple raycast/step along line checking collision tiles
6. Implement `npc_state_transition_system` with the full state machine
7. Implement `chase_system`: NPC moves toward target (simple direct movement; obstacle avoidance can be a later improvement)
8. Implement `catch_system` + `caught_penalty_system`: stun the player, show visual feedback
9. Add detection cone visual indicator (optional debug/gameplay aid — shows NPC vision area)
10. Add teacher NPC to cafeteria map with patrol path defined in Tiled (object layer)
11. Add principal with different behavior profile
12. Add lunch lady (stationary, zone-based detection)
13. Add janitor (follows splat marks, cleans ground effects)

**Acceptance:** NPCs patrol, detect misbehaving players, chase them, and apply penalties. Players must be strategic about when they throw. Different NPC roles behave distinctly.

---

### Phase 6: Game Flow & UI

**Goal:** Complete game loop from main menu through round play to results screen.

**Game States:**
```rust
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    MapSelect,
    Loading,
    Playing,
    Paused,
    RoundOver,
}
```

**Win Condition Options (configurable):**
- **Last Standing:** First player to reach 0 health loses
- **High Score:** Timed rounds, most hits scored wins
- **Lives:** Each player has N lives, last one with lives wins

**Systems:**
- `hud_system` — renders health bars, score, held item icon, ammo count
- `pause_system` — ESC toggles pause, freezes all gameplay systems
- `round_timer_system` — countdown for timed modes
- `win_check_system` — evaluates win condition each frame
- `scoreboard_system` — displays results at round end

**Tasks:**
1. Implement `GameState` enum and Bevy `States` integration
2. Build main menu screen: title, "Start Game", "Quit" buttons
3. Build map selection screen: show available maps, highlight selection
4. Implement loading state: load selected map + assets, transition to `Playing`
5. Build in-game HUD: health bar per player, score, currently held item, launcher ammo
6. Implement pause: ESC freezes gameplay systems, shows resume/quit overlay
7. Implement win condition check (start with "last standing" — simplest)
8. Build round-over screen: winner announcement, stats (hits landed, food thrown, times caught), play again / menu buttons
9. Wire up full flow: Menu → Map Select → Loading → Playing → RoundOver → back to Menu or replay

**Acceptance:** Full playable loop from menu to gameplay to results, with pause support.

---

### Phase 7: Additional Maps

**Goal:** Add playground, gym, and school bus maps with unique characteristics.

**Map Definitions:**

| Map | Layout | Unique Element | NPC Configuration |
|-----|--------|---------------|-------------------|
| Cafeteria | Indoor, tables as cover, lunch line | Food respawns fastest here | 1 teacher patrol, 1 lunch lady at counter |
| Playground | Open outdoor, monkey bars, sandbox | Monkey bars = elevated platform (can throw over NPCs), sandbox = slow zone | 1 teacher, wide patrol |
| Gym | Large indoor, bleachers on sides, basketball hoops | Bleachers = elevated cover, hoops = trick shot bonus points if food goes through | 1 teacher, 1 principal |
| School Bus | Narrow long corridor, seats as partial cover | Bus rocks/tilts, food slides toward back periodically | 1 bus driver NPC (doesn't leave seat, yells to stun nearby players) |

**Tasks:**
1. Design each map in Tiled with collision, spawn, and NPC path layers
2. Create tilesets for each environment (or placeholder colored tiles)
3. Add map-specific hazard components (slow zone, elevated platform, rocking bus)
4. Configure NPC patrol paths per map in the Tiled object layer
5. Implement map-specific systems where needed (bus rocking, trick shot detection)
6. Test each map for balance: sightlines, cover distribution, NPC coverage

**Acceptance:** All four maps playable with distinct feel and strategy.

---

### Phase 8: Polish & Juice

**Goal:** Make the game feel good to play.

**Tasks:**
1. **Screen shake** — on hit impact, brief camera shake (scale with damage)
2. **Splat particles** — food-colored particles burst on impact
3. **Splat decals** — food leaves marks on floor/walls that persist during the round
4. **Hit flash** — damaged player sprite flashes white briefly
5. **Throw animation** — wind-up and release frames
6. **Sound effects** — splat, throw whoosh, pickup, launcher fire, NPC whistle/yell, caught jingle
7. **Background music** — per-map ambient tracks (cafeteria chatter, playground ambiance, gym squeaks)
8. **Food-specific effects:**
   - Spaghetti leaves a trail
   - Jello wobbles in flight
   - Milk carton explodes into a splash
   - Banana peel spins when thrown, stays on ground as a slip trap
   - Mystery meat has random color/size each time
9. **NPC feedback** — teacher blows whistle when chasing, principal's footsteps get louder, exclamation mark appears over NPC head when they spot you
10. **UI polish** — animated health bars, score pop-ups (+100!), item pickup notifications

---

## Art Pipeline

**Placeholder Phase (Phases 1–3):**
- Colored rectangles for all entities
- Distinct colors per food type, per player, per NPC role
- Focus on mechanics, not visuals

**Kid Art Phase (Phase 4+):**
- Use Aseprite ($20) or Piskel (free, browser-based) for pixel art
- Target resolution: 32×32 per sprite, 16×16 for food items
- Each kid owns a set of assets to create (e.g., one does food, one does characters, one does maps)
- Character sprites need: idle (1 frame minimum), walk (4 frames × 4 directions), throw (3 frames)
- Food sprites need: item icon, in-flight sprite, splat sprite

**Free Asset Sources (for bases/reference):**
- itch.io/game-assets (search "food sprites", "school tileset", "character top-down")
- OpenGameArt.org
- Kenney.nl (excellent free tilesets and character bases)

## Audio Pipeline

- Freesound.org for placeholder SFX
- Kids can record their own (phone mic → Audacity → export `.ogg`)
- Background music: OpenGameArt.org or Kevin MacLeod (incompetech.com)

---

## Configuration & Data

Keep game balance data external or in a central constants module so the kids can tweak without touching game logic:

```rust
// src/data.rs or loaded from a RON/JSON file
pub struct FoodStats {
    pub food_type: FoodType,
    pub damage: f32,
    pub speed: f32,
    pub trajectory: Trajectory,
    pub sprite_path: String,
}

pub struct LauncherStats {
    pub launcher_type: LauncherType,
    pub cooldown_secs: f32,
    pub speed_multiplier: f32,
    pub uses: u32,             // 0 = infinite
    pub sprite_path: String,
}

pub struct NpcStats {
    pub role: NpcRole,
    pub speed: f32,
    pub detection_radius: f32,
    pub detection_angle: f32,  // radians
    pub catch_radius: f32,
    pub penalty_stun_secs: f32,
    pub penalty_point_loss: i32,
}
```

Consider using Bevy's RON asset loader or a simple JSON file so values can be tweaked without recompilation.

---

## Dependencies (Cargo.toml)

```toml
[dependencies]
bevy = "0.15"
bevy_ecs_tilemap = "0.15"      # Match Bevy version
serde = { version = "1", features = ["derive"] }
serde_json = "1"                # For loading config/stats
rand = "0.8"                    # Random spawns, mystery meat

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

The `[profile]` settings enable Bevy's recommended "fast compiles" configuration.

---

## Implementation Notes for Claude Code

- **Start each phase by scaffolding the module structure** (create the files, define the components, register empty systems) before implementing logic.
- **Register all systems via plugins.** Each module (`player`, `food`, `npc`, etc.) should expose a `Plugin` that registers its systems, and `main.rs` adds all plugins.
- **Use Bevy's `States` for game flow.** Systems should be gated with `.run_if(in_state(GameState::Playing))` to prevent gameplay during menus/pause.
- **Use `FixedUpdate` for gameplay logic** (movement, physics, collision) and `Update` for rendering/UI.
- **Placeholder art first.** Use `bevy::sprite::Sprite` with solid colors or simple shapes. Don't block on art assets.
- **Test incrementally.** Each phase should be playable/demonstrable before moving to the next.
- **Keep systems small and focused.** One system per concern. This makes debugging with the kids much easier.
