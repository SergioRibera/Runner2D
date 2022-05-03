#![allow(dead_code)]

use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_parallax::{LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxResource};
use heron::prelude::*;

use crate::{GameConfigAsset, GameConfigController};

use super::{player::PLAYER_SPEED, GameState, platform::draw_atlas};

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
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 12, rows = 12))]
    #[asset(path = "enviroment/platforms.png")]
    pub platforms: Handle<TextureAtlas>,
}

#[derive(PhysicsLayer)]
pub enum Layer {
    Player,
    World,
    Enemy,
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
        })
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_enviroment).with_system(draw_atlas))
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(move_camera_system));
    }
}

fn get_resource_name(name: &str) -> String {
    format!("enviroment/Layer_00{}.png", name)
}

fn setup_enviroment(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: ResMut<AssetServer>,
    assets: Res<Assets<GameConfigAsset>>,
    q: Res<GameConfigController>,
) {
    let cfg = assets.get(q.handle.clone()).unwrap();
    let window = windows.get_primary().unwrap();
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(ParallaxCameraComponent);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("DebugPixel.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -(window.height() * cfg.floor_multiplier), 5.0),
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
