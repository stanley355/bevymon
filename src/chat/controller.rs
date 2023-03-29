use bevy::prelude::*;

use super::component::*;
use super::resource::ChatResource;

#[derive(Debug)]
pub struct Chat;

impl Chat {
    fn new(
        chat_res: Res<ChatResource>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
        window_query: Query<&Window>,
    ) {
        if chat_res.spawn {
            let window = window_query.single();

            let chatbox_name= Name::new("Chatbox");
            let chatbox = ChatBoxSprite::new(&asset_server, texture_atlas_res, window);

            let chattext_name = Name::new("ChatText");
            let chattext = ChatBoxText::new(&asset_server, window, &chat_res.dialgoues[chat_res.dialogue_index]);

            commands.spawn((chatbox_name, chatbox)).insert(ChatBoxSprite);
            commands.spawn((chattext_name, chattext)).insert(ChatBoxText);
        }
    }
}