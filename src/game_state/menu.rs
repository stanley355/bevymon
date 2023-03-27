use super::GameState;
use bevy::prelude::*;

#[derive(Debug)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Menu::view);
    }
}

#[derive(Debug, Component)]
pub struct Menu;

impl Menu {
    fn view() {

    }
}