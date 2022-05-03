use bevy::core::Timer;
use bevy::prelude::*;

use super::enviroment::EnviromentAssets;

pub struct PlatformData {
    pub interval: Timer,
}

pub fn draw_atlas(mut commands: Commands, windows: Res<Windows>, assets: Res<EnviromentAssets>) {
    let window = windows.get_primary().unwrap();
    let intit_player_pos_x = -(window.width() * 0.35);
    let atlas = assets.platforms.clone();
    // draw single texture from sprite sheet starting at index 0
    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(intit_player_pos_x, 0., 1.6),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: atlas,
        ..Default::default()
    });
}
