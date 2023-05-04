use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TextColorLens, Animator, EaseFunction, Tween};

const COLOR_TWEEN_DURATION: u64 = 500;

#[derive(Component, Default, Clone)]
pub struct TransitionElement {
    pub show: bool,
    pub color_target: Color,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum TransitionStateText {
    Show,
    #[default]
    Idle,
    Hide,
}

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<TransitionStateText>()
            .add_system(transition_update);
    }
}

pub fn hide_text(
    mut commands: Commands,
    mut elements: Query<(Entity, &Text, With<TransitionElement>)>,
    mut transition_state: ResMut<NextState<TransitionStateText>>,
) {
    for (entity, text, _) in elements.iter_mut() {
        let tween_text = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(COLOR_TWEEN_DURATION),
            TextColorLens {
                start: text.sections[0].style.color,
                end: Color::rgba(0., 0., 0., 0.),
                section: 0,
            },
        )
        .with_completed_event(99);
        commands.entity(entity).remove::<Animator<Text>>();
        commands.entity(entity).insert(Animator::new(tween_text));
    }
    transition_state.set(TransitionStateText::Hide);
}

pub fn show_text(
    mut commands: Commands,
    mut elements: Query<(Entity, &Text, &TransitionElement)>,
    mut transition_state: ResMut<NextState<TransitionStateText>>,
) {
    for (entity, _text, t) in elements.iter_mut() {
        let tween_text = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(COLOR_TWEEN_DURATION),
            TextColorLens {
                start: Color::rgba(0., 0., 0., 0.),
                end: t.color_target,
                section: 0,
            },
        )
        .with_completed_event(99);
        commands.entity(entity).remove::<Animator<Text>>();
        commands.entity(entity).insert(Animator::new(tween_text));
    }
    transition_state.set(TransitionStateText::Show);
}

fn visibility_from_bool(v: bool) -> Visibility {
    if v {
        return Visibility::Visible;
    }
    Visibility::Hidden
}

pub fn transition_update(
    mut query_text: Query<(&Animator<Text>, &mut Visibility, &mut TransitionElement)>,
    transition_state: Res<State<TransitionStateText>>,
    mut set_transition_state: ResMut<NextState<TransitionStateText>>,
) {
    match transition_state.0 {
        TransitionStateText::Show => {
            for (animator, mut visibility, mut t) in query_text.iter_mut() {
                // let progress = animator.progress();
                // if progress <= 0.01 {
                //     t.show = true;
                //     *visibility = visibility_from_bool(t.show);
                // }
            }
            set_transition_state.set(TransitionStateText::Idle);
        }
        TransitionStateText::Hide => {
            for (animator, mut visibility, mut t) in query_text.iter_mut() {
                // let progress = animator.progress();
                // if progress >= 0.99 {
                //     t.show = false;
                //     *visibility = visibility_from_bool(t.show);
                // }
            }
            set_transition_state.set(TransitionStateText::Idle);
        }
        TransitionStateText::Idle => {}
    }
}
