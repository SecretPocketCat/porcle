//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::{NextTransitionedState, Screen};
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Tutorial), enter);
    app.add_systems(Update, handle_action.run_if(in_state(Screen::Tutorial)));
    app.register_type::<TutorialAction>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TutorialAction {
    Play,
}

fn enter(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Tutorial))
        .with_children(|children| {
            children.header("HOW TO PLAY");
            children.label("Use your good ol' trusty Porcle to protect the core as long as you can!");
            children.label("Enemies will swarm you from all angles, so be sure make sure both of your gun and your saw blade & keep it going.");
            children.header("MOUSE CONTROLS");
            children.label("Move: aim ship");
            children.label("LMB: Shoot");
            children.label("RMB: Capture & release sawblade");
            children.label("R: Restart");
            children.label("ESC: Back to menu");
            children.header("GAMEPAD CONTROLS");
            children.label("LEFT or RIGHT STICK: aim ship");
            children.label("RIGHT BUMPER or CROSS/A: Shoot");
            children.label("LEFT BUMPER or SQUARE/X: Capture & release sawblade");
            children.header("SHIP CYCLE CONTROLS");
            children.label("Cycle 2x clockwise: recall sawblade");
            children.label("Cycle 1x counter-clockwise: gain ammo based on current sawblade speed");
           
            children.button("PLAY").insert(TutorialAction::Play);            
        });
}

fn handle_action(
    mut next_screen: ResMut<NextTransitionedState>,
    mut button_query: InteractionQuery<&TutorialAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TutorialAction::Play => next_screen.set(Screen::Game),
            }
        }
    }
}
