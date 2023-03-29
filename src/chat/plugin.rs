use super::resource::ChatResource;
use super::state::ChatState;
use bevy::prelude::*;

#[derive(Debug)]
pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        let chat_resource = ChatResource::new();

        app.add_state::<ChatState>().insert_resource(chat_resource);

        // app.insert_resource(textbox)
        //     .add_startup_system(TextBox::spawn)
        // .add_system(TextBox::detect_change);
        // TODO: Activate this code once functionality done
        // .add_system(TextBox::spawn.in_schedule(OnEnter(StartupState::InGame)));
    }
}
