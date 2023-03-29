use bevy::prelude::*;

#[derive(Debug)]
pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(NewGame::start);
        // app.add_system(NewGame::start.in_schedule(OnEnter(StoryState::NewGame)));
    }
}

#[derive(Debug, Component)]
pub struct NewGame;

impl NewGame {
    
}
