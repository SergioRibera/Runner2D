use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TextColorLens, Animator, EaseFunction, Tween, TweeningType};

use crate::GlobalUIAssets;

use super::{
    splash::UIElement,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MainMenuState {
    Main,
    Play,
    Options,
    Credits,
    Quit,
}

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_state(MainMenuState::Main)
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(setup_ui)
                    .with_system(show_text),
            )
            .add_system_set(SystemSet::on_enter(MainMenuState::Main).with_system(show_text))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(button_system))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(hide_text))
            // Submenu Options
            .add_system_set(SystemSet::on_enter(MainMenuState::Options).with_system(show_text))
            .add_system_set(SystemSet::on_exit(MainMenuState::Options).with_system(hide_text))
            // Submenu Credits
            .add_system_set(SystemSet::on_enter(MainMenuState::Credits).with_system(show_text))
            .add_system_set(SystemSet::on_exit(MainMenuState::Credits).with_system(hide_text));
    }
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<(&Text, &mut Visibility)>,
    mut btn_query: Query<(&Text, &MainMenuButton)>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        let (_text, mut visibility) = text_query.get_mut(children[0]).unwrap();
        let (_text, btn) = btn_query.get_mut(children[1]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                match btn.action {
                    MainMenuState::Play => {
                        game_state.set(GameState::InGame).unwrap();
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
                visibility.is_visible = true;
            }
            Interaction::None => {
                visibility.is_visible = false;
            }
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    font_assets: Res<GlobalUIAssets>,
    entity_text: Query<Entity, With<UIElement>>,
) {
    if let Ok(entity) = entity_text.get_single() {
        commands.entity(entity).despawn();
    }

    // build_credits_menu(&mut commands, &font_assets);
    build_main_menu(commands, font_assets);
}

fn build_main_menu(mut commands: Commands, font_assets: Res<GlobalUIAssets>) {
    let text_buttons = vec![
        ("Play", MainMenuState::Play),
        ("Options", MainMenuState::Options),
        ("Credits", MainMenuState::Credits),
        ("Quit", MainMenuState::Quit),
    ];
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
            parent
                .spawn_bundle(TextBundle {
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
                })
                .insert(TransitionElement {
                    color_target: Color::WHITE,
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
            build_btn(
                parent,
                font_assets.pixel_font.clone(),
                "Back",
                MainMenuState::Main,
            );

            for (title, content) in credits {
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
                        title,
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

                for text in content {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            position: Rect {
                                top: Val::Percent(0.),
                                left: Val::Percent(0.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            text,
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
                        ..default()
                    });
                }
            }
        });
}

fn build_btn(parent: &mut ChildBuilder, font: Handle<Font>, text: &str, action: MainMenuState) {
    parent
        .spawn_bundle(ButtonBundle {
            color: UiColor(Color::rgba(0., 0., 0., 0.)),
            ..default()
        })
        .with_children(|btn_parent| {
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
                            font: font.clone(),
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
                .insert(TransitionElement {
                    color_target: Color::WHITE,
                    ..default()
                })
                .insert(Animator::new(tween));

            // Instancing text content of button
            btn_parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        text,
                        TextStyle {
                            font,
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(TransitionElement {
                    color_target: Color::WHITE,
                    ..default()
                })
                .insert(MainMenuButton { action });
        });
}
