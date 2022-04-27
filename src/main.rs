#[cfg(feature = "ui-debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use bevy::{prelude::*, window::WindowMode};
use bevy_parallax::ParallaxPlugin;

mod game;

use game::{enviroment::Enviroment, player::PlayerPlugin, audio::AmbientAudioPlugin};

fn main() {
    let mut app = App::new();
    app
    .insert_resource(WindowDescriptor {
        title: "Bevy Infinity Runner".to_string(),
        // width: 1280.0,
        // height: 720.0,
        mode: WindowMode::BorderlessFullscreen,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.462745098, 0.576470588, 0.701960784)))
    .add_plugins(DefaultPlugins)
    .add_plugin(ParallaxPlugin)
    .add_plugin(Enviroment)
    .add_plugin(AmbientAudioPlugin)
    .add_plugin(PlayerPlugin);
    // .add_system(load_scene_system);

    #[cfg(feature = "ui-debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
