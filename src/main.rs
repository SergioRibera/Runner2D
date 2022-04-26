#[cfg(feature = "ui-debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use bevy::prelude::*;

mod game;

use game::{enviroment::Enviroment, player::PlayerPlugin};

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Bevy Infinity Runner".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(Enviroment)
    .add_plugin(PlayerPlugin);
    // .add_system(load_scene_system);

    #[cfg(feature = "ui-debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
