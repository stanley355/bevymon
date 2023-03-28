use bevy::prelude::*;

use super::menu_component::MenuComponent;
use super::StartupState;

#[derive(Debug)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(MenuScreen::view.in_schedule(OnExit(StartupState::Splash)));
    }
}

#[derive(Debug, Component)]
pub struct MenuScreen;

impl MenuScreen {
    fn view(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Window>) {
        let window = query.single();

        let background = MenuComponent::background(&asset_server, window);
        let bg_name = Name::new("menu_bg");

        let batch = vec![(background, bg_name)];

        commands.spawn_batch(batch);
    }
}
