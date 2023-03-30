use bevy::prelude::*;

use super::scene::BackgroundScene;
use super::state::ChapterOneState;

#[derive(Debug)]
pub struct ChapterOnePlugin;

impl Plugin for ChapterOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ChapterOneState>()
            .add_system(BackgroundScene::new.in_schedule(OnEnter(ChapterOneState::BackgroundScene)));
    }
}
