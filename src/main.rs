use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(camera_setup)
        .add_startup_system(tile::Tile::setup)
        .add_plugin(player::PlayerPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
