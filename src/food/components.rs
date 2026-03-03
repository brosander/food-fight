//! Food and launcher entity components.
//!
//! Data flow: `FoodType::stats()` → spawn with `FoodItem` + `Throwable` (on ground).
//! On pickup: moved into `Inventory::held_food`. On throw: spawned as `InFlight`
//! (straight/arc/bounce) with damage copied from `FoodStats`. `InFlight` is what
//! combat reads — `FoodItem::damage` is never queried post-spawn.

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FoodType {
    Pizza,
    Meatball,
    Jello,
    Grape,
    MilkCarton,
    Spaghetti,
    BananaPeel,
    MysteryMeat,
}

impl FoodType {
    pub fn stats(&self) -> FoodStats {
        match self {
            FoodType::Pizza => FoodStats {
                damage: 15.0,
                speed: 300.0,
                trajectory: TrajectoryKind::Straight,
                color: Color::srgb(0.9, 0.7, 0.1),
                size: Vec2::new(14.0, 14.0),
            },
            FoodType::Meatball => FoodStats {
                damage: 20.0,
                speed: 250.0,
                trajectory: TrajectoryKind::Arc { gravity: 200.0 },
                color: Color::srgb(0.5, 0.25, 0.15),
                size: Vec2::new(10.0, 10.0),
            },
            FoodType::Jello => FoodStats {
                damage: 8.0,
                speed: 350.0,
                trajectory: TrajectoryKind::Bounce {
                    bounces: 3,
                },
                color: Color::srgb(0.2, 0.9, 0.3),
                size: Vec2::new(12.0, 12.0),
            },
            FoodType::Grape => FoodStats {
                damage: 5.0,
                speed: 400.0,
                trajectory: TrajectoryKind::Straight,
                color: Color::srgb(0.5, 0.1, 0.6),
                size: Vec2::new(6.0, 6.0),
            },
            FoodType::MilkCarton => FoodStats {
                damage: 25.0,
                speed: 200.0,
                trajectory: TrajectoryKind::Arc { gravity: 300.0 },
                color: Color::srgb(0.95, 0.95, 0.95),
                size: Vec2::new(12.0, 16.0),
            },
            FoodType::Spaghetti => FoodStats {
                damage: 12.0,
                speed: 280.0,
                trajectory: TrajectoryKind::Straight,
                color: Color::srgb(0.9, 0.8, 0.3),
                size: Vec2::new(14.0, 8.0),
            },
            FoodType::BananaPeel => FoodStats {
                damage: 3.0,
                speed: 200.0,
                trajectory: TrajectoryKind::Arc { gravity: 400.0 },
                color: Color::srgb(0.95, 0.9, 0.2),
                size: Vec2::new(12.0, 8.0),
            },
            FoodType::MysteryMeat => FoodStats {
                damage: 30.0,
                speed: 220.0,
                trajectory: TrajectoryKind::Straight,
                color: Color::srgb(0.4, 0.4, 0.35),
                size: Vec2::new(16.0, 16.0),
            },
        }
    }

    /// All food types for random spawning.
    pub const ALL: &[FoodType] = &[
        FoodType::Pizza,
        FoodType::Meatball,
        FoodType::Jello,
        FoodType::Grape,
        FoodType::MilkCarton,
        FoodType::Spaghetti,
        FoodType::BananaPeel,
        FoodType::MysteryMeat,
    ];
}

pub struct FoodStats {
    pub damage: f32,
    pub speed: f32,
    pub trajectory: TrajectoryKind,
    #[allow(dead_code)]
    pub color: Color, // used to tint sprite; read at spawn time only
    pub size: Vec2,
}

#[derive(Clone, Debug)]
pub enum TrajectoryKind {
    Straight,
    Arc { gravity: f32 },
    Bounce { bounces: u8 },
}

/// A food item entity with type and damage info.
#[derive(Component)]
pub struct FoodItem {
    pub food_type: FoodType,
    #[allow(dead_code)]
    pub damage: f32, // copied into InFlight::damage at throw time; combat reads InFlight
}

/// Marker: this food is on the ground and can be picked up.
#[derive(Component)]
pub struct Throwable;

/// Marker: this food is flying through the air.
#[derive(Component)]
pub struct InFlight {
    pub thrown_by: Entity,
    pub direction: Vec2,
    pub speed: f32,
    pub damage: f32,
    pub max_range: f32,
    pub distance_traveled: f32,
}

/// For arc trajectory: vertical velocity component (simulated Z).
#[derive(Component)]
pub struct ArcFlight {
    pub vertical_velocity: f32,
    pub gravity: f32,
    pub simulated_z: f32,
}

/// For bounce trajectory: bounces remaining.
#[derive(Component)]
pub struct BounceFlight {
    pub bounces_remaining: u8,
}

/// Player's held food item.
#[derive(Component)]
pub struct Inventory {
    pub held_food: Option<FoodType>,
}

/// Food spawn point marker.
#[derive(Component)]
pub struct FoodSpawnPoint {
    pub respawn_timer: Timer,
    pub active: bool,
}

/// Launcher spawn point marker — single center-map point, 20s respawn.
#[derive(Component)]
pub struct LauncherSpawnPoint {
    pub respawn_timer: Timer,
    pub active: bool,
}

/// Splat effect that fades out.
#[derive(Component)]
pub struct SplatEffect {
    pub lifetime: Timer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeleeWeaponType {
    LunchTray,
    Baguette,
}

/// A melee weapon pickup on the ground.
#[derive(Component)]
pub struct MeleeWeaponPickup {
    pub weapon_type: MeleeWeaponType,
}

/// Spawn point for melee weapons; one per side of the map.
#[derive(Component)]
pub struct MeleeWeaponSpawnPoint {
    pub respawn_timer: Timer,
    pub active: bool,
}

/// A player holding a melee weapon.
#[derive(Component)]
pub struct EquippedMeleeWeapon {
    pub weapon_type: MeleeWeaponType,
    pub swing_cooldown: Timer,
    pub uses_remaining: u32,
    /// Set to true the frame a baguette swing fires; cleared by the animation system.
    pub swinging: bool,
    /// Direction the player was facing when the last baguette swing fired.
    pub swing_facing: Vec2,
}

/// Added on R1 just_pressed (0.2s window). Food hitting the front arc during this window is deflected.
#[derive(Component)]
pub struct ParryWindow {
    pub timer: Timer,
}

/// Added when R1 is held past the parry window. Food in the front arc is blocked (despawned, no damage).
#[derive(Component)]
pub struct Blocking;

/// Follows a player to display their equipped melee weapon sprite.
/// Spawned/despawned automatically when EquippedMeleeWeapon is added/removed.
#[derive(Component)]
pub struct MeleeVisual {
    pub player_entity: Entity,
}
