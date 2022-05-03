#![allow(dead_code)]

use bevy::prelude::*;
use heron::prelude::*;
use bevy_parallax::{LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxResource};
use bevy_asset_loader::AssetCollection;

use super::player::PLAYER_SPEED;

const ENVIROMENT_WIDTH: f32 = 928.0;
const ENVIROMENT_HEIGHT: f32 = 793.0;

pub struct Enviroment;

#[derive(Component)]
pub struct Floor;

#[derive(AssetCollection)]
pub struct EnviromentAssets {
    #[asset(path = "audio/game_ambient.ogg")]
    pub background: Handle<AudioSource>,
    #[asset(path = "enviroment/Layer_0009_2.png")]
    pub layer_0: Handle<Image>,
}

#[derive(PhysicsLayer)]
pub enum Layer {
    Player,
    World,
    Enemy
}

//
// Plugin to handle the enviroment
//
impl Plugin for Enviroment {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParallaxResource {
            layer_data: vec![
                // Layer 1
                LayerData {
                    speed: 1.0,
                    path: get_resource_name("09_2"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    cols: 1,
                    rows: 1,
                    scale: 1.2,
                    z: 0.5,
                    position: Vec2::new(0.0, ENVIROMENT_HEIGHT / 4.2),
                    ..Default::default()
                },
                // Gray trees
                LayerData {
                    speed: 0.8,
                    path: get_resource_name("05_5"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    cols: 1,
                    rows: 1,
                    scale: 1.5,
                    z: 1.0,
                    position: Vec2::new(0.0, ENVIROMENT_HEIGHT / 4.0),
                    ..Default::default()
                },
                // More proximite trees
                LayerData {
                    speed: 0.7,
                    path: get_resource_name("03_6"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    cols: 1,
                    rows: 1,
                    scale: 1.5,
                    z: 1.1,
                    position: Vec2::new(0.0, ENVIROMENT_HEIGHT / 4.0),
                    ..Default::default()
                },
                // Top leaf
                LayerData {
                    speed: 0.6,
                    path: get_resource_name("02_7"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    cols: 1,
                    rows: 1,
                    scale: 1.5,
                    z: 1.1,
                    position: Vec2::new(0.0, ENVIROMENT_HEIGHT / 4.5),
                    ..Default::default()
                },
                // Floor of leaf
                LayerData {
                    speed: 0.5,
                    path: get_resource_name("02_7"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    cols: 1,
                    rows: 1,
                    scale: 1.5,
                    z: 2.0,
                    position: Vec2::new(0.0, -570.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        app.add_startup_system(setup_enviroment);
        // app.add_system(move_camera_system);
    }
}

fn get_resource_name(name: &str) -> String {
    format!("enviroment/Layer_00{}.png", name)
}

fn setup_enviroment(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary().unwrap();
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(ParallaxCameraComponent);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("DebugPixel.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -(window.height() * 0.34), 5.0),
                ..Default::default()

            },
            ..default()
        })
        .insert(Floor)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(window.width(), 50.0).extend(0.0),
            border_radius: None,
        })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Static);
}

pub fn move_camera_system(
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    mut query: Query<(&mut Floor, &mut Transform)>,
) {
    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: PLAYER_SPEED,
    });

    for (_floor, mut transform) in query.iter_mut() {
        transform.translation.x += PLAYER_SPEED;
    }
}
