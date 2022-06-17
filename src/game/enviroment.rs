#![allow(dead_code)]

use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
// use bevy_parallax::{LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxResource};
use heron::prelude::*;

use crate::{GameConfigAsset, GameConfigController};

use super::{platform::draw_atlas, player::PLAYER_SPEED, GameState};

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

// Parallax system
#[derive(Component)]
pub struct ParallaxItem(pub LayerData);

pub struct LayerData {
    pub speed: Vec2,
    pub path: String,
    pub tile_size: Vec2,
    pub scale: f32,
    pub z: f32,
    pub position: Vec2,
    pub transition_factor: f32,
}

impl Default for LayerData {
    fn default() -> Self {
        Self {
            speed: Vec2::new(1.0, 0.0),
            path: "".to_string(),
            tile_size: Vec2::ZERO,
            scale: 1.0,
            z: 0.0,
            position: Vec2::ZERO,
            transition_factor: 1.2,
        }
    }
}

impl Clone for LayerData {
    fn clone(&self) -> Self {
        Self {
            speed: self.speed,
            path: self.path.clone(),
            tile_size: self.tile_size,
            scale: self.scale,
            z: self.z,
            position: self.position,
            transition_factor: self.transition_factor,
        }
    }
}

pub struct ParallaxResource {
    pub layers: Vec<LayerData>,
    pub count: u32,
    pub initial_speed: f32,
}

//
// Plugin to handle the enviroment
//
impl Plugin for Enviroment {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParallaxResource {
            count: 4,
            initial_speed: PLAYER_SPEED,
            layers: vec![
                // Layer 1
                LayerData {
                    speed: Vec2::new(-1.0, 0.0),
                    path: get_resource_name("09_2"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.2,
                    z: 0.5,
                    position: Vec2::new(- ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT / 4.2),
                    ..Default::default()
                },
                // Gray trees
                LayerData {
                    speed: Vec2::new(0.8, 0.0),
                    path: get_resource_name("05_5"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.2,
                    z: 1.0,
                    position: Vec2::new(- ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT / 4.0),
                    ..Default::default()
                },
                // More proximite trees
                LayerData {
                    speed: Vec2::new(0.7, 0.0),
                    path: get_resource_name("03_6"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.2,
                    z: 1.1,
                    position: Vec2::new(- ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT / 4.0),
                    ..Default::default()
                },
                // Top leaf
                LayerData {
                    speed: Vec2::new(0.6, 0.0),
                    path: get_resource_name("02_7"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.2,
                    z: 1.1,
                    position: Vec2::new(- ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT / 4.5),
                    ..Default::default()
                },
                // Floor of leaf
                LayerData {
                    speed: Vec2::new(0.5, 0.0),
                    path: get_resource_name("02_7"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.,
                    z: 2.0,
                    position: Vec2::new(- ENVIROMENT_WIDTH, -370.0),
                    ..Default::default()
                },
            ],
        })
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(setup_enviroment)
                .with_system(draw_atlas),
        )
        // .add_system(move_parallax_system)
        ;
        // .add_system_set(SystemSet::on_update(GameState::InGame).with_system(move_parallax_system));
    }
}

fn move_parallax_system(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &ParallaxItem)>,
    game_state: Res<State<GameState>>,
    layer: Res<ParallaxResource>,
) {
    match game_state.current() {
        GameState::MainMenu | GameState::InGame => {
            let window = windows.get_primary().unwrap();
            for (mut transform, with) in query.iter_mut() {
                // if (with.0.position.x
                //     + (with.0.tile_size.x * with.0.scale) * with.0.transition_factor)
                //     - transform.translation.x
                //     > 0.
                //     || (with.0.position.x
                //         + (with.0.tile_size.x * with.0.scale) * with.0.transition_factor)
                //         - transform.translation.x
                //         < window.width()
                // {
                //     transform.translation.x = with.0.position.x;
                // } else {
                    transform.translation.x += with.0.speed.x * layer.initial_speed;
                // }
                //
                // if (with.0.position.y
                //     + (with.0.tile_size.y * with.0.scale) * with.0.transition_factor)
                //     - transform.translation.y
                //     > 0.
                //     || (with.0.position.y
                //         + (with.0.tile_size.y * with.0.scale) * with.0.transition_factor)
                //         - transform.translation.y
                //         < window.height()
                // {
                //     transform.translation.y = with.0.position.y;
                // } else {
                //     transform.translation.y += with.0.speed.y + layer.initial_speed;
                // }
            }
        }
        _ => {}
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
    parallax: Res<ParallaxResource>,
) {
    let cfg = assets.get(q.handle.clone()).unwrap();
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for (i, layer) in parallax.layers.iter().enumerate() {
        let mut layer_entity = commands.spawn();
        layer_entity
            .insert(Name::new(format!("Parallax Layer ({})", i)))
            .insert(Transform {
                translation: Vec3::new(0.0, 0.0, layer.z),
                scale: Vec3::new(layer.scale, layer.scale, 1.0),
                ..Default::default()
            })
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                let image: Handle<Image> = asset_server.load(&layer.path.clone());
                for x in 0..parallax.count {
                    let new_pos = layer.position + Vec2::new(x as f32 * layer.tile_size.x, 0.0);
                    parent
                        .spawn_bundle(SpriteBundle {
                            texture: image.clone(),
                            transform: Transform {
                                translation: Vec3::new(new_pos.x, new_pos.y, 0.0),
                                scale: Vec3::new(layer.scale, layer.scale, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(GlobalTransform::default())
                        .insert(ParallaxItem(layer.clone()));
                }
            });
    }

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

// pub fn move_camera_system(
//     mut move_event_writer: EventWriter<ParallaxMoveEvent>,
//     mut query: Query<(&mut Floor, &mut Transform)>,
// ) {
//     move_event_writer.send(ParallaxMoveEvent {
//         camera_move_speed: PLAYER_SPEED,
//     });
//
//     for (_floor, mut transform) in query.iter_mut() {
//         transform.translation.x += PLAYER_SPEED;
//     }
// }
