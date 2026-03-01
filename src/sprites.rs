//! Sprite assets, frame-based animation, and NPC visual state feedback.
//!
//! `SpritePlugin` loads player sprite atlases at startup and runs animation systems
//! in `FixedUpdate`. `AnimationState` drives frame selection from named `FrameRange`s.
//! NPC color changes (yellow=suspicious, red=chasing) also live here.

use bevy::prelude::*;

use crate::food::components::{FoodType, Inventory};
use crate::npc::components::*;
use crate::player::components::{Player, Velocity};
use crate::states::GameState;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sprite_assets)
            .add_systems(
                FixedUpdate,
                (update_player_animation, update_npc_animation, animate_sprites)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

// --- Components ---

#[derive(Clone, Debug)]
pub struct FrameRange {
    pub start: usize,
    pub end: usize,
    pub fps: f32,
    pub looping: bool,
}

#[derive(Component)]
pub struct AnimationState {
    pub current_anim: &'static str,
    pub frame_range: FrameRange,
    pub current_frame: usize,
    pub timer: Timer,
    pub finished: bool,
}

impl AnimationState {
    pub fn new(name: &'static str, range: FrameRange) -> Self {
        let timer = Timer::from_seconds(1.0 / range.fps, TimerMode::Repeating);
        let start = range.start;
        Self {
            current_anim: name,
            frame_range: range,
            current_frame: start,
            timer,
            finished: false,
        }
    }

    pub fn set_animation(&mut self, name: &'static str, range: FrameRange) {
        if self.current_anim == name {
            return;
        }
        self.timer = Timer::from_seconds(1.0 / range.fps, TimerMode::Repeating);
        self.current_frame = range.start;
        self.current_anim = name;
        self.frame_range = range;
        self.finished = false;
    }
}

#[derive(Component)]
pub struct PlayerSpriteId(#[allow(dead_code)] pub u8); // indexes into SpriteAssets::player_images

// --- Resource ---

#[derive(Resource)]
pub struct SpriteAssets {
    pub player_images: [Handle<Image>; 4],
    pub player_layout: Handle<TextureAtlasLayout>,

    pub teacher_image: Handle<Image>,
    pub teacher_layout: Handle<TextureAtlasLayout>,
    pub principal_image: Handle<Image>,
    pub principal_layout: Handle<TextureAtlasLayout>,
    pub lunch_lady_image: Handle<Image>,
    pub lunch_lady_layout: Handle<TextureAtlasLayout>,

    pub food_image: Handle<Image>,
    pub food_layout: Handle<TextureAtlasLayout>,

    pub launcher_image: Handle<Image>,
    pub launcher_layout: Handle<TextureAtlasLayout>,

    pub effects_image: Handle<Image>,
    pub effects_layout: Handle<TextureAtlasLayout>,
}

// --- Loading ---

fn load_sprite_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        8,
        4,
        None,
        None,
    ));
    let teacher_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        7,
        4,
        None,
        None,
    ));
    let principal_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        7,
        4,
        None,
        None,
    ));
    let lunch_lady_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        6,
        3,
        None,
        None,
    ));
    let food_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        8,
        8,
        None,
        None,
    ));
    let launcher_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        5,
        6,
        None,
        None,
    ));
    let effects_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        6,
        3,
        None,
        None,
    ));

    commands.insert_resource(SpriteAssets {
        player_images: [
            asset_server.load("sprites/players/player_blue.png"),
            asset_server.load("sprites/players/player_red.png"),
            asset_server.load("sprites/players/player_green.png"),
            asset_server.load("sprites/players/player_yellow.png"),
        ],
        player_layout,
        teacher_image: asset_server.load("sprites/npcs/teacher.png"),
        teacher_layout,
        principal_image: asset_server.load("sprites/npcs/principal.png"),
        principal_layout,
        lunch_lady_image: asset_server.load("sprites/npcs/lunch_lady.png"),
        lunch_lady_layout,
        food_image: asset_server.load("sprites/food/food_items.png"),
        food_layout,
        launcher_image: asset_server.load("sprites/launchers/launchers.png"),
        launcher_layout,
        effects_image: asset_server.load("sprites/effects/effects.png"),
        effects_layout,
    });
}

