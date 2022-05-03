#![allow(dead_code)]

use bevy::prelude::{Gamepad, GamepadButtonType, KeyCode};
use leafwing_input_manager::prelude::*;

use self::player::PlayerAction;

pub mod audio;
pub mod enviroment;
pub mod mainmenu;
pub mod platform;
pub mod player;
pub mod splash;
pub mod transition;

pub struct GameSettings {
    pub music_volume: f32,
    pub vfx_volume: f32,
    pub fullscreen: bool,
    pub player_ctrl: InputMap<PlayerAction>,
}

impl Default for GameSettings {
    fn default() -> Self {
        let mut ctrl = InputMap::default();

        ctrl.set_gamepad(Gamepad(0));

        ctrl.insert(PlayerAction::Pause, KeyCode::Escape);
        ctrl.insert(PlayerAction::Pause, GamepadButtonType::Select);

        // Move to left
        ctrl.insert(PlayerAction::MoveLeft, KeyCode::A);
        ctrl.insert(PlayerAction::MoveLeft, KeyCode::Left);
        ctrl.insert(PlayerAction::MoveLeft, GamepadButtonType::East);

        // Move to right
        ctrl.insert(PlayerAction::MoveRight, KeyCode::D);
        ctrl.insert(PlayerAction::MoveRight, KeyCode::Right);
        ctrl.insert(PlayerAction::MoveRight, GamepadButtonType::West);

        // Jump
        ctrl.insert(PlayerAction::Jump, KeyCode::Space);
        ctrl.insert(PlayerAction::Jump, GamepadButtonType::South);

        GameSettings {
            music_volume: 0.5,
            vfx_volume: 0.5,
            fullscreen: false,
            player_ctrl: ctrl,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Splash,
    SplashEnd,
    MainMenu,
    GameLoading,
    InGame,
    Paused,
    GameOver,
}
