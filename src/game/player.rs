#![allow(dead_code)]
use bevy::prelude::*;

use heron::prelude::*;

pub const PLAYER_SPEED: f32 = 3.0;
pub const PLAYER_JUMP_FORCE: f32 = 700.0;

const SPRITE_SIZE: f32 = 150.0;

const SATURATION_DESELECTED: f32 = 0.3;
const LIGHTNESS_DESELECTED: f32 = 0.2;
const SATURATION_SELECTED: f32 = 0.9;
const LIGHTNESS_SELECTED: f32 = 0.7;
const ALPHA: f32 = 0.92;

const SHOWCASE_TIMER_SECS: f32 = 3.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(startup_player)
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
    asset_server: Res<AssetServer>
) {
    let window = windows.get_primary().unwrap();

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(get_resource_name("01").as_str()),
            transform: Transform {
                translation: Vec3::new(- (window.width() * 0.75), 0.0, 1.7),
                ..Default::default()

            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(170.0, 170.0)),
                ..default()
            },
            ..default()
        })
        .insert(PlayerSettings)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(20.0, 45.0).extend(0.0),
            border_radius: None,
        })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Dynamic);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerSettings, &mut Sprite, &mut Transform)>,
    // mut camera: Query<(&Camera, &mut Transform)>,
) {
    for (_player, mut _sprite, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            transform.translation.y *= PLAYER_JUMP_FORCE;
        }
        transform.translation.x += PLAYER_SPEED;
    }
}
