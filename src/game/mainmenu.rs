#![allow(clippy::type_complexity)]
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TextColorLens, Animator, EaseFunction, RepeatStrategy, Tween};

use crate::GlobalUIAssets;

use super::{
    transition::{hide_text, show_text, TransitionElement},
    GameState,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MainMenu;

#[derive(Component)]
struct MainMenuButton {
    action: MainMenuState,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
enum MainMenuState {
    #[default]
    Main,
    Play,
    Options,
    Credits,
    Quit,
}

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_systems((build_main_menu, show_text).in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(show_text.in_schedule(OnEnter(MainMenuState::Main)))
            .add_system(button_system.run_if(in_state(GameState::MainMenu)))
            .add_system(hide_text.in_schedule(OnExit(GameState::MainMenu)))
            // Submenu Options
            .add_system(show_text.in_schedule(OnEnter(MainMenuState::Options)))
            .add_system(hide_text.in_schedule(OnExit(MainMenuState::Options)))
            // Submenu Credits
            .add_system(show_text.in_schedule(OnEnter(MainMenuState::Credits)))
            .add_system(hide_text.in_schedule(OnExit(MainMenuState::Credits)));
    }
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<(&Text, &mut Visibility)>,
    mut btn_query: Query<(&Text, &MainMenuButton)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        let (_text, mut visibility) = text_query.get_mut(children[0]).unwrap();
        let (_text, btn) = btn_query.get_mut(children[1]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                match btn.action {
                    MainMenuState::Play => {
                        game_state.set(GameState::InGame);
                    }
                    MainMenuState::Options => {}
                    MainMenuState::Credits => {}
                    MainMenuState::Quit => {
                        std::process::exit(0);
                    }
                    // Return to main menu
                    MainMenuState::Main => {}
                }
            }
            Interaction::Hovered => {
                *visibility = Visibility::Visible;
            }
            Interaction::None => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn build_main_menu(mut commands: Commands, font_assets: Res<GlobalUIAssets>) {
    let text_buttons = vec![
        ("Play", MainMenuState::Play),
        ("Options", MainMenuState::Options),
        ("Credits", MainMenuState::Credits),
        ("Quit", MainMenuState::Quit),
    ];
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position: UiRect {
                    top: Val::Percent(0.),
                    left: Val::Percent(0.),
                    ..default()
                },
                ..Default::default()
            },
            // color: UiColor(Color::rgba(0., 0., 0., 0.)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        position: UiRect {
                            bottom: Val::Percent(6.),
                            left: Val::Percent(13.),
                            ..default()
                        },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Bevy Runner",
                        TextStyle {
                            font: font_assets.pixel_font.clone(),
                            font_size: 62.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                })
                .insert(TransitionElement {
                    color_target: Color::WHITE,
                    ..default()
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        margin: UiRect {
                            left: Val::Percent(65.),
                            ..default()
                        },
                        position: UiRect {
                            top: Val::Percent(6.),
                            ..default()
                        },
                        ..Default::default()
                    },
                    ..default()
                })
                .with_children(|node_parent| {
                    for btn in text_buttons {
                        build_btn(node_parent, font_assets.pixel_font.clone(), btn.0, btn.1);
                    }
                });
        });
}

fn build_credits_menu(commands: &mut Commands, font_assets: &Res<GlobalUIAssets>) {
    let credits = vec![
        ("Programmer", vec!["Sergio Ribera"]),
        ("Artist", vec!["Sergio Ribera"]),
        ("Music", vec!["Sergio Ribera"]),
        ("Sound Effects", vec!["Sergio Ribera"]),
    ];
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position: UiRect {
                    top: Val::Percent(0.),
                    left: Val::Percent(0.),
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            build_btn(
                parent,
                font_assets.pixel_font.clone(),
                "Back",
                MainMenuState::Main,
            );

            for (title, content) in credits {
                parent.spawn(TextBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        position: UiRect {
                            bottom: Val::Percent(6.),
                            left: Val::Percent(13.),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::from_section(
                        title,
                        TextStyle {
                            font: font_assets.pixel_font.clone(),
                            font_size: 62.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                });

                for text in content {
                    parent.spawn(TextBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            position: UiRect {
                                top: Val::Percent(0.),
                                left: Val::Percent(0.),
                                ..default()
                            },
                            ..default()
                        },
                        text: Text::from_section(
                            text,
                            TextStyle {
                                font: font_assets.pixel_font.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        ..default()
                    });
                }
            }
        });
}

fn build_btn(parent: &mut ChildBuilder, font: Handle<Font>, text: &str, action: MainMenuState) {
    parent
        .spawn(ButtonBundle::default())
        .with_children(|btn_parent| {
            let tween = Tween::new(
                EaseFunction::CubicInOut,
                Duration::from_millis(500),
                TextColorLens {
                    start: Color::rgba(0.0, 0.0, 0.0, 0.0),
                    end: Color::WHITE,
                    section: 0,
                },
            )
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

            btn_parent
                .spawn(TextBundle {
                    style: Style {
                        position: UiRect {
                            top: Val::Percent(0.5),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::from_section(
                        ">",
                        TextStyle {
                            font: font.clone(),
                            font_size: 48.,
                            color: Color::rgba(0., 0., 0., 0.),
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(TransitionElement {
                    color_target: Color::WHITE,
                    ..default()
                })
                .insert(Animator::new(tween));

            // Instancing text content of button
            btn_parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font,
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                })
                .insert(TransitionElement {
                    color_target: Color::WHITE,
                    ..default()
                })
                .insert(MainMenuButton { action });
        });
}
