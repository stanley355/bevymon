use bevy::prelude::*;

use super::components::OpeningSceneSprite;
use super::state::OpeningSceneState;
use crate::chat::{resource::ChatResource, state::ChatState};

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
        chat_res_mut: ResMut<ChatResource>,
        current_scene: Res<State<OpeningSceneState>>,
        mut next_scene: ResMut<NextState<OpeningSceneState>>,
    ) {
        if chat_res_mut.dialogue_index == 2 && current_scene.0 == OpeningSceneState::NoImg {
            next_scene.set(OpeningSceneState::EggImg);
        }
        if chat_res_mut.dialogue_index == 3 && current_scene.0 == OpeningSceneState::EggImg {
            next_scene.set(OpeningSceneState::MewImg);
        }
        if chat_res_mut.dialogue_index == 4 && current_scene.0 == OpeningSceneState::MewImg {
            next_scene.set(OpeningSceneState::ArceusImg);
        }
        if chat_res_mut.dialogue_index == 5 && current_scene.0 == OpeningSceneState::ArceusImg {
            next_scene.set(OpeningSceneState::CleanupImg);
        }
        if chat_res_mut.dialogue_index == 7 && current_scene.0 == OpeningSceneState::CleanupImg{
            next_scene.set(OpeningSceneState::WorldMapImg);
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
            OpeningSceneState::CleanupImg => {
                let sprite_entity = sprite_query.single();
                commands.entity(sprite_entity).despawn();
            }
            _ => (),
        }
    }
}
