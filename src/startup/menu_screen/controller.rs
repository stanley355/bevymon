use bevy::prelude::*;
use bevy_tweening::Animator;

use crate::startup::state::StartupState;
use super::components::MenuScreenComponent;

#[derive(Debug, Component)]
pub struct MenuScreen;

impl MenuScreen {
    pub fn new(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Window>) {
        let window = query.single();

        let bg_image = MenuScreenComponent::background(&asset_server, window);
        let bg_name = Name::new("menu_screen");

        let text_wrap = MenuScreenComponent::text_wrap();

        let logo_wrap = MenuScreenComponent::logo_wrap();
        let logo = MenuScreenComponent::logo(&asset_server);
        let title = MenuScreenComponent::title(&asset_server);
        let cta_text = MenuScreenComponent::cta_text(&asset_server);

        let cta_animation = Animator::new(MenuScreenComponent::cta_text_animation());

        commands
            .spawn((bg_image, bg_name, MenuScreen))
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

    pub fn enter_game(
        keyboard_input: Res<Input<KeyCode>>,
        mut startup_state: ResMut<NextState<StartupState>>,
    ) {
        if keyboard_input.pressed(KeyCode::Return) {
            startup_state.set(StartupState::InGame);
        }
    }

    pub fn cleanup(
        mut commands: Commands,
        query: Query<Entity, With<MenuScreen>>,
    ) {
        let screen = query.single();
        commands.entity(screen).despawn_descendants();
        commands.entity(screen).despawn();
    }
}