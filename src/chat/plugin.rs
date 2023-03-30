use bevy::prelude::*;

use super::controller::Chat;
use super::resource::ChatResource;
use super::state::ChatState;

#[derive(Debug)]
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        let chat_resource = ChatResource::new();

        app.add_state::<ChatState>()
            .insert_resource(chat_resource)
            .add_system(Chat::new.in_schedule(OnEnter(ChatState::OnChat)))
            .add_system(Chat::next_dialogue.in_set(OnUpdate(ChatState::OnChat)))
            .add_system(Chat::cleanup.in_schedule(OnEnter(ChatState::OnChat)));
    }
}
