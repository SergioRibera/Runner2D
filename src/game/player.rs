#![allow(dead_code)]
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use heron::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{GameSettings, GameState};

pub const PLAYER_SPEED: f32 = 3.0;
pub const PLAYER_JUMP_FORCE: f32 = 700.0;

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

#[derive(AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "player/2BlueWizardIdle/Chara - BlueIdle00001.png")]
    player: Handle<Image>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(startup_player),
        )
        .add_system(player_movement);
    }
}

#[derive(Component, Debug)]
pub struct PlayerSettings;

fn get_resource_name(name: &str) -> String {
    format!("player/2BlueWizardIdle/Chara - BlueIdle000{}.png", name)
}

fn startup_player(
    mut commands: Commands,
    windows: Res<Windows>,
    game_cfg: Res<GameSettings>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary().unwrap();

    let intit_player_pos_x = -(window.width() * 0.35);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(get_resource_name("01").as_str()),
            transform: Transform {
                translation: Vec3::new(intit_player_pos_x, 0., 1.7),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(170., 170.)),
                ..default()
            },
            ..default()
        })
        .insert_bundle(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: game_cfg.player_ctrl.clone(),
        })
        .insert(PlayerSettings)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(20., 45.).extend(0.),
            border_radius: None,
        })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Dynamic);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("DebugPixel.png"),
            transform: Transform {
                translation: Vec3::new(intit_player_pos_x, -200., 1.7),
                ..Default::default()
            },
            ..default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(200., 50.).extend(0.),
            border_radius: None,
        })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Static);
}

fn player_movement(
    input: Query<&ActionState<PlayerAction>, With<PlayerSettings>>,
    mut query: Query<(&mut PlayerSettings, &mut Sprite, &mut Transform)>,
    mut game_state: ResMut<State<GameState>>,
    // mut camera: Query<(&Camera, &mut Transform)>,
) {
    if let Ok(action) = input.get_single() {
        if action.just_pressed(PlayerAction::Pause) {
            game_state.set(GameState::MainMenu).unwrap();
            return;
        }
        for (_player, mut _sprite, mut transform) in query.iter_mut() {
            if game_state.current().eq(&GameState::InGame) {
                if action.pressed(PlayerAction::MoveLeft) {
                    transform.translation.x -= PLAYER_SPEED;
                }
                if action.pressed(PlayerAction::MoveRight) {
                    transform.translation.x += PLAYER_SPEED;
                }
                if action.just_pressed(PlayerAction::Jump) {
                    transform.translation.y *= PLAYER_JUMP_FORCE;
                }
                transform.translation.x += PLAYER_SPEED;
            }
        }
    }
}
