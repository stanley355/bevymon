use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod tile;
mod frame;

fn main() {
    let win_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Pokemon Rust".into(),
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(win_plugin)) // prevent blurry sprite
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(frame::Frame::frame_setup)
        .add_startup_system(camera_setup)
        // .add_plugin(player::PlayerPlugin)
        // .add_startup_system(tile::Tile::setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
