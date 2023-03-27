use bevy::core_pipeline::{clear_color::ClearColorConfig, core_2d::Camera2d};
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
    let camera = Camera {
        viewport: Some(bevy::render::camera::Viewport {
            physical_position: UVec2::new(400, 0),
            physical_size: UVec2::new(256, 256),
            ..default()
        }),
        ..default()
    };

    let camera_bundle = Camera2dBundle {
        camera,
        ..default()
    };

    commands.spawn(camera_bundle);
}
