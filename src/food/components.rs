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
    pub color: Color,
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
    pub damage: f32,
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

/// Splat effect that fades out.
#[derive(Component)]
pub struct SplatEffect {
    pub lifetime: Timer,
}
