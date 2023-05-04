use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{EaseFunction, SplashAssetType, SplashItem, SplashPlugin, SplashScreen};

use super::GameState;

pub fn load_splash() -> impl Plugin {
    SplashPlugin::new(GameState::Splash, GameState::MainMenu, false)
        .add_screen(SplashScreen {
            brands: vec![SplashItem {
                asset: SplashAssetType::SingleText(
                    Text::from_sections([
                        TextSection::new(
                            "Sergio Ribera\n",
                            TextStyle {
                                font_size: 76.,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "presents\n",
                            TextStyle {
                                font_size: 38.,
                                ..default()
                            },
                        ),
                    ])
                    .with_alignment(TextAlignment::Center),
                    "fonts/pixel_font.ttf".to_string(),
                ),
                tint: Color::WHITE,
                size: Size::new(Val::Percent(40.), Val::Px(80.)),
                ease_function: EaseFunction::QuarticInOut.into(),
                duration: Duration::from_secs(5),
                is_static: false,
            }],
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .add_screen(SplashScreen {
            brands: vec![SplashItem {
                asset: SplashAssetType::SingleText(
                    Text::from_section(
                        "Runner\n",
                        TextStyle {
                            font_size: 150.,
                            ..default()
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    "fonts/pixel_font.ttf".to_string(),
                ),
                tint: Color::WHITE,
                size: Size::new(Val::Percent(35.), Val::Px(160.)),
                ease_function: EaseFunction::QuarticInOut.into(),
                duration: Duration::from_secs(5),
                is_static: false,
            }],
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
}
