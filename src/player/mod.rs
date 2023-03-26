use bevy::prelude::*;

mod animation;
mod component;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(component::Player::setup)
            .add_system(component::Player::move_player)
            .add_system(animation::PlayerAnimation::animate_player_movement);
    }
}
