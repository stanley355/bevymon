use bevy::prelude::*;

use super::GameState;

#[derive(Debug)]
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(splash_setup)
            .add_system(splash_countdown)
            .add_system(splash_setup_2.in_schedule(OnExit(GameState::Splash)));
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Component, Debug)]
struct SplashScreen;

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

    commands.spawn((bundle, SplashScreen));
    commands.insert_resource(SplashTimer(Timer::from_seconds(3., TimerMode::Once)));
}

fn splash_countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    let timer_tick = timer.tick(time.delta());
    if timer_tick.finished() {
        game_state.set(GameState::Menu);
    }
}

fn splash_setup_2(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Window>, query2: Query<(Entity, &SplashScreen)>) {
    let window = query.single();
    let screen = query2.single();

    let splash_img = asset_server.load("tiles.png");
    let bundle = ImageBundle {
        style: Style {
            // This will set the logo to be 200px wide, and auto adjust its height
            size: Size::new(Val::Px(window.width()), Val::Auto),
            ..default()
        },
        image: UiImage::new(splash_img),
        ..default()
    };

    commands.get_entity(screen.0).unwrap().despawn();
    commands.spawn(bundle);
}
