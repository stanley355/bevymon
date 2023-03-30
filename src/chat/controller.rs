use bevy::prelude::*;

use super::components::*;
use super::resource::ChatResource;
use super::state::ChatState;

#[derive(Debug)]
pub struct Chat;

impl Chat {
    pub fn new(
        chat_res: Res<ChatResource>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
        window_query: Query<&Window>,
    ) {
        let window = window_query.single();

        let chatbox_name = Name::new("Chatbox");
        let chatbox = ChatBoxSprite::new(&asset_server, texture_atlas_res, window);

        let chattext_name = Name::new("ChatText");
        let chattext = ChatBoxText::new(
            &asset_server,
            window,
            &chat_res.dialogues[chat_res.dialogue_index],
        );

        commands
            .spawn((chatbox_name, chatbox))
            .insert(ChatBoxSprite);
        commands
            .spawn((chattext_name, chattext))
            .insert(ChatBoxText);
    }

    pub fn new_diagoue(
        chat_res: ResMut<ChatResource>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window_query: Query<&Window>,
    ) {
        let window = window_query.single();

        let chattext_name = Name::new("ChatText");
        let chattext = ChatBoxText::new(
            &asset_server,
            window,
            &chat_res.dialogues[chat_res.dialogue_index],
        );
        commands
            .spawn((chattext_name, chattext))
            .insert(ChatBoxText);
    }

    pub fn next_dialogue(
        keyboard: Res<Input<KeyCode>>,
        mut commands: Commands,
        mut chat_res_mut: ResMut<ChatResource>,
        mut chat_state: ResMut<NextState<ChatState>>,
        text_query: Query<Entity, With<ChatBoxText>>,
        asset_server: Res<AssetServer>,
        window_query: Query<&Window>,
    ) {
        if keyboard.just_pressed(KeyCode::Z) {
            let text_entity = text_query.single();
            commands.entity(text_entity).despawn();
            if chat_res_mut.dialogue_index < (chat_res_mut.dialogues.len() - 1) {
                chat_res_mut.dialogue_index += 1;
                Self::new_diagoue(chat_res_mut, commands, asset_server, window_query)
            } else {
                chat_state.set(ChatState::OffChat);
            }
        }
    }

    pub fn cleanup(
        mut commands: Commands,
        mut chat_res_mut: ResMut<ChatResource>,
        box_query: Query<Entity, With<ChatBoxSprite>>,
    ) {
        chat_res_mut.dialogue_index = 0;
        chat_res_mut.dialogues = vec!["".to_string()];
        let box_entity = box_query.single();
        commands.entity(box_entity).despawn();
    }
}
