use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod tile;

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
        // .add_startup_system(camera_setup)
        // .add_plugin(player::PlayerPlugin)
        // .add_startup_system(tile::Tile::setup)
        .run();
}

fn window_setup(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    let left_border = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(0.1 * window.width()), Val::Px(800.)),
            // margin: UiRect::all(Val::Px(100.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        background_color: Color::WHITE.into(),
        ..Default::default()
    };

    commands.spawn(left_border);
}

fn camera_setup(mut commands: Commands, query: Query<&Window>) {

    // let camera = Camera {
    //     viewport: Some(bevy::render::camera::Viewport {
    //         physical_position: UVec2::new(500, 100),
    //         physical_size: UVec2::new(1200, 800),
    //         ..default()
    //     }),
    //     ..default()
    // };

    // let camera_bundle = Camera2dBundle {
    //     camera,
    //     ..default()
    // };

    let border_bundle = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(0.1 * window.width()), Val::Px(800.)),
            // margin: UiRect::all(Val::Px(100.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        background_color: Color::WHITE.into(),
        ..Default::default()
    };

    // commands.spawn(Camera2dBundle::default());
    commands.spawn(border_bundle);
}
