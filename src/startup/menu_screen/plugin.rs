use bevy::prelude::*;

use super::controller::MenuScreen;
use crate::startup::state::StartupState;

#[derive(Debug)]
pub struct MenuScreenPlugin;

impl Plugin for MenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(MenuScreen::new.in_schedule(OnEnter(StartupState::Menu)))
            .add_system(MenuScreen::enter_game.in_set(OnUpdate(StartupState::Menu)))
            .add_system(MenuScreen::cleanup.in_schedule(OnExit(StartupState::Menu)));
    }
}