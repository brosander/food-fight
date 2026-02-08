use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NpcRole {
    Teacher,
    Principal,
    LunchLady,
    Janitor,
}

#[derive(Component)]
pub struct NpcAuthority {
    pub role: NpcRole,
    pub detection_radius: f32,
    /// FOV in radians (e.g., PI/3 for 60 degrees each side of facing direction).
    pub detection_angle: f32,
    pub move_speed: f32,
    pub catch_radius: f32,
}

#[derive(Component)]
pub enum NpcState {
    Patrolling { waypoint_index: usize },
    Suspicious { last_seen: Vec2, timer: Timer },
    Chasing { target: Entity },
    Returning { waypoint_index: usize },
}

#[derive(Component)]
pub struct PatrolPath {
    pub waypoints: Vec<Vec2>,
}

/// Facing direction of the NPC (normalized).
#[derive(Component)]
pub struct Facing(pub Vec2);

/// Marker on players who are doing suspicious things.
#[derive(Component)]
pub struct Suspicious;

/// Applied to players caught by an NPC.
#[derive(Component)]
pub struct Caught {
    pub stun_timer: Timer,
}
