use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct ChatResource {
    pub spawn: bool,
    pub dialogue_index: usize,
    pub dialgoues: Vec<String>,
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
