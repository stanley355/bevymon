use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SplashBackground;

impl SplashBackground {
    pub fn new(asset_server: &Res<AssetServer>, window: &Window) -> ImageBundle {
        let splash_img = asset_server.load("images/screen/splash.png");
        let size = Size::new(Val::Px(window.width()), Val::Px(window.height()));

        ImageBundle {
            style: Style { size, ..default() },
            image: UiImage::new(splash_img),
            ..default()
        }
    }
}

#[derive(Debug, Component)]
pub struct SplashText;

impl SplashText {
    pub fn new(asset_server: &Res<AssetServer>, window: &Window) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/poke-unown.ttf");

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
}