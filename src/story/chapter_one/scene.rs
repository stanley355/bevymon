use bevy::prelude::*;

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
}
