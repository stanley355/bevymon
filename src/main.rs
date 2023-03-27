use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod tile;

fn main() {
    let image_plugin = ImagePlugin::default_nearest();

    App::new()
        .add_plugins(DefaultPlugins.set(image_plugin)) // prevent blurry sprite
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(camera_setup)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(tile::Tile::setup)
        .run();
}

fn camera_setup(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    println!("{:?}", window.width());

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

    let border_bundle = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(1200.), Val::Px(800.)),
            margin: UiRect::all(Val::Px(100.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        background_color: Color::WHITE.into(),
        ..Default::default()
    };

    commands.spawn(camera_bundle);
    commands.spawn(border_bundle);
}
