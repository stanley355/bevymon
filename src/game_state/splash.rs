use bevy::prelude::*;

#[derive(Debug)]
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_startup_system(splash_setup);
        // While in this state, run the `countdown` system
        // .add_systems(Update, countdown.in_set(OnUpdate(GameState::Splash)))
        // When exiting the state, despawn everything that was spawned for this screen
        // .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Window>) {
    let window = query.single();

    let splash_img = asset_server.load("splash.png");
    let bundle = ImageBundle {
        style: Style {
            // This will set the logo to be 200px wide, and auto adjust its height
            size: Size::new(Val::Px(window.width()), Val::Auto),
            ..default()
        },
        image: UiImage::new(splash_img),
        ..default()
    };

    commands.spawn(bundle);
}