// --- Animation Systems ---

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut AnimationState, &mut Sprite)>) {
    for (mut anim, mut sprite) in &mut query {
        if anim.finished {
            continue;
        }
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            let next = anim.current_frame + 1;
            if next > anim.frame_range.end {
                if anim.frame_range.looping {
                    anim.current_frame = anim.frame_range.start;
                } else {
                    anim.finished = true;
                }
            } else {
                anim.current_frame = next;
            }
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = anim.current_frame;
            }
        }
    }
}

fn update_player_animation(
    mut query: Query<
        (&Velocity, &Inventory, Option<&Caught>, &mut AnimationState),
        With<Player>,
    >,
) {
    for (velocity, inventory, caught, mut anim) in &mut query {
        let is_stunned = caught.is_some();
        let has_food = inventory.held_food.is_some();
        let (name, range) = player_animation_for(velocity.0, has_food, is_stunned);
        anim.set_animation(name, range);
    }
}

fn update_npc_animation(
    mut npcs: Query<(&NpcAuthority, &NpcState, &Facing, &mut AnimationState)>,
) {
    for (npc, state, facing, mut anim) in &mut npcs {
        let (name, range) = match npc.role {
            NpcRole::LunchLady => lunch_lady_animation_for(state, facing.0),
            _ => npc_standard_animation_for(state, facing.0, 7),
        };
        anim.set_animation(name, range);
    }
}

// --- Index Helpers ---

pub fn atlas_index(row: usize, col: usize, columns: usize) -> usize {
    row * columns + col
}

pub fn player_atlas_index(row: usize, col: usize) -> usize {
    atlas_index(row, col, 8)
}

pub fn food_atlas_index(row: usize, col: usize) -> usize {
    atlas_index(row, col, 8)
}

pub fn launcher_atlas_index(row: usize, col: usize) -> usize {
    atlas_index(row, col, 5)
}

pub fn effects_atlas_index(row: usize, col: usize) -> usize {
    atlas_index(row, col, 6)
}

// --- Type-to-Row Mappings ---

pub fn food_type_row(food_type: &FoodType) -> usize {
    match food_type {
        FoodType::Pizza => 0,
        FoodType::Meatball => 1,
        FoodType::Jello => 2,
        FoodType::Grape => 3,
        FoodType::MilkCarton => 4,
        FoodType::Spaghetti => 5,
        FoodType::BananaPeel => 6,
        FoodType::MysteryMeat => 7,
    }
}

pub fn launcher_type_row(launcher_type: &crate::food::launcher::LauncherType) -> usize {
    use crate::food::launcher::LauncherType;
    match launcher_type {
        LauncherType::Slingshot => 0,
        LauncherType::KetchupGun => 1,
        LauncherType::SporkLauncher => 2,
        LauncherType::LunchTrayCatapult => 3,
        LauncherType::StrawBlowgun => 4,
        LauncherType::WatermelonCatapult => 5,
    }
}

/// Maps a food type to the correct splat sprite index in the effects atlas.
pub fn food_splat_index(food_type: &FoodType) -> usize {
    let col = match food_type {
        FoodType::Pizza | FoodType::Meatball | FoodType::Spaghetti => 0, // splat_red
        FoodType::Jello => 1,       // splat_green
        FoodType::Grape => 2,       // splat_purple
        FoodType::MilkCarton => 3,  // splat_white
        FoodType::BananaPeel => 4,  // splat_yellow
        FoodType::MysteryMeat => 5, // splat_brown
    };
    effects_atlas_index(1, col)
}

// --- Animation Selection ---

