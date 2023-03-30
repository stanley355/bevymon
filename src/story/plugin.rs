use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use super::chapter_one::plugin::ChapterOnePlugin;
use super::state::StoryState;

#[derive(Debug)]
pub struct StoryPluginGroup;

impl PluginGroup for StoryPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StoryPlugin)
            .add(ChapterOnePlugin)
    }
}

#[derive(Debug)]
pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StoryState>();
    }
}
