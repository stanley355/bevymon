use bevy::prelude::*;
use bevy_tweening::Animator;

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

        let text_wrap = MenuComponent::text_wrap();

        let logo_wrap = MenuComponent::logo_wrap();
        let logo = MenuComponent::logo(&asset_server);
        let title = MenuComponent::title(&asset_server);
        let cta_text = MenuComponent::cta_text(&asset_server);

        let cta_animation = Animator::new(MenuComponent::cta_text_animation());

        commands
            .spawn((bg_image, bg_name))
            .with_children(|bg_parent| {
                bg_parent
                    .spawn(text_wrap)
                    .with_children(|text_wrap_parent| {
                        text_wrap_parent.spawn(logo_wrap).with_children(|parent| {
                            parent.spawn(logo);
                            parent.spawn(title);
                        });

                        text_wrap_parent.spawn((cta_text, cta_animation));
                    });
            });
    }
}
