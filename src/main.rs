use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(camera_setup)
        .add_startup_system(player::setup_player)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}