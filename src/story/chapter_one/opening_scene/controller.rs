use bevy::prelude::*;

use super::components::OpeningSceneSprite;
use super::state::OpeningSceneState;
use crate::{chat::{resource::ChatResource, state::ChatState}, story::chapter_one::state::ChapterOneState};

#[derive(Debug, Component)]
pub struct OpeningScene;

impl OpeningScene {
    pub fn new(
        mut chat_res_mut: ResMut<ChatResource>,
        mut chat_state: ResMut<NextState<ChatState>>,
    ) {
        chat_res_mut.dialogues = vec![
            "At the beginning there was creation".to_string(),
            "But where does creation comes from?".to_string(),
            "Some said it started from egg,".to_string(),
            "Some said it started from Mew,".to_string(),
            "Some said it started from Arceus.".to_string(),
            "Nobody knows...".to_string(),
            "But that creation has become what we know as the three kingdoms...".to_string(),
            "Hespera, Kalypso, and Anthea.".to_string(),
            "Why do they wage the war? What do they strive for?".to_string(),
            "It's your journey to unfold.".to_string()
        ];
        chat_state.set(ChatState::OnChat);
    }

    pub fn spawn_img_detection(
        keyboard: Res<Input<KeyCode>>,
        chat_res_mut: ResMut<ChatResource>,
        current_scene: Res<State<OpeningSceneState>>,
        mut next_scene: ResMut<NextState<OpeningSceneState>>,
        mut next_chap_one_state: ResMut<NextState<ChapterOneState>>,
    ) {
        match (chat_res_mut.dialogue_index, current_scene.0) {
            (2, OpeningSceneState::NoImg) =>  next_scene.set(OpeningSceneState::EggImg),
            (3, OpeningSceneState::EggImg) => next_scene.set(OpeningSceneState::MewImg),
            (4, OpeningSceneState::MewImg) => next_scene.set(OpeningSceneState::ArceusImg),
            (5, OpeningSceneState::ArceusImg) => next_scene.set(OpeningSceneState::CleanupImg),
            (7, OpeningSceneState::CleanupImg) => next_scene.set(OpeningSceneState::WorldMapImg),
            (8, OpeningSceneState::WorldMapImg) => next_scene.set(OpeningSceneState::CleanupImg),
            _ => ()
        }

        if chat_res_mut.dialogue_index == 9 && keyboard.just_pressed(KeyCode::Z) {
            next_chap_one_state.set(ChapterOneState::Introduction);
        }
    }

    pub fn spawn_dialogue_img(
        current_scene: Res<State<OpeningSceneState>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        sprite_query: Query<Entity, With<OpeningSceneSprite>>,
    ) {
        match current_scene.0 {
            OpeningSceneState::EggImg => {
                let img_bundle = OpeningSceneSprite::new(&asset_server, "pokemon/egg/egg.png");
                commands.spawn(img_bundle).insert(OpeningSceneSprite);
            }
            OpeningSceneState::MewImg => {
                let sprite_entity = sprite_query.single();
                let img_bundle =
                    OpeningSceneSprite::new(&asset_server, "pokemon/mew/mew_shiny.png");
                commands.entity(sprite_entity).despawn();
                commands.spawn(img_bundle).insert(OpeningSceneSprite);
            }
            OpeningSceneState::ArceusImg => {
                let sprite_entity = sprite_query.single();
                let img_bundle =
                    OpeningSceneSprite::new(&asset_server, "pokemon/arceus/arceus_shiny.png");
                commands.entity(sprite_entity).despawn();
                commands.spawn(img_bundle).insert(OpeningSceneSprite);
            }
            OpeningSceneState::WorldMapImg => {
                let img_bundle =
                    OpeningSceneSprite::world_map(&asset_server);
                commands.spawn(img_bundle).insert(OpeningSceneSprite);
            }
            OpeningSceneState::CleanupImg => {
                let sprite_entity = sprite_query.single();
                commands.entity(sprite_entity).despawn();
            }
            _ => (),
        }
    }
}
