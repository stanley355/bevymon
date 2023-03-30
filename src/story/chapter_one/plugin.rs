use bevy::prelude::*;

use super::scene::BackgroundScene;
use super::state::{BgSceneState, ChapterOneState};

#[derive(Debug)]
pub struct ChapterOnePlugin;

impl Plugin for ChapterOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ChapterOneState>()
            .add_state::<BgSceneState>()
            .add_system(BackgroundScene::new.in_schedule(OnEnter(ChapterOneState::BackgroundScene)))
            .add_system(
                BackgroundScene::show_img_detection
                    .in_set(OnUpdate(ChapterOneState::BackgroundScene)),
            )
            .add_system(
                BackgroundScene::spawn_dialogue_img.in_schedule(OnEnter(BgSceneState::EggImg)),
            );
    }
}
