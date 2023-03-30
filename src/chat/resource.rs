use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct ChatResource {
    pub dialogue_index: usize,
    pub dialogues: Vec<String>,
}

impl ChatResource {
    pub fn new() -> Self {
        ChatResource {
            dialogue_index: 0,
            dialogues: vec!["".to_string()],
        }
    }
}
