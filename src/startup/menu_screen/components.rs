
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

#[derive(Debug, Component)]
pub struct MenuComponent;

impl MenuComponent {
    pub fn background(asset_server: &Res<AssetServer>, window: &Window) -> ImageBundle {
        let background = asset_server.load("images/war_bg.png");
        let size = Size::new(Val::Px(window.width()), Val::Px(window.height()));

        let bundle = ImageBundle {
            style: Style { size, ..default() },
            image: UiImage::new(background),
            ..default()
        };

        return bundle;
    }

    pub fn text_wrap() -> NodeBundle {
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::left(Val::Percent(20.)),
                ..default()
            },
            ..default()
        }
    }

    pub fn logo_wrap() -> NodeBundle {
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    }

    pub fn logo(asset_server: &Res<AssetServer>) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/poke-solid.ttf");
        TextBundle::from_section(
            "Pokemon",
            TextStyle {
                font: font.clone(),
                font_size: 175.0,
                color: Color::YELLOW,
            },
        )
    }

    pub fn title(asset_server: &Res<AssetServer>) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/poke-unown.ttf");
        TextBundle::from_section(
            "War of Origin",
            TextStyle {
                font: font.clone(),
                font_size: 80.0,
                color: Color::WHITE,
            },
        )
    }

    pub fn cta_text_animation() -> Tween<Text> {
        Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            TextColorLens {
                start: Color::NONE,
                end: Color::WHITE,
                section: 0,
            },
        ).with_repeat_count(RepeatCount::Infinite)
    }

    pub fn cta_text(asset_server: &Res<AssetServer>) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/poke-text.ttf");
        TextBundle::from_section(
            "Press Enter to join War",
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
    }
}
