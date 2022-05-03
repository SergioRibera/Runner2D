use bevy::{audio::AudioSink, prelude::*};

use super::{enviroment::EnviromentAssets, GameState};

pub struct AmbientAudioPlugin;

impl Plugin for AmbientAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(setup),
        );
    }
}

pub struct MusicController(Handle<AudioSink>);

fn setup(
    mut commands: Commands,
    audio_assets: Res<EnviromentAssets>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    commands.remove_resource::<MusicController>();
    let handle = audio_sinks.get_handle(audio.play_with_settings(
        audio_assets.background.clone(),
        PlaybackSettings {
            repeat: true,
            volume: 0.15,
            ..Default::default()
        },
    ));
    commands.insert_resource(MusicController(handle));
}
