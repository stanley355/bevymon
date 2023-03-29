use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod state;
mod menu;
mod splash;
mod menu_component;

// Enum that will be used as a global state for the game

#[derive(Debug)]
pub struct StartupPluginGroup;

impl PluginGroup for StartupPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(splash::SplashPlugin)
            .add(menu::MenuPlugin)
    }
}
