use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(camera_setup)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(tile::Tile::setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    let camera = Camera {
        viewport: Some(bevy::render::camera::Viewport {
            physical_position: UVec2::new(500, 100),
            physical_size: UVec2::new(1200, 800),
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
