#![allow(dead_code)]
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::TextColorLens, Animator, EaseFunction, Lens, Tween, TweenCompleted, TweeningType,
};

use super::GameState;

#[derive(Component)]
pub struct SplashProgress;
#[derive(Component)]
pub struct UIElement;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UIColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}
impl Lens<UiColor> for UIColorLens {
    fn lerp(&mut self, target: &mut UiColor, ratio: f32) {
        // Note: Add<f32> for Color affects alpha, but not Mul<f32>. So use Vec4 for consistency.
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.0.set_r(value.x);
        target.0.set_g(value.y);
        target.0.set_b(value.z);
        target.0.set_a(value.w);
    }
}

pub fn load_splash(mut commands: Commands, fonts: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        TweeningType::PingPong,
        Duration::from_secs(3),
        TextColorLens {
            start: Color::rgba(0.0, 0.0, 0.0, 0.0),
            end: Color::WHITE,
            section: 0,
        },
    )
    .with_completed_event(true, 2);

    commands
        // This is where we're going to define the layout of the main menu.
        .spawn_bundle(TextBundle {
            style: Style {
                position: Rect {
                    left: Val::Percent(32.),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Auto,
                },
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            text: Text::with_section(
                "Sergio Ribera",
                TextStyle {
                    // font: fonts.pixel_font.clone(),
                    font: fonts.load("fonts/pixel_font.ttf"),
                    font_size: 76.,
                    color: Color::rgba(0., 0., 0., 0.),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(UIElement)
        .insert(Animator::new(tween));
}

pub fn on_splash(
    mut commands: Commands,
    quey_text: Query<&Animator<Text>, With<UIElement>>,
    entity_text: Query<Entity, With<UIElement>>,
    mut game_state: ResMut<State<GameState>>,
) {
    match game_state.current() {
        GameState::Splash => {
            // if let Ok(anim_logo) = query_anim.get_single() {
            //     if let Some(tween_logo) = anim_logo.tweenable() {
            //         let progress = tween_logo.progress();
            //         println!("{:5.1}%", progress * 100.);
            //     }
            // }
            if let Ok(anim_text) = quey_text.get_single() {
                if let Some(tween_text) = anim_text.tweenable() {
                    if tween_text.progress() >= 0.8 {
                        game_state.set(GameState::MainMenu).unwrap();
                        println!("Splash completed 1");
                        if let Ok(entity) = entity_text.get_single() {
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }
        }
        _ => return,
    }
}
