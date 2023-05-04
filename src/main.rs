#![allow(dead_code)]
#[cfg(feature = "ui-debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::{prelude::*, reflect::TypeUuid, window::WindowMode};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
// use bevy_parallax::ParallaxPlugin;
use bevy_rapier2d::{
    plugin::RapierPhysicsPlugin, prelude::NoUserData, render::RapierDebugRenderPlugin,
};
use bevy_tweening::TweeningPlugin;
use leafwing_input_manager::prelude::*;

mod game;

use game::{
    audio::AmbientAudioPlugin,
    enviroment::{Enviroment, EnviromentAssets},
    mainmenu::MainMenu,
    player::{PlayerAction, PlayerAssets, PlayerPlugin},
    splash::load_splash,
    transition::TransitionPlugin,
    GameSettings, GameState,
};

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "b7f64775-6e72-4080-9ced-167607f1f0b2"]
pub struct GameConfigAsset {
    pub gravity_multiplier: f32,
    pub player_initial_pos_x: f32,
    pub player_size_x: f32,
    pub player_size_y: f32,
    pub player_box_size_x: f32,
    pub player_box_size_y: f32,
    pub audio_volume: f32,
    pub floor_multiplier: f32,
}

#[derive(Resource)]
pub struct GameConfigAssetHandler(Handle<GameConfigAsset>);

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "DebugPixel.png")]
    debug_pixel: Handle<Image>,
    #[asset(path = "GameLogo.png")]
    game_logo: Handle<Image>,
    #[asset(path = "brand.png")]
    brand_logo: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct GlobalUIAssets {
    #[asset(path = "fonts/pixel_font.ttf")]
    pixel_font: Handle<Font>,
    #[asset(path = "fonts/tomorrow_night.ttf")]
    tomorrow_font: Handle<Font>,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(
        0.462_745_1,
        0.576_470_6,
        0.701_960_8,
    )))
    .insert_resource(GameSettings::default())
    .insert_resource(Msaa::Sample4)
    .add_state::<GameState>()
    .add_loading_state(LoadingState::new(GameState::Splash))
    .add_collection_to_loading_state::<_, ImageAssets>(GameState::Splash)
    .add_collection_to_loading_state::<_, EnviromentAssets>(GameState::Splash)
    .add_collection_to_loading_state::<_, PlayerAssets>(GameState::Splash)
    .add_collection_to_loading_state::<_, GlobalUIAssets>(GameState::Splash)
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Infinity Runner".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    }))
    .add_plugin(RonAssetPlugin::<GameConfigAsset>::new(&["config.ron"]))
    .add_startup_system(setup)
    .add_plugin(load_splash())
    // .add_plugin(TweeningPlugin)
    .add_plugin(TransitionPlugin)
    // .add_plugin(ParallaxPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(InputManagerPlugin::<PlayerAction>::default())
    .add_plugin(MainMenu)
    .add_plugin(Enviroment)
    .add_plugin(AmbientAudioPlugin)
    .add_plugin(PlayerPlugin);

    #[cfg(feature = "ui-debug")]
    app.add_plugin(RapierDebugRenderPlugin::default());

    #[cfg(feature = "ui-debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameConfigAssetHandler(asset_server.load("config.ron")));
    commands.spawn(Camera2dBundle::default());
}
