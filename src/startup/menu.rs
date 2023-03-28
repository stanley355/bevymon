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

        let bg_image = MenuComponent::background(&asset_server, window);
        let bg_name = Name::new("menu_screen");

        let logo_wrap = MenuComponent::logo_wrap(window);
        let logo = MenuComponent::logo(&asset_server);
        let title = MenuComponent::title(&asset_server);

        commands.spawn((bg_image, bg_name)).with_children(|parent| {
            parent.spawn(logo_wrap).with_children(|parent| {
                parent.spawn(logo);
                parent.spawn(title);
            });
        });
    }
}
