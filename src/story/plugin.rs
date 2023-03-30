use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Debug)]
pub struct StoryPluginGroup;

impl PluginGroup for StoryPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
