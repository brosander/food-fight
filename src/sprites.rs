//! Sprite assets, frame-based animation, and NPC visual state feedback.
//!
//! `SpritePlugin` loads player sprite atlases at startup and runs animation systems
//! in `FixedUpdate`. `AnimationState` drives frame selection from named `FrameRange`s.
//! NPC color changes (yellow=suspicious, red=chasing) also live here.

use bevy::prelude::*;

use crate::food::components::{
    Blocking, EquippedMeleeWeapon, FoodType, Inventory, MeleeVisual, MeleeWeaponType, ParryWindow,
};
use crate::input::ControllerInput;
use crate::npc::components::*;
use crate::player::components::{Player, Velocity};
use crate::states::{GameState, Gameplay};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sprite_assets)
            .add_systems(
                FixedUpdate,
                (
                    despawn_melee_visuals,
                    spawn_melee_visuals,
                    update_player_animation,
                    update_npc_animation,
                    update_melee_animation,
                    sync_melee_visual_position,
                    animate_sprites,
                )
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

    pub melee_image: Handle<Image>,
    pub melee_layout: Handle<TextureAtlasLayout>,
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
    let melee_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        6,
        2,
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
        melee_image: asset_server.load("sprites/melee/melee_weapons.png"),
        melee_layout,
    });
}

// --- Melee Visual Lifecycle Systems ---

/// Spawns a weapon overlay sprite when a player picks up a melee weapon.
fn spawn_melee_visuals(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
    added: Query<(Entity, &EquippedMeleeWeapon), Added<EquippedMeleeWeapon>>,
    visuals: Query<&MeleeVisual>,
) {
    for (player_entity, weapon) in &added {
        // Guard against duplicates (can happen when Added<T> fires during a swap).
        if visuals.iter().any(|v| v.player_entity == player_entity) {
            continue;
        }
        let row = melee_weapon_type_row(&weapon.weapon_type);
        let equipped_idx = melee_atlas_index(row, 2);
        commands.spawn((
            Sprite {
                image: sprite_assets.melee_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_assets.melee_layout.clone(),
                    index: equipped_idx,
                }),
                custom_size: Some(Vec2::splat(32.0)),
                color: Color::srgba(1.0, 1.0, 1.0, 0.0), // hidden until first update
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.2),
            AnimationState::new(
                "equipped",
                FrameRange {
                    start: equipped_idx,
                    end: equipped_idx,
                    fps: 3.0,
                    looping: true,
                },
            ),
            MeleeVisual { player_entity },
            Gameplay,
        ));
    }
}

/// Despawns weapon overlay sprites when a player drops or loses their melee weapon.
/// Skips despawn when the removal is part of a weapon swap (player still has
/// EquippedMeleeWeapon after the command batch is applied); the existing visual
/// continues to track the player and update_melee_animation reads the new weapon type.
fn despawn_melee_visuals(
    mut commands: Commands,
    mut removed: RemovedComponents<EquippedMeleeWeapon>,
    still_armed: Query<(), With<EquippedMeleeWeapon>>,
    visuals: Query<(Entity, &MeleeVisual)>,
) {
    for player_entity in removed.read() {
        if still_armed.get(player_entity).is_ok() {
            continue; // swap, not a drop — keep the visual
        }
        for (visual_entity, visual) in &visuals {
            if visual.player_entity == player_entity {
                commands.entity(visual_entity).despawn();
            }
        }
    }
}

/// Keeps the weapon overlay sprite on the owning player each tick.
/// Baguette swing: arcs right-to-left relative to facing, peaking forward at mid-frame.
/// LunchTray parry/block: shifts forward in the facing direction and rotates to match.
fn sync_melee_visual_position(
    players: Query<
        (&Transform, &ControllerInput, Option<&EquippedMeleeWeapon>, Option<&ParryWindow>, Option<&Blocking>),
        (With<Player>, Without<MeleeVisual>),
    >,
    mut visuals: Query<(&MeleeVisual, &AnimationState, &mut Transform)>,
) {
    for (visual, anim, mut tf) in &mut visuals {
        let Ok((player_tf, input, weapon, parry, blocking)) = players.get(visual.player_entity) else {
            continue;
        };

        let base = Vec3::new(player_tf.translation.x, player_tf.translation.y, 1.2);

        match weapon {
            Some(w) if w.weapon_type == MeleeWeaponType::Baguette
                && anim.current_anim == "swing" && !anim.finished =>
            {
                let range_len = (anim.frame_range.end - anim.frame_range.start) as f32;
                let t = if range_len > 0.0 {
                    (anim.current_frame - anim.frame_range.start) as f32 / range_len
                } else {
                    0.0
                };
                let forward = Vec3::new(w.swing_facing.x, w.swing_facing.y, 0.0);
                let right = Vec3::new(w.swing_facing.y, -w.swing_facing.x, 0.0);
                let lateral = right * (1.0 - 2.0 * t) * 22.0;
                let fwd_dist = 15.0 + 15.0 * (1.0 - (2.0 * t - 1.0).powi(2));
                tf.translation = base + forward * fwd_dist + lateral;
                tf.rotation = Quat::IDENTITY;
            }
            Some(w) if w.weapon_type == MeleeWeaponType::LunchTray
                && (parry.is_some() || blocking.is_some()) =>
            {
                let s = input.move_stick;
                let facing = if s != Vec2::ZERO { s.normalize() } else { Vec2::Y };
                tf.translation = base + Vec3::new(facing.x, facing.y, 0.0) * 20.0;
                tf.rotation = Quat::from_rotation_z(facing.to_angle());
            }
            _ => {
                tf.translation = base;
                tf.rotation = Quat::IDENTITY;
            }
        }
    }
}

