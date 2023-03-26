use bevy::prelude::*;
mod component;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(component::Player::setup)
            .add_system(component::Player::player_movement)
            .add_system(component::Player::animate_player_movement);
    }
}
