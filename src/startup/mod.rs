use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod menu;
mod splash;
mod menu_component;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum StartupState {
    #[default]
    Splash,
    Menu,
    InGame
}

#[derive(Debug)]
pub struct StartupPluginGroup;

impl PluginGroup for StartupPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(splash::SplashPlugin)
            .add(menu::MenuPlugin)
    }
}
