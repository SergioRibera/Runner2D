#![allow(dead_code)]
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{GameConfigAsset, GameConfigAssetHandler};

use super::{GameSettings, GameState};

pub const PLAYER_SPEED: f32 = 3.0;
pub const PLAYER_JUMP_FORCE: f32 = 30.0;

const SPRITE_SIZE: f32 = 150.0;

const SATURATION_DESELECTED: f32 = 0.3;
const LIGHTNESS_DESELECTED: f32 = 0.2;
const SATURATION_SELECTED: f32 = 0.9;
const LIGHTNESS_SELECTED: f32 = 0.7;
const ALPHA: f32 = 0.92;

const SHOWCASE_TIMER_SECS: f32 = 3.0;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Pause,
    Jump,
    MoveLeft,
    MoveRight,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "player/2BlueWizardIdle/Chara - BlueIdle00001.png")]
    player: Handle<Image>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            startup_player
                .run_if(run_once())
                .in_schedule(OnEnter(GameState::InGame)),
        )
        .add_system(player_movement);
    }
}

#[derive(Component, Debug)]
pub struct PlayerSettings;

#[derive(Component, Debug)]
pub struct PlayerCamera {
    pub offset_x: f32,
    pub offset_y: f32,
}

fn get_resource_name(name: &str) -> String {
    format!("player/2BlueWizardIdle/Chara - BlueIdle000{}.png", name)
}

fn startup_player(
    mut commands: Commands,
    windows: Query<(&Window, With<PrimaryWindow>)>,
    cfg: Res<Assets<GameConfigAsset>>,
    cfg_handle: Res<GameConfigAssetHandler>,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let cfg = cfg.get(&cfg_handle.0).unwrap();
    let (window, _) = windows.get_single().unwrap();
    let intit_player_pos_x = -(window.width() * cfg.player_initial_pos_x);
    rapier_config.gravity = Vec2::new(0.0, -9.81 * cfg.gravity_multiplier);

    let mut input_map = InputMap::default();
    input_map.insert(KeyCode::Escape, PlayerAction::Pause);
    input_map.insert(GamepadButtonType::Select, PlayerAction::Pause);
    // Move to left
    input_map.insert(KeyCode::A, PlayerAction::MoveLeft);
    input_map.insert(KeyCode::Left, PlayerAction::MoveLeft);
    input_map.insert(GamepadButtonType::East, PlayerAction::MoveLeft);
    // Move to right
    input_map.insert(KeyCode::D, PlayerAction::MoveRight);
    input_map.insert(KeyCode::Right, PlayerAction::MoveRight);
    input_map.insert(GamepadButtonType::West, PlayerAction::MoveRight);
    // Jump
    input_map.insert(KeyCode::Space, PlayerAction::Jump);
    input_map.insert(GamepadButtonType::South, PlayerAction::Jump);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(get_resource_name("01").as_str()),
            transform: Transform {
                translation: Vec3::new(intit_player_pos_x, 0., 1.7),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(cfg.player_size_x, cfg.player_size_y)),
                ..default()
            },
            ..default()
        })
        .insert(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        })
        .insert(PlayerSettings)
        .insert(Collider::cuboid(
            cfg.player_box_size_x,
            cfg.player_box_size_y,
        ))
        .insert(RigidBody::Dynamic);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("DebugPixel.png"),
            transform: Transform {
                translation: Vec3::new(intit_player_pos_x, -200., 1.7),
                ..Default::default()
            },
            ..default()
        })
        .insert(Collider::cuboid(200., 50.))
        .insert(RigidBody::Fixed);
}

fn player_movement(
    input: Query<&ActionState<PlayerAction>, With<PlayerSettings>>,
    mut query: Query<(&mut PlayerSettings, &mut Sprite, &mut Transform)>,
    mut camera_query: Query<(&mut Transform, &mut PlayerCamera), Without<PlayerSettings>>,
    game_state: Res<State<GameState>>,
    mut set_game_state: ResMut<NextState<GameState>>,
    // mut camera: Query<(&Camera, &mut Transform)>,
) {
    if let Ok(action) = input.get_single() {
        if action.just_pressed(PlayerAction::Pause) {
            set_game_state.set(GameState::MainMenu);
            return;
        }
        for (_player, mut _sprite, mut transform) in query.iter_mut() {
            if game_state.0.eq(&GameState::InGame) {
                if action.pressed(PlayerAction::MoveLeft) {
                    transform.translation.x -= PLAYER_SPEED;
                }
                if action.pressed(PlayerAction::MoveRight) {
                    transform.translation.x += PLAYER_SPEED;
                }
                if action.just_pressed(PlayerAction::Jump) {
                    transform.translation.y += PLAYER_JUMP_FORCE;
                }
                transform.translation.x += PLAYER_SPEED;
            }

            if let Ok((mut camera_transform, camera_settings)) = camera_query.get_single_mut() {
                let mut pos_offset = camera_transform.translation - transform.translation;
                pos_offset -= Vec3::new(camera_settings.offset_x, camera_settings.offset_y, 9.);
                pos_offset.z = 999.9;
                // camera_transform.translation = pos_offset;
            }
        }
    }
}
