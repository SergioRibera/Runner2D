use bevy::prelude::*;

pub struct Enviroment;

//
// Data to configure the plugin
//
pub struct EnviromentResource {
}

impl Default for EnviromentResource {
    fn default() -> Self {
        Self {
        }
    }
}

//
// Plugin to handle the enviroment
//
impl Plugin for Enviroment {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnviromentResource {
            ..Default::default()
        });
        app.add_startup_system(setup_enviroment);
    }
}

fn get_resource_name(name: &str) -> String {
    format!("enviroment/Layer_00{}.png", name)
}

fn setup_enviroment(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(get_resource_name("11_0").as_str()),
        ..default()
    });


    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(get_resource_name("05_5").as_str()),
        ..default()
    });
}
