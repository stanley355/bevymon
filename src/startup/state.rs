use bevy::prelude::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum StartupState {
    #[default]
    Splash,
    Menu,
    InGame
}
