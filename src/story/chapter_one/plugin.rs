use bevy::prelude::*;

use super::state::ChapterOneState;
use super::opening_scene::plugin::OpeningScenePlugin;

#[derive(Debug)]
pub struct ChapterOnePlugin;

impl Plugin for ChapterOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ChapterOneState>()
            .add_plugin(OpeningScenePlugin);
    }
}
