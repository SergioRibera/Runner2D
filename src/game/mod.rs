#![allow(dead_code)]

pub mod enviroment;
pub mod player;
pub mod audio;
pub mod splash;

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
