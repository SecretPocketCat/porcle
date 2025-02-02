//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::{NextTransitionedState, Screen};
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title)
        .add_systems(OnEnter(Screen::Exit), exit_app)
        .register_type::<TitleAction>()
        .add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    Tutorial,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.header("PORCLE");
            children.button("PLAY").insert(TitleAction::Play);
            children.button("TUTORIAL").insert(TitleAction::Tutorial);
            children.button("CREDITS").insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("EXIT").insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut next_screen: ResMut<NextTransitionedState>,
    mut button_query: InteractionQuery<&TitleAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Game),
                TitleAction::Tutorial => next_screen.set(Screen::Tutorial),
                TitleAction::Credits => next_screen.set(Screen::Credits),
                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => next_screen.set(Screen::Exit),
            }
        }
    }
}

fn exit_app(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
