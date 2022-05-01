use bevy::{audio::AudioSink, prelude::*};

use super::GameState;

pub struct AmbientAudioPlugin;

impl Plugin for AmbientAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub struct MusicController(Handle<AudioSink>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    _game_state: Res<State<GameState>>,
) {
    // if game_state.current().eq(&GameState::Splash) || game_state.current().eq(&GameState::SplashEnd) {
    //     return;
    // }

    // let ambient_music = asset_server.load("audio/game_ambient.ogg");
    // let handle = audio_sinks.get_handle(audio.play_with_settings(
    //     ambient_music,
    //     PlaybackSettings {
    //         repeat: true,
    //         volume: 0.15,
    //         ..Default::default()
    //     },
    // ));
    //
    // commands.insert_resource(MusicController(handle));
}
