use bevy::prelude::*;

use crate::chat::resource::ChatResource;

#[derive(Debug, Component)]
pub struct BackgroundScene;

impl BackgroundScene {
   pub fn  new(
        mut chat_res_mut: ResMut<ChatResource>,

    ) {
        chat_res_mut.dialogues = vec!["At the beginning there was creation".to_string(), "But where does creation comes from?".to_string()]
    }
}