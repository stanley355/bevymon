use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod new_game;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum StoryState {
    #[default]
    PreGame, //Splash screen and Menu
    NewGame,
    Intro,
}

#[derive(Debug)]
pub struct StoryPluginGroup;

impl PluginGroup for StoryPluginGroup{
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(new_game::NewGamePlugin)
    }
}
