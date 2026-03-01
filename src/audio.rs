use bevy::prelude::*;

use crate::states::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .add_systems(Startup, load_sound_assets)
            .add_systems(Update, play_sounds)
            .add_systems(OnEnter(GameState::RoundOver), play_round_over);
    }
}

/// Game sound effect events. Send one of these to play the corresponding sound.
#[derive(Event, Clone, Copy)]
pub enum SoundEvent {
    FoodPickup,
    LauncherPickup,
    FoodThrow,
    LauncherFire,
    FoodHit,
    PlayerCaught,
}

#[derive(Resource)]
pub struct SoundAssets {
    pub food_pickup: Handle<AudioSource>,
    pub launcher_pickup: Handle<AudioSource>,
    pub food_throw: Handle<AudioSource>,
    pub launcher_fire: Handle<AudioSource>,
    pub food_hit: Handle<AudioSource>,
    pub player_caught: Handle<AudioSource>,
    pub round_over: Handle<AudioSource>,
}

fn load_sound_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundAssets {
        food_pickup:     asset_server.load("sounds/food_pickup.ogg"),
        launcher_pickup: asset_server.load("sounds/launcher_pickup.ogg"),
        food_throw:      asset_server.load("sounds/food_throw.ogg"),
        launcher_fire:   asset_server.load("sounds/launcher_fire.ogg"),
        food_hit:        asset_server.load("sounds/food_hit.ogg"),
        player_caught:   asset_server.load("sounds/player_caught.ogg"),
        round_over:      asset_server.load("sounds/round_over.ogg"),
    });
}

fn play_sounds(
    mut commands: Commands,
    mut events: EventReader<SoundEvent>,
    sounds: Res<SoundAssets>,
) {
    for event in events.read() {
        let source = match event {
            SoundEvent::FoodPickup     => sounds.food_pickup.clone(),
            SoundEvent::LauncherPickup => sounds.launcher_pickup.clone(),
            SoundEvent::FoodThrow      => sounds.food_throw.clone(),
            SoundEvent::LauncherFire   => sounds.launcher_fire.clone(),
            SoundEvent::FoodHit        => sounds.food_hit.clone(),
            SoundEvent::PlayerCaught   => sounds.player_caught.clone(),
        };
        commands.spawn((AudioPlayer(source), PlaybackSettings::DESPAWN));
    }
}

fn play_round_over(mut commands: Commands, sounds: Res<SoundAssets>) {
    commands.spawn((AudioPlayer(sounds.round_over.clone()), PlaybackSettings::DESPAWN));
}
