use bevy::prelude::*;

use super::chapter_one::state::ChapterOneState;

#[derive(Debug)]
pub struct Story;

impl Story {
    pub fn new(mut chapter_state: ResMut<NextState<ChapterOneState>>) {
        chapter_state.set(ChapterOneState::BackgroundScene);
    }
}