fn player_animation_for(velocity: Vec2, has_food: bool, is_stunned: bool) -> (&'static str, FrameRange) {
    if is_stunned {
        return (
            "stunned",
            FrameRange {
                start: player_atlas_index(0, 6),
                end: player_atlas_index(0, 7),
                fps: 4.0,
                looping: true,
            },
        );
    }

    if velocity.length_squared() < 0.01 {
        if has_food {
            return (
                "holding_food",
                FrameRange {
                    start: player_atlas_index(3, 4),
                    end: player_atlas_index(3, 5),
                    fps: 3.0,
                    looping: true,
                },
            );
        }
        return (
            "idle",
            FrameRange {
                start: player_atlas_index(0, 4),
                end: player_atlas_index(0, 5),
                fps: 3.0,
                looping: true,
            },
        );
    }

    let dir = velocity.normalize();
    if dir.y < -0.5 {
        (
            "walk_down",
            FrameRange {
                start: player_atlas_index(0, 0),
                end: player_atlas_index(0, 3),
                fps: 8.0,
                looping: true,
            },
        )
    } else if dir.y > 0.5 {
        (
            "walk_up",
            FrameRange {
                start: player_atlas_index(1, 0),
                end: player_atlas_index(1, 3),
                fps: 8.0,
                looping: true,
            },
        )
    } else if dir.x < 0.0 {
        (
            "walk_left",
            FrameRange {
                start: player_atlas_index(2, 0),
                end: player_atlas_index(2, 3),
                fps: 8.0,
                looping: true,
            },
        )
    } else {
        (
            "walk_right",
            FrameRange {
                start: player_atlas_index(3, 0),
                end: player_atlas_index(3, 3),
                fps: 8.0,
                looping: true,
            },
        )
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn facing_to_direction(facing: Vec2) -> Dir {
    if facing.y.abs() > facing.x.abs() {
        if facing.y > 0.0 { Dir::Up } else { Dir::Down }
    } else if facing.x > 0.0 {
        Dir::Right
    } else {
        Dir::Left
    }
}

fn npc_standard_animation_for(
    state: &NpcState,
    facing: Vec2,
    columns: usize,
) -> (&'static str, FrameRange) {
    let dir = facing_to_direction(facing);
    match state {
        NpcState::Patrolling { .. } => {
            let (name, row, start, end, fps) = match dir {
                Dir::Down => ("patrol_down", 0, 0, 1, 4.0),
                Dir::Up => ("patrol_up", 0, 2, 3, 4.0),
                Dir::Left => ("patrol_left", 0, 4, 4, 1.0),
                Dir::Right => ("patrol_right", 0, 5, 5, 1.0),
            };
            (
                name,
                FrameRange {
                    start: atlas_index(row, start, columns),
                    end: atlas_index(row, end, columns),
                    fps,
                    looping: true,
                },
            )
        }
        NpcState::Suspicious { .. } => {
            let (name, row, start, end, fps) = match dir {
                Dir::Down => ("suspicious_down", 1, 0, 1, 4.0),
                Dir::Up => ("suspicious_up", 1, 2, 3, 4.0),
                Dir::Left => ("suspicious_left", 1, 4, 4, 1.0),
                Dir::Right => ("suspicious_right", 1, 5, 5, 1.0),
            };
            (
                name,
                FrameRange {
                    start: atlas_index(row, start, columns),
                    end: atlas_index(row, end, columns),
                    fps,
                    looping: true,
                },
            )
        }
        NpcState::Chasing { .. } => {
            let (name, row, start, end, fps) = match dir {
                Dir::Down => ("chase_down", 2, 0, 1, 6.0),
                Dir::Up => ("chase_up", 2, 2, 3, 6.0),
                Dir::Left => ("chase_left", 2, 4, 4, 1.0),
                Dir::Right => ("chase_right", 2, 5, 5, 1.0),
            };
            (
                name,
                FrameRange {
                    start: atlas_index(row, start, columns),
                    end: atlas_index(row, end, columns),
                    fps,
                    looping: true,
                },
            )
        }
        NpcState::Returning { .. } => (
            "returning",
            FrameRange {
                start: atlas_index(3, 5, columns),
                end: atlas_index(3, 6, columns),
                fps: 3.0,
                looping: true,
            },
        ),
    }
}

fn lunch_lady_animation_for(state: &NpcState, facing: Vec2) -> (&'static str, FrameRange) {
    let columns = 6;
    match state {
        NpcState::Patrolling { .. } | NpcState::Returning { .. } => (
            "idle_stir",
            FrameRange {
                start: atlas_index(0, 0, columns),
                end: atlas_index(0, 1, columns),
                fps: 2.0,
                looping: true,
            },
        ),
        NpcState::Suspicious { .. } => (
            "suspicious_stir",
            FrameRange {
                start: atlas_index(1, 0, columns),
                end: atlas_index(1, 1, columns),
                fps: 2.0,
                looping: true,
            },
        ),
        NpcState::Chasing { .. } => {
            let dir = facing_to_direction(facing);
            match dir {
                Dir::Left | Dir::Up => (
                    "swing_left",
                    FrameRange {
                        start: atlas_index(2, 0, columns),
                        end: atlas_index(2, 1, columns),
                        fps: 6.0,
                        looping: true,
                    },
                ),
                _ => (
                    "swing_right",
                    FrameRange {
                        start: atlas_index(2, 2, columns),
                        end: atlas_index(2, 3, columns),
                        fps: 6.0,
                        looping: true,
                    },
                ),
            }
        }
    }
}
