#![allow(dead_code)]
#[cfg(feature = "ui-debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use bevy::{prelude::*, window::WindowMode};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_parallax::ParallaxPlugin;
use bevy_tweening::TweeningPlugin;
use heron::{Gravity, PhysicsPlugin};

mod game;

use game::{
    audio::AmbientAudioPlugin,
    enviroment::{Enviroment, EnviromentAssets},
    player::{PlayerAssets, PlayerPlugin},
    splash::{load_splash, on_splash},
    GameState, mainmenu::MainMenu,
};

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
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(ClearColor(Color::rgb(
        0.462745098,
        0.576470588,
        0.701960784,
    )))
    .add_state(GameState::Splash)
    // .add_system_set(
    //     SystemSet::on_enter(GameState::Splash)
    //         .with_system(load_splash)
    // )
    // .add_system_set(
    //     SystemSet::on_enter(GameState::SplashEnd)
    //         .with_system(on_splash_end)
    // )
    // .add_system(on_splash)
    .insert_resource(Gravity::from(Vec3::new(0.0, -9.81 * 10., 0.0)))
    .add_plugins(DefaultPlugins)
    .add_plugin(TweeningPlugin)
    .add_plugin(ParallaxPlugin)
    .add_plugin(PhysicsPlugin::default())
    .add_plugin(MainMenu)
    .add_plugin(Enviroment)
    .add_plugin(AmbientAudioPlugin)
    .add_plugin(PlayerPlugin);

    #[cfg(feature = "ui-debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
