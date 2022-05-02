use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::TextColorLens, Animator, EaseFunction, Lens, Tween, TweenCompleted, TweeningType,
};

use crate::GlobalUIAssets;

use super::GameState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MainMenu;

#[derive(Debug, Hash, Eq, PartialEq)]
enum MainMenuState {
    Main,
    Options,
    Credits,
    Quit,
}

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_state(&MainMenuState::Main)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_ui))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(button_system));
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    // for (interaction, mut color, children) in interaction_query.iter_mut() {
    //     let mut text = text_query.get_mut(children[0]).unwrap();
    //     match *interaction {
    //         Interaction::Clicked => {
    //             text.sections[0].value = "Press".to_string();
    //             *color = PRESSED_BUTTON.into();
    //         }
    //         Interaction::Hovered => {
    //             text.sections[0].value = "Hover".to_string();
    //             *color = HOVERED_BUTTON.into();
    //         }
    //         Interaction::None => {
    //             text.sections[0].value = "Button".to_string();
    //             *color = NORMAL_BUTTON.into();
    //         }
    //     }
    // }
}

fn setup_ui(mut commands: Commands, font_assets: Res<GlobalUIAssets>) {
    let text_buttons = vec![
        ("Play", MainMenuState::Main),
        ("Options", MainMenuState::Options),
        ("Credits", MainMenuState::Credits),
        ("Quit", MainMenuState::Quit),
    ];

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position: Rect {
                    top: Val::Percent(0.),
                    left: Val::Percent(0.),
                    right: Val::Auto,
                    bottom: Val::Auto,
                    ..default()
                },
                ..Default::default()
            },
            color: UiColor(Color::rgba(0., 0., 0., 0.)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                    position: Rect {
                        bottom: Val::Percent(6.),
                        left: Val::Percent(13.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Bevy Runner",
                    TextStyle {
                        font: font_assets.pixel_font.clone(),
                        font_size: 62.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..default()
            });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        margin: Rect {
                            left: Val::Percent(65.),
                            ..Default::default()
                        },
                        position: Rect {
                            top: Val::Percent(6.),
                            ..default()
                        },
                        ..Default::default()
                    },
                    color: UiColor(Color::rgba(0., 0., 0., 0.)),
                    ..default()
                })
                .with_children(|node_parent| {
                    for text in text_buttons {
                        node_parent
                            .spawn_bundle(ButtonBundle {
                                color: UiColor(Color::rgba(0., 0., 0., 0.)),
                                ..default()
                            })
                            .with_children(|btn_parent| {
                                // TODO: Add button signal selected (>)
                                // add tweener to ping pong the transparency
                                let tween = Tween::new(
                                    EaseFunction::CubicInOut,
                                    TweeningType::PingPong,
                                    Duration::from_millis(500),
                                    TextColorLens {
                                        start: Color::rgba(0.0, 0.0, 0.0, 0.0),
                                        end: Color::WHITE,
                                        section: 0,
                                    },
                                );

                                btn_parent
                                    .spawn_bundle(TextBundle {
                                        style: Style {
                                            position: Rect {
                                                top: Val::Percent(0.5),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                        text: Text::with_section(
                                            ">",
                                            TextStyle {
                                                font: font_assets.pixel_font.clone(),
                                                font_size: 48.,
                                                color: Color::rgba(0., 0., 0., 0.),
                                            },
                                            TextAlignment {
                                                vertical: VerticalAlign::Center,
                                                horizontal: HorizontalAlign::Center,
                                            },
                                        ),
                                        visibility: Visibility { is_visible: false },
                                        ..Default::default()
                                    })
                                    .insert(Animator::new(tween));

                                // Instancing text content of button
                                btn_parent.spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        text.0,
                                        TextStyle {
                                            font: font_assets.pixel_font.clone(),
                                            font_size: 32.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    ..Default::default()
                                });
                            });
                    }
                });
        });
}

fn update_ui(mut commands: Commands) {}
