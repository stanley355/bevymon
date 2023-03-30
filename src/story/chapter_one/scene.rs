use bevy::prelude::*;

use super::state::BgSceneState;
use super::components::BackgroundSceneSprite;
use crate::chat::{resource::ChatResource, state::ChatState};

#[derive(Debug, Component)]
pub struct BackgroundScene;

impl BackgroundScene {
    pub fn new(
        mut chat_res_mut: ResMut<ChatResource>,
        mut chat_state: ResMut<NextState<ChatState>>,
    ) {
        chat_res_mut.dialogues = vec![
            "At the beginning there was creation".to_string(),
            "But where does creation comes from?".to_string(),
            "Some said it started from egg".to_string(),
            "Some said it started from Mew".to_string(),
            "Some said it started from Arceus".to_string(),
            "Nobody knows...".to_string(),
        ];
        chat_state.set(ChatState::OnChat);
    }

    pub fn show_img_detection(
        chat_res_mut: ResMut<ChatResource>,
        old_bg_scene_state: Res<State<BgSceneState>>,
        mut bg_scene_state: ResMut<NextState<BgSceneState>>,
    ) {
        if chat_res_mut.dialogue_index == 2 && old_bg_scene_state.0 != BgSceneState::EggImg {
            bg_scene_state.set(BgSceneState::EggImg);
        }
    }

    pub fn spawn_dialogue_img(
        bg_scene_state: Res<State<BgSceneState>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        if bg_scene_state.0 == BgSceneState::EggImg {
            let img_bundle = BackgroundSceneSprite::poke_image(&asset_server, "pokemon/egg/egg.png");
            commands.spawn(img_bundle).insert(BackgroundSceneSprite);
        }
    }
}
