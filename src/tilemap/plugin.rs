use bevy::prelude::*;

use super::controller::TileMap;

#[derive(Debug)]
pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(TileMap::new);
    }
}