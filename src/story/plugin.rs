use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use super::chapter_one::plugin::ChapterOnePlugin;

#[derive(Debug)]
pub struct StoryPluginGroup;

impl PluginGroup for StoryPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(ChapterOnePlugin)
    }
}
