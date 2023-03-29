use bevy::prelude::*;

use super::components::{SplashBackground, SplashText};
use crate::startup::state::StartupState;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct SplashTimer(Timer);

#[derive(Component, Debug)]
pub struct SplashScreen;

impl SplashScreen {
    pub fn new(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Window>) {
        let window = query.single();

        let splash_bg = SplashBackground::new(&asset_server, window);
        let splash_text = SplashText::new(&asset_server, window);

        commands
            .spawn((splash_bg, SplashBackground))
            .with_children(|par| {
                par.spawn((splash_text, SplashText));
            });
        commands.insert_resource(SplashTimer(Timer::from_seconds(2., TimerMode::Once)));
    }

    pub fn countdown(
        mut game_state: ResMut<NextState<StartupState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        let timer_tick = timer.tick(time.delta());
        if timer_tick.finished() {
            game_state.set(StartupState::Menu);
        }
    }

    pub fn despawn(mut commands: Commands, splash_query: Query<Entity, With<SplashBackground>>) {
        let splash_screen = splash_query.single();
        commands.entity(splash_screen).despawn_descendants();
        commands.entity(splash_screen).despawn();
    }
}
