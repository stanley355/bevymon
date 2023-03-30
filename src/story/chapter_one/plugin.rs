use bevy::prelude::*;

use super::state::ChapterOneState;

#[derive(Debug)]
pub struct ChapterOnePlugin;

impl Plugin for ChapterOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ChapterOneState>();
    }
}
