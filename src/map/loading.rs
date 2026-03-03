use bevy::prelude::*;

use super::Wall;
use crate::states::Gameplay;

/// Spawns the cafeteria map layout using simple sprite entities.
/// This is a procedural placeholder — will be replaced with Tiled map loading later.
pub fn spawn_cafeteria(mut commands: Commands) {
    let wall_color = Color::srgb(0.35, 0.3, 0.25);
    let table_color = Color::srgb(0.55, 0.4, 0.25);
    let counter_color = Color::srgb(0.6, 0.55, 0.5);
    let floor_color = Color::srgb(0.25, 0.22, 0.2);

    // Floor
    commands.spawn((
        Sprite {
            color: floor_color,
            custom_size: Some(Vec2::new(960.0, 640.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        Gameplay,
    ));

    // Perimeter walls
    let bounds_x = 480.0;
    let bounds_y = 320.0;
    let thickness = 16.0;

    // Top wall
    spawn_wall(
        &mut commands,
        Vec3::new(0.0, bounds_y - thickness / 2.0, 0.0),
        Vec2::new(bounds_x * 2.0, thickness),
        wall_color,
    );
    // Bottom wall
    spawn_wall(
        &mut commands,
        Vec3::new(0.0, -bounds_y + thickness / 2.0, 0.0),
        Vec2::new(bounds_x * 2.0, thickness),
        wall_color,
    );
    // Left wall
    spawn_wall(
        &mut commands,
        Vec3::new(-bounds_x + thickness / 2.0, 0.0, 0.0),
        Vec2::new(thickness, bounds_y * 2.0),
        wall_color,
    );
    // Right wall
    spawn_wall(
        &mut commands,
        Vec3::new(bounds_x - thickness / 2.0, 0.0, 0.0),
        Vec2::new(thickness, bounds_y * 2.0),
        wall_color,
    );

    // === Interior: Cafeteria tables (horizontal) ===
    // Two rows of 3 tables
    for row in [-120.0_f32, 120.0] {
        for col in [-250.0_f32, 0.0, 250.0] {
            spawn_wall(
                &mut commands,
                Vec3::new(col, row, 0.0),
                Vec2::new(120.0, 24.0),
                table_color,
            );
        }
    }

    // === Lunch counter (top area) ===
    spawn_wall(
        &mut commands,
        Vec3::new(0.0, 260.0, 0.0),
        Vec2::new(400.0, 20.0),
        counter_color,
    );

    // Small walls/pillars for partial cover
    spawn_wall(
        &mut commands,
        Vec3::new(-380.0, 0.0, 0.0),
        Vec2::new(24.0, 80.0),
        wall_color,
    );
    spawn_wall(
        &mut commands,
        Vec3::new(380.0, 0.0, 0.0),
        Vec2::new(24.0, 80.0),
        wall_color,
    );

    // === Corner detention tables (visual only, no collision) ===
    // One per corner for lunch detention — players are banished here when eliminated.
    let detention_color = Color::srgb(0.45, 0.28, 0.15);
    for pos in [
        Vec3::new(-400.0, -260.0, 0.0), // bottom-left
        Vec3::new(400.0, -260.0, 0.0),  // bottom-right
        Vec3::new(-400.0, 260.0, 0.0),  // top-left
        Vec3::new(400.0, 260.0, 0.0),   // top-right
    ] {
        commands.spawn((
            Sprite {
                color: detention_color,
                custom_size: Some(Vec2::new(80.0, 50.0)),
                ..default()
            },
            Transform::from_translation(pos),
            Gameplay,
        ));
    }

    // Melee weapon spawn point markers — subtle floor markings at (-280, 50) and (280, -50)
    let melee_marker_color = Color::srgb(0.32, 0.28, 0.22);
    for (x, y) in [(-280.0_f32, 50.0_f32), (280.0, -50.0)] {
        commands.spawn((
            Sprite {
                color: melee_marker_color,
                custom_size: Some(Vec2::new(40.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(x, y, -0.5),
            Gameplay,
        ));
    }

    // Door openings (spawn locations for players) — marked by colored floor tiles
    let door_color = Color::srgb(0.3, 0.35, 0.28);
    commands.spawn((
        Sprite {
            color: door_color,
            custom_size: Some(Vec2::new(40.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(-400.0, -300.0, -0.5),
        Gameplay,
    ));
    commands.spawn((
        Sprite {
            color: door_color,
            custom_size: Some(Vec2::new(40.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(400.0, -300.0, -0.5),
        Gameplay,
    ));
}

fn spawn_wall(commands: &mut Commands, position: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(size),
            ..default()
        },
        Transform::from_translation(position),
        Wall {
            half_size: size / 2.0,
        },
        Gameplay,
    ));
}