/// Drives animation on weapon overlay sprites based on the player's melee state.
///
/// LunchTray:  parry  → col 3 (flash, visible)
///             block  → cols 4-5 loop (pulse, visible)
///             idle   → hidden (alpha 0 — tray only shows when active)
/// Baguette:   swing  → cols 3-5 one-shot (windup → peak → recovery, always visible)
///             idle   → col 2 (equipped)
///
/// Also applies the first frame immediately on animation transitions so the windup
/// frame is never skipped (set_animation doesn't update atlas.index, animate_sprites
/// only does it on timer fire).
fn update_melee_animation(
    mut players: Query<(
        &mut EquippedMeleeWeapon,
        Option<&ParryWindow>,
        Option<&Blocking>,
    )>,
    mut visuals: Query<(&MeleeVisual, &mut AnimationState, &mut Sprite)>,
) {
    for (visual, mut anim, mut sprite) in &mut visuals {
        let Ok((mut weapon, parry, blocking)) = players.get_mut(visual.player_entity) else {
            continue;
        };
        let row = melee_weapon_type_row(&weapon.weapon_type);
        let prev_anim = anim.current_anim;

        match weapon.weapon_type {
            MeleeWeaponType::Baguette => {
                if weapon.swinging {
                    sprite.color = Color::WHITE;
                    anim.set_animation(
                        "swing",
                        FrameRange {
                            start: melee_atlas_index(row, 3),
                            end:   melee_atlas_index(row, 5),
                            fps:   12.0,
                            looping: false,
                        },
                    );
                    weapon.swinging = false;
                } else if anim.current_anim == "swing" && !anim.finished {
                    // Keep visible while swing is playing
                    sprite.color = Color::WHITE;
                } else {
                    sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.0);
                    anim.set_animation(
                        "equipped",
                        FrameRange {
                            start: melee_atlas_index(row, 2),
                            end:   melee_atlas_index(row, 2),
                            fps:   3.0,
                            looping: true,
                        },
                    );
                }
            }
            MeleeWeaponType::LunchTray => {
                if parry.is_some() {
                    sprite.color = Color::WHITE;
                    anim.set_animation(
                        "parry",
                        FrameRange {
                            start: melee_atlas_index(row, 3),
                            end:   melee_atlas_index(row, 3),
                            fps:   15.0,
                            looping: true,
                        },
                    );
                } else if blocking.is_some() {
                    sprite.color = Color::WHITE;
                    anim.set_animation(
                        "blocking",
                        FrameRange {
                            start: melee_atlas_index(row, 4),
                            end:   melee_atlas_index(row, 5),
                            fps:   4.0,
                            looping: true,
                        },
                    );
                } else {
                    sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.0);
                    anim.set_animation(
                        "equipped",
                        FrameRange {
                            start: melee_atlas_index(row, 2),
                            end:   melee_atlas_index(row, 2),
                            fps:   3.0,
                            looping: true,
                        },
                    );
                }
            }
        }

        // Apply the initial frame immediately on any animation transition.
        // animate_sprites only updates atlas.index on timer fire, so without this
        // the first frame of a new animation would be skipped entirely.
        if anim.current_anim != prev_anim {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = anim.current_frame;
            }
        }
    }
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

pub fn melee_atlas_index(row: usize, col: usize) -> usize {
    atlas_index(row, col, 6)
}

pub fn melee_weapon_type_row(weapon_type: &crate::food::components::MeleeWeaponType) -> usize {
    use crate::food::components::MeleeWeaponType;
    match weapon_type {
        MeleeWeaponType::LunchTray => 0,
        MeleeWeaponType::Baguette  => 1,
    }
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
