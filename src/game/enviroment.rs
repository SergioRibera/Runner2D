#![allow(dead_code)]

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::AssetCollection;
// use bevy_parallax::{LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxResource};
use bevy_rapier2d::prelude::*;

use crate::{GameConfigAsset, GameConfigAssetHandler};

use super::{
    platform::draw_atlas,
    player::{PlayerCamera, PlayerSettings, PLAYER_SPEED},
    GameState,
};

const ENVIROMENT_WIDTH: f32 = 928.0;
const ENVIROMENT_HEIGHT: f32 = 793.0;

pub struct Enviroment;

#[derive(Component)]
pub struct Floor;

#[derive(AssetCollection, Resource)]
pub struct EnviromentAssets {
    #[asset(path = "audio/game_ambient.ogg")]
    pub background: Handle<AudioSource>,
    #[asset(path = "enviroment/Layer_0009_2.png")]
    pub layer_0: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 12, rows = 12))]
    #[asset(path = "enviroment/platforms.png")]
    pub platforms: Handle<TextureAtlas>,
}

// Parallax system
#[derive(Component)]
pub struct ParallaxItem(pub LayerData, pub Vec2);

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

#[derive(Resource)]
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
                    position: Vec2::new(-ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT / 4.2),
                    ..Default::default()
                },
                // Gray trees
                LayerData {
                    speed: Vec2::new(0.8, 0.0),
                    path: get_resource_name("05_5"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.2,
                    z: 1.0,
                    position: Vec2::new(-ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT / 4.0),
                    ..Default::default()
                },
                // More proximite trees
                LayerData {
                    speed: Vec2::new(0.7, 0.0),
                    path: get_resource_name("03_6"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.2,
                    z: 1.1,
                    position: Vec2::new(-ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT / 4.0),
                    ..Default::default()
                },
                // Top leaf
                LayerData {
                    speed: Vec2::new(0.6, 0.0),
                    path: get_resource_name("02_7"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.2,
                    z: 1.1,
                    position: Vec2::new(-ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT / 4.5),
                    ..Default::default()
                },
                // Floor of leaf
                LayerData {
                    speed: Vec2::new(0.5, 0.0),
                    path: get_resource_name("02_7"),
                    tile_size: Vec2::new(ENVIROMENT_WIDTH, ENVIROMENT_HEIGHT),
                    scale: 1.,
                    z: 2.0,
                    position: Vec2::new(-ENVIROMENT_WIDTH, -370.0),
                    ..Default::default()
                },
            ],
        })
        .add_systems((setup_enviroment, draw_atlas).in_schedule(OnEnter(GameState::MainMenu)))
        .add_system(move_parallax_system);
        // .add_system_set(SystemSet::on_update(GameState::InGame).with_system(move_parallax_system));
    }
}

fn move_parallax_system(
    windows: Query<(&Window, With<PrimaryWindow>)>,
    mut query: Query<(&mut Transform, &ParallaxItem)>,
    mut player_query: Query<(&mut PlayerSettings, &mut Transform), Without<ParallaxItem>>,
    game_state: Res<State<GameState>>,
    layer: Res<ParallaxResource>,
) {
    match game_state.0 {
        GameState::MainMenu | GameState::InGame => {
            let (window, _) = windows.get_single().unwrap();
            for (mut transform, parallax_item) in query.iter_mut() {
                if let Ok((_player, player_transform)) = player_query.get_single_mut() {
                    if player_transform.translation.x - transform.translation.x
                        + ((parallax_item.0.tile_size.x * parallax_item.0.scale) / 2.)
                        < -(window.width() * parallax_item.0.transition_factor)
                    {
                        transform.translation.x -= parallax_item.0.speed.x * layer.initial_speed;
                    } else if player_transform.translation.x - transform.translation.x
                        + ((parallax_item.0.tile_size.x * parallax_item.0.scale) / 2.)
                        > window.width() * parallax_item.0.transition_factor
                    {
                        transform.translation.x += parallax_item.0.speed.x * layer.initial_speed;
                    }

                    // if (parallax_item.0.position.x
                    //     + (parallax_item.0.tile_size.x * parallax_item.0.scale)
                    //         * parallax_item.0.transition_factor)
                    //     - transform.translation.x
                    //     > (parallax_item.0.transition_factor * parallax_item.0.tile_size.x)
                    // {
                    //     transform.translation.x = parallax_item.1.x * layer.count as f32;
                    // } else {
                    //     transform.translation.x -= parallax_item.0.speed.x * layer.initial_speed;
                    // }
                    //
                    // if (parallax_item.0.position.y
                    //     + (parallax_item.0.tile_size.y * parallax_item.0.scale)
                    //         * parallax_item.0.transition_factor)
                    //     - transform.translation.y
                    //     > 0.
                    //     || (parallax_item.0.position.y
                    //         + (parallax_item.0.tile_size.y * parallax_item.0.scale)
                    //             * parallax_item.0.transition_factor)
                    //         - transform.translation.y
                    //         < window.height()
                    // {
                    //     transform.translation.y = parallax_item.1.y * layer.count as f32;
                    // } else {
                    //     transform.translation.y -= parallax_item.0.speed.y + layer.initial_speed;
                    // }
                }
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
    windows: Query<(&Window, With<PrimaryWindow>)>,
    asset_server: ResMut<AssetServer>,
    cfg: ResMut<Assets<GameConfigAsset>>,
    cfg_handle: Res<GameConfigAssetHandler>,
    parallax: Res<ParallaxResource>,
) {
    let (window, _) = windows.get_single().unwrap();
    let cfg = cfg.get(&cfg_handle.0).unwrap();
    let mut cam2d = commands.spawn(Camera2dBundle::default());

    cam2d.insert(PlayerCamera {
        offset_x: -250.,
        offset_y: 0.,
    });

    for (i, layer) in parallax.layers.iter().enumerate() {
        commands
            .spawn_empty()
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
                        .spawn(SpriteBundle {
                            texture: image.clone(),
                            transform: Transform {
                                translation: Vec3::new(new_pos.x, new_pos.y, 0.0),
                                scale: Vec3::new(layer.scale, layer.scale, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(ParallaxItem(layer.clone(), new_pos.clone()));
                }
            });
    }

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("DebugPixel.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -(window.height() * cfg.floor_multiplier), 5.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Floor)
        .insert(Collider::cuboid(window.width(), 50.))
        .insert(RigidBody::Fixed);
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
