use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use game_state::GameState;

// mod player;
// mod tile;
// mod frame;
// mod game_state;
mod startup;

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
        .add_startup_system(camera_setup)
        .add_plugins(startup::StartupPluginGroup)
        // .add_plugin(game_state::splash::SplashPlugin)
        // .add_plugin(game_state::menu::MenuPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    let bundle =     Camera2dBundle {
        camera_2d: Camera2d {
            // no "background color", we need to see the main camera's output
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    };

    commands.spawn(bundle);
}
