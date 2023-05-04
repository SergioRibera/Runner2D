use bevy::{audio::AudioSink, prelude::*};

use crate::{GameConfigAsset, GameConfigAssetHandler};

use super::{enviroment::EnviromentAssets, GameState};

pub struct AmbientAudioPlugin;

impl Plugin for AmbientAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            setup
                .run_if(resource_exists::<GameConfigAssetHandler>())
                .in_schedule(OnEnter(GameState::MainMenu)),
        );
    }
}

#[derive(Resource)]
pub struct MusicController(Handle<AudioSink>);

fn setup(
    mut commands: Commands,
    audio_assets: Res<EnviromentAssets>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    assets: Res<Assets<GameConfigAsset>>,
    q: Res<GameConfigAssetHandler>,
) {
    let cfg = assets.get(&q.0).unwrap();
    commands.remove_resource::<MusicController>();
    let handle = audio_sinks.get_handle(audio.play_with_settings(
        audio_assets.background.clone(),
        PlaybackSettings {
            repeat: true,
            volume: cfg.audio_volume,
            ..Default::default()
        },
    ));
    commands.insert_resource(MusicController(handle));
}
