use bevy::prelude::*;

#[derive(Clone, Debug)]
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

#[derive(Component)]
pub struct FoodItem {
    pub food_type: FoodType,
    pub damage: f32,
}

#[derive(Component)]
pub struct Throwable;

#[derive(Component)]
pub struct InFlight {
    pub thrown_by: Entity,
    pub trajectory: Trajectory,
}

#[derive(Clone)]
pub enum Trajectory {
    Straight { speed: f32 },
    Arc { speed: f32, gravity: f32 },
    Bounce { speed: f32, bounces_remaining: u8 },
}

#[derive(Component)]
pub struct Inventory {
    pub held_food: Option<FoodType>,
}
