//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::game::{
    // assets::SoundtrackKey,
    audio::soundtrack::PlaySoundtrack,
    spawn::level::SpawnLevel,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Game), enter_playing);
    app.add_systems(OnExit(Screen::Game), exit_playing);
    app.add_systems(OnEnter(Screen::RestartGame), enter_restart);

    app.add_systems(
        Update,
        (
            return_to_title_screen
                .run_if(in_state(Screen::Game).and_then(input_just_pressed(KeyCode::Escape))),
            restart_game.run_if(in_state(Screen::Game).and_then(input_just_pressed(KeyCode::KeyR))),
        ),
    );
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    // commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn restart_game(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::RestartGame);
}

fn enter_restart(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Game);
}
