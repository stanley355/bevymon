use bevy::prelude::*;

use super::controller::OpeningScene;
use super::state::OpeningSceneState;
use crate::story::chapter_one::state::ChapterOneState;

#[derive(Debug)]
pub struct OpeningScenePlugin;

impl Plugin for OpeningScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<OpeningSceneState>()
            .add_system(OpeningScene::new.in_schedule(OnEnter(ChapterOneState::OpeningScene)))
            .add_system(
                OpeningScene::spawn_img_detection.in_set(OnUpdate(ChapterOneState::OpeningScene)),
            )
            .add_system(
                OpeningScene::spawn_dialogue_img.in_schedule(OnEnter(OpeningSceneState::EggImg)),
            )
            .add_system(
                OpeningScene::spawn_dialogue_img.in_schedule(OnEnter(OpeningSceneState::MewImg)),
            )
            .add_system(
                OpeningScene::spawn_dialogue_img.in_schedule(OnEnter(OpeningSceneState::ArceusImg)),
            )
            .add_system(
                OpeningScene::spawn_dialogue_img
                    .in_schedule(OnEnter(OpeningSceneState::WorldMapImg)),
            )
            .add_system(
                OpeningScene::spawn_dialogue_img
                    .in_schedule(OnEnter(OpeningSceneState::CleanupImg)),
            );
    }
}
