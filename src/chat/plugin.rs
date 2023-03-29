
use super::state::ChatState;
use bevy::prelude::*;

#[derive(Debug)]
pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        // let dummy = vec![
        //     "At the start there's creation...".to_string(),
        //     "But where does the creation comes from?".to_string(),
        // ];
        // let textbox = TextBox::new(true, dummy);

        app.add_state::<ChatState>();

        // app.insert_resource(textbox)
        //     .add_startup_system(TextBox::spawn)
        // .add_system(TextBox::detect_change);
        // TODO: Activate this code once functionality done
        // .add_system(TextBox::spawn.in_schedule(OnEnter(StartupState::InGame)));
    }
}