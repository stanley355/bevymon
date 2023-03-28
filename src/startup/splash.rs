use super::StartupState;
use bevy::prelude::*;

#[derive(Debug)]
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StartupState>()
            .add_startup_system(SplashScreen::start)
            .add_system(SplashScreen::countdown)
            .add_system(SplashScreen::despawn.in_schedule(OnExit(StartupState::Splash)));
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct SplashTimer(Timer);

#[derive(Component, Debug)]
pub struct SplashScreen;

impl SplashScreen {
    fn start(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Window>) {
        let window = query.single();

        let splash_img = asset_server.load("images/splash.png");
        let size = Size::new(Val::Px(window.width()), Val::Px(window.height()));

        let bundle = ImageBundle {
            style: Style {
                size,
                ..default()
            },
            image: UiImage::new(splash_img),
            ..default()
        };

        commands.spawn((bundle, SplashScreen));
        commands.insert_resource(SplashTimer(Timer::from_seconds(2., TimerMode::Once)));
    }

    fn countdown(
        mut game_state: ResMut<NextState<StartupState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        let timer_tick = timer.tick(time.delta());
        if timer_tick.finished() {
            game_state.set(StartupState::Menu);
        }
    }

    pub fn despawn(mut commands: Commands, query: Query<(Entity, &SplashScreen)>) {
        let screen = query.single();
        commands.get_entity(screen.0).unwrap().despawn();
    }
}
