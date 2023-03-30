use bevy::render::camera::Viewport;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;

mod chat;
mod startup;
mod story;

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
        .add_plugin(TweeningPlugin)
        .add_plugins(story::plugin::StoryPluginGroup)
        .add_plugin(chat::plugin::ChatPlugin)
        .add_plugins(startup::StartupPluginGroup)
        .run();
}

fn camera_setup(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    let win_width = window.physical_width() as u32;
    let win_height = window.physical_height() as u32;

    let camera = Camera {
        viewport: Some(Viewport {
            physical_size: UVec2::new(win_width, win_height),
            ..default()
        }),
        ..Default::default()
    };

    let bundle = Camera2dBundle {
        camera,
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    };

    commands.spawn(bundle);
}
