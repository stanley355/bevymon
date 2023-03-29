use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod textbox;

#[derive(Debug)]
pub struct MiscellanousPluginGroup;

impl PluginGroup for MiscellanousPluginGroup{
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(textbox::TextboxPlugin)
    }
}
