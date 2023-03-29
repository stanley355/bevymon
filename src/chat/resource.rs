use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct ChatResource {
    spawn: bool,
    dialogue_index: usize,
    dialgoues: Vec<String>,
}

impl ChatResource {
    pub fn new() -> Self {
        ChatResource {
            spawn: false,
            dialogue_index: 0,
            dialgoues: vec!["".to_string()],
        }
    }
}
