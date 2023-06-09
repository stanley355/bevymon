use bevy::prelude::*;

use crate::startup::state::StartupState;
use super::controller::SplashScreen;

#[derive(Debug)]
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StartupState>()
            .add_system(SplashScreen::new.in_schedule(OnEnter(StartupState::Splash)))
            .add_system(SplashScreen::countdown.in_set(OnUpdate(StartupState::Splash)))
            .add_system(SplashScreen::despawn.in_schedule(OnExit(StartupState::Splash)));
    }
}