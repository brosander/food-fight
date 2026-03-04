use bevy::prelude::*;

use super::Wall;
use crate::sprites::SpriteAssets;
use crate::states::Gameplay;

/// Spawns the cafeteria map: one background sprite for visuals, invisible Wall
/// entities for collision. All decorative elements (tables, doors, markers) are
/// baked into the background image; only elements that need AABB collision are
/// spawned as separate entities.
pub fn spawn_cafeteria(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    // Background — full cafeteria art at z=-1.0 (behind all gameplay entities)
    commands.spawn((
        Sprite {
            image: sprite_assets.cafeteria_bg.clone(),
            custom_size: Some(Vec2::new(960.0, 640.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        Gameplay,
    ));

    // Perimeter walls (collision only — visual is in the background image)
    let bounds_x = 480.0;
    let bounds_y = 320.0;
    let thickness = 16.0;

    spawn_wall(
        &mut commands,
        Vec3::new(0.0, bounds_y - thickness / 2.0, 0.0),
        Vec2::new(bounds_x * 2.0, thickness),
    );
    spawn_wall(
        &mut commands,
        Vec3::new(0.0, -bounds_y + thickness / 2.0, 0.0),
        Vec2::new(bounds_x * 2.0, thickness),
    );
    spawn_wall(
        &mut commands,
        Vec3::new(-bounds_x + thickness / 2.0, 0.0, 0.0),
        Vec2::new(thickness, bounds_y * 2.0),
    );
    spawn_wall(
        &mut commands,
        Vec3::new(bounds_x - thickness / 2.0, 0.0, 0.0),
        Vec2::new(thickness, bounds_y * 2.0),
    );

    // Cafeteria tables
    for row in [-120.0_f32, 120.0] {
        for col in [-250.0_f32, 0.0, 250.0] {
            spawn_wall(
                &mut commands,
                Vec3::new(col, row, 0.0),
                Vec2::new(120.0, 24.0),
            );
        }
    }

    // Lunch counter
    spawn_wall(
        &mut commands,
        Vec3::new(0.0, 260.0, 0.0),
        Vec2::new(400.0, 20.0),
    );

    // Pillars
    spawn_wall(
        &mut commands,
        Vec3::new(-380.0, 0.0, 0.0),
        Vec2::new(24.0, 80.0),
    );
    spawn_wall(
        &mut commands,
        Vec3::new(380.0, 0.0, 0.0),
        Vec2::new(24.0, 80.0),
    );
}

/// Spawns a collision-only wall entity (no visual — baked into cafeteria_bg).
fn spawn_wall(commands: &mut Commands, position: Vec3, size: Vec2) {
    commands.spawn((
        Transform::from_translation(position),
        Wall {
            half_size: size / 2.0,
        },
        Gameplay,
    ));
}
