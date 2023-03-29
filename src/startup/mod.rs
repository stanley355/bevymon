use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod state;
mod menu_screen;
mod splash;

#[derive(Debug)]
pub struct StartupPluginGroup;

impl PluginGroup for StartupPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(splash::plugin::SplashPlugin)
            .add(menu_screen::plugin::MenuScreenPlugin)
    }
}
