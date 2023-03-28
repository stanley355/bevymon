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
pub struct SplashText;

#[derive(Component, Debug)]
pub struct SplashScreen;

impl SplashScreen {
    fn loading_text(asset_server: Res<AssetServer>, window: &Window) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/poke-solid.ttf");

        let text_style = TextStyle {
            font,
            font_size: 100.,
            color: Color::WHITE,
        };

        let style = Style {
            margin: UiRect {
                top: Val::Px(window.height() * 0.8),
                left: Val::Px(25.),
                ..Default::default()
            },
            ..Default::default()
        };

        TextBundle::from_section("Loading...", text_style).with_style(style)
    }

    fn start(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Window>) {
        let window = query.single();

        let splash_img = asset_server.load("images/splash.png");
        let size = Size::new(Val::Px(window.width()), Val::Px(window.height()));

        let bundle = ImageBundle {
            style: Style { size, ..default() },
            image: UiImage::new(splash_img),
            ..default()
        };

        let text = Self::loading_text(asset_server, window);

        commands.spawn((bundle, SplashScreen)).with_children(|par| {
            par.spawn((text, SplashText));
        });
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

    pub fn despawn(mut commands: Commands, screen_query: Query<(Entity, &SplashScreen)>, text_query: Query<(Entity, &SplashText)>) {
        let screen = screen_query.single();
        let text = text_query.single();
        commands.get_entity(screen.0).unwrap().despawn();
        commands.get_entity(text.0).unwrap().despawn();
    }
}
