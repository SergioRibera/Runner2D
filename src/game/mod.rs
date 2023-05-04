#![allow(dead_code)]

use bevy::prelude::*;

pub mod audio;
pub mod enviroment;
pub mod mainmenu;
pub mod platform;
pub mod player;
pub mod splash;
pub mod transition;

#[derive(Resource)]
pub struct GameSettings {
    pub music_volume: f32,
    pub vfx_volume: f32,
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            music_volume: 0.5,
            vfx_volume: 0.5,
            fullscreen: false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    SplashEnd,
    MainMenu,
    GameLoading,
    InGame,
    Paused,
    GameOver,
}

