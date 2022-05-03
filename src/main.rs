#![allow(dead_code)]
#[cfg(feature = "ui-debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use bevy::{prelude::*, reflect::TypeUuid, window::WindowMode, asset::AssetServerSettings};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_asset_ron::*;
use bevy_parallax::ParallaxPlugin;
use bevy_tweening::TweeningPlugin;
use heron::PhysicsPlugin;
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

pub struct GameConfigController {
    handle: Handle<GameConfigAsset>,
}

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

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "DebugPixel.png")]
    debug_pixel: Handle<Image>,
    #[asset(path = "GameLogo.png")]
    game_logo: Handle<Image>,
    #[asset(path = "brand.png")]
    brand_logo: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct GlobalUIAssets {
    #[asset(path = "fonts/pixel_font.ttf")]
    pixel_font: Handle<Font>,
    #[asset(path = "fonts/tomorrow_night.ttf")]
    tomorrow_font: Handle<Font>,
}

fn main() {
    let mut app = App::new();

    AssetLoader::new(GameState::Splash)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<ImageAssets>()
        .with_collection::<EnviromentAssets>()
        .with_collection::<PlayerAssets>()
        .with_collection::<GlobalUIAssets>()
        .build(&mut app);

    app.insert_resource(WindowDescriptor {
        title: "Bevy Infinity Runner".to_string(),
        mode: WindowMode::BorderlessFullscreen,
        ..Default::default()
    })
    .insert_resource(AssetServerSettings {
        watch_for_changes: true,
        ..default()
    })
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(ClearColor(Color::rgb(
        0.462745098,
        0.576470588,
        0.701960784,
    )))
    .insert_resource(GameSettings::default())
    .add_state(GameState::Splash)
    .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(load_splash))
    .add_plugins(DefaultPlugins)
    .add_plugin(RonAssetPlugin::<GameConfigAsset>::new(&["ron"]))
    .add_startup_system(load_config)
    .add_plugin(TweeningPlugin)
    .add_plugin(TransitionPlugin)
    .add_plugin(ParallaxPlugin)
    .add_plugin(PhysicsPlugin::default())
    .add_plugin(InputManagerPlugin::<PlayerAction>::default())
    .add_plugin(MainMenu)
    .add_plugin(Enviroment)
    .add_plugin(AmbientAudioPlugin)
    .add_plugin(PlayerPlugin);

    #[cfg(feature = "ui-debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
fn load_config(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let handle = asset_server.load("config.ron");
    commands.insert_resource(GameConfigController { handle });
}
