use bevy::prelude::*;

use super::StoryState;
use crate::miscellanous::textbox::TextBox;

#[derive(Debug)]
pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(NewGame::start.in_schedule(OnEnter(StoryState::NewGame)));
    }
}

#[derive(Debug, Component)]
pub struct NewGame;

impl NewGame {
    fn start(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
        window_query: Query<&Window>,
    ) {
        let window = window_query.single();
        let textbox = TextBox::bundle(asset_server, texture_atlas_res, window);
        commands.spawn(textbox);
    }
}
